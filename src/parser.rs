use regex::Regex;

#[derive(Debug)]
pub struct ParsedCode {
    pub defines: Vec<String>,
    pub funciones: Vec<Funcion>,
    pub eventos: Vec<Evento>,
}

#[derive(Debug)]
pub struct Funcion {
    pub nombre: String,
    pub parametros: Vec<String>,
    pub cuerpo: String,
}

#[derive(Debug)]
pub struct Evento {
    pub nombre: String,
    pub cuerpo: String,
}

pub fn parse_code(code: &str) -> ParsedCode {
    let mut defines = Vec::new();
    let mut funciones = Vec::new();
    let mut eventos = Vec::new();

    // Escapa las comillas correctamente en la expresión regular
    let define_re = Regex::new(r#"(define|definir)\s+\w+\s+.+"#).unwrap();
    // let funcion_re = Regex::new(r"inicio\s+(\w+)\s*(.*?)\s*final").unwrap();
    // Teniendo en cuaenta que el codigo es algo como:
    /*
    # Función para enviar mensajes programados
inicio enviarMensajeProgramado
    parametro canalID
    parametro mensaje
    esperar 5m
    enviar canalID mensaje
final

*/
    let funcion_re = Regex::new(r"inicio\s+(\w+)\s*(?s)(.*?)\s*final").unwrap();
    let evento_re = Regex::new(r"evento\s+(\w+)\s*(.*?)\s*final").unwrap();

    for define in define_re.captures_iter(code) {
        println!("{:?}", define);
        defines.push(define[0].to_string());
    }

    for funcion in funcion_re.captures_iter(code) {
        let nombre = funcion[1].to_string();
        let cuerpo = funcion[2].to_string();
        let mut parametros = Vec::new();
        /*
        # Función para saludar al usuario
inicio saludar
    parametro usuario
    devolver "Hola, " + usuario + "!"
final
*/
        for parametro in funcion[2].lines() {
            if parametro.starts_with("parametro") {
                parametros.push(parametro[10..].to_string());
            }
        }
        println!("{:?}", parametros);
        funciones.push(Funcion { nombre, parametros, cuerpo });
    }

    for evento in evento_re.captures_iter(code) {
        let nombre = evento[1].to_string();
        let cuerpo = evento[2].to_string();
        eventos.push(Evento { nombre, cuerpo });
    }

    ParsedCode { defines, funciones, eventos }
}
