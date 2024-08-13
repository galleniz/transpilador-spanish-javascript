use crate::parser::{ParsedCode, Funcion, Evento};

pub fn transpile(parsed: ParsedCode) -> String {
    let mut js_code = String::new();

    // Convertir las definiciones
    for define in parsed.defines {
        let trimmed_ldefine = define.trim(); // Aplica trim para eliminar espacios innecesarios

        let parts: Vec<&str> = trimmed_ldefine.split_whitespace().collect();
        if parts.len() >= 3 {
            let variable_name = parts[1];
            let variable_value = trimmed_ldefine.splitn(3, ' ').last().unwrap().replace("\"", "");
            js_code.push_str(&format!("var {} = \"{}\";\n", variable_name, variable_value));
        }
    }

    // Add headers
    js_code.push_str("var headers = {};\n");

    // Convertir las funciones
    for funcion in parsed.funciones {
        js_code.push_str(&transpile_funcion(&funcion));
    }

    // Convertir los eventos
    for evento in parsed.eventos {
        js_code.push_str(&transpile_evento(&evento));
    }

    js_code
}

fn transpile_funcion(funcion: &Funcion) -> String {
    let mut js_funcion = String::new();
    // js_funcion.push_str(&format!("async function {}() {{\n", funcion.nombre));
    js_funcion.push_str(&format!("async function {}({}) {{\n", funcion.nombre, funcion.parametros.join(", ")));

    for linea in funcion.cuerpo.lines() {
        let trimmed_line = linea.trim(); // Aplica trim para eliminar espacios innecesarios
        println!("{}", trimmed_line);

        if trimmed_line.contains("resultadoJson") {
            /*
            devolver resultadoJson.imagen
            */
            // First insert the JSON object
            js_funcion.push_str(&format!("let resultadoJson = await response.json();\n"));
            js_funcion.push_str(&transpile_http_request(trimmed_line));
        } else if trimmed_line.starts_with("header") || trimmed_line.starts_with("get") || trimmed_line.starts_with("devolver") || trimmed_line.starts_with("endHeaders") {
            js_funcion.push_str(&transpile_http_request(trimmed_line));
        } else if trimmed_line.starts_with("intentar") {
            println!("Intentar");
            js_funcion.push_str("try {\n");
        } else if trimmed_line.starts_with("responder") {
            js_funcion.push_str(&transpile_responder(trimmed_line));
        } else if trimmed_line.starts_with("imagen") {
            js_funcion.push_str(&transpile_imagen(trimmed_line));
        } else if trimmed_line.starts_with("sino si") {
            js_funcion.push_str(&format!("}} else if ({}) {{\n", &trimmed_line[7..]));
        } else if trimmed_line.starts_with("sino") {
            js_funcion.push_str("} else {\n");
        } else if trimmed_line.starts_with("si falla") {
            js_funcion.push_str("} catch (error) {\n");
        } else if trimmed_line.starts_with("si") {
            js_funcion.push_str(&format!("if ({}) {{\n", &trimmed_line[3..]));
        } else if trimmed_line.starts_with("fin") {
            js_funcion.push_str("}\n");
        } else if trimmed_line.starts_with("parametro") {
            // No hacer nada
        } else if trimmed_line.starts_with("esperar") {
            let mut tiempo = &trimmed_line[7..];
            // Convert &str to number, if isNAN then use 0
            tiempo = match tiempo.parse::<u32>() {
                Ok(_) => tiempo,
                Err(_) => "0"
            };
            js_funcion.push_str(&format!("await new Promise(resolve => setTimeout(resolve, {} * 1_000));\n", tiempo));
        } else if trimmed_line.starts_with("var") {
            let parts: Vec<&str> = trimmed_line.split_whitespace().collect();
            if parts.len() >= 3 {
                let variable_name = parts[1];
                let variable_value = trimmed_line.splitn(3, ' ').last().unwrap().replace("\"", "");
                js_funcion.push_str(&format!("var {} = {};\n", variable_name, variable_value));
            }
        } 
        else {
            js_funcion.push_str(&format!("    {}\n", trimmed_line));
        }
    }

    js_funcion.push_str("}\n\n");
    js_funcion
}


fn transpile_evento(evento: &Evento) -> String {
    let mut js_evento = String::default();
    js_evento.push_str(&format!("bot.on('{}', async (msg) => {{\n", evento.nombre));
    for linea in evento.cuerpo.lines() {
        if linea.contains("responder") {
            js_evento.push_str(&transpile_responder(linea));
        } else {
            js_evento.push_str(&format!("    {}\n", linea));
        }
    }
    js_evento.push_str("});\n\n");
    js_evento
}

fn transpile_http_request(linea: &str) -> String {
    if linea.starts_with("header") {
        let args = linea.split_whitespace().collect::<Vec<&str>>();
        // Example for args
        // ["header", "Content-Type:", ""application/json""]
        // If dont have " then is a variable
        // In this case the header need to be like this
        // `Content-Type: ${contentType}`
        if args.len() == 3 {
            format!("headers[{}] = '{}';\n", args[1], args[2])
        } else {
            format!("headers[{}] = '';\n", args[1])
        }
    }
    else if linea.starts_with("get") {
        let args = linea.split_whitespace().collect::<Vec<&str>>();
        // Example for args
        // ["get", ""api.openweathermap.org/data/2.5/weather?q=". "ciudad", ""&appid=Your_API_Key""]
        // If dont have " then is a variable
        // In this case the url need to be like this
        // `http://api.openweathermap.org/data/2.5/weather?q=${ciudad}&appid=Your_API_Key`
        let mut url = String::default();
        
        url.push('`');
        url.push_str("http://");
        for arg in args.iter().skip(1) {
            if arg.starts_with('"') {
                url.push_str(&arg[1..arg.len()-1]);
            } else {
                url.push_str(&format!("${{ {} }}", arg));
            }
        }
        url.push('`');
        format!(
            "const response = await fetch({}, {{ headers }});\nlet codigoHTTP = response.status;\n", url.trim()
        )
    } else if linea.starts_with("devolver") {
        if linea[9..].trim() == "nada" {
            format!("return;\n")
        } else if linea.len() > 9 {
            let return_value = &linea[9..];
            format!("return {};\n", return_value.trim())
        } else {
            format!("return;\n")
        }
} else {
        String::default()
    }
}

fn transpile_responder(linea: &str) -> String {
    let message = &linea[10..].trim();
    format!("msg.channel.send({});\n", message)
}

fn transpile_imagen(linea: &str) -> String {
    let image_url = &linea[7..].trim();
    format!("msg.channel.send({{ files: ['{}'] }});\n", image_url)
}
