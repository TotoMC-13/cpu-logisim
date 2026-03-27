// Los # Seran para comentarios
// Debo eliminar espacios en blanco y lineas vacias
// La , sera simplemente un espacio mas, la trato de igual forma

pub fn lexer(input: String) -> Vec<String> {
    // 1. Limpiar comentarios y espacios
    // 2. Tratar las comas como espacios
    // 3. Devolver el vector de tokens
    input
        .lines() // Procesamos linea por linea
        .flat_map(|line| {
            line.split('#') // Cortamos en el primer # para ignorar comentarios
                .next() // Nos quedamos con la parte de la izquierda
                .unwrap_or("")
                .replace(',', " ") // Cambiamos comas por espacios
                .split_whitespace() // Separamos por cualquier espacio en blanco
                .map(String::from)
                .collect::<Vec<String>>()
        })
        .collect()
}
