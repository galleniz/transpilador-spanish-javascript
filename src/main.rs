mod parser;
mod transpiler;

use std::fs;

fn main() {
    // Leer el archivo de entrada
    let input_code = fs::read_to_string("input.botcode").expect("Error al leer el archivo.");

    // Trim all the lines
    let input_code = input_code.lines().map(|line| line.trim()).collect::<Vec<&str>>().join("\n");

    // Parsear el código
    let parsed_code = parser::parse_code(&input_code);

    // Transpilar a JavaScript
    let js_code = transpiler::transpile(parsed_code);

    // Guardar el código transpilado a un archivo
    fs::write("output.js", js_code).expect("Error al escribir el archivo.");
}
