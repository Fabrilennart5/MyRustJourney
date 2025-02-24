use std::{u128, usize};

fn main() {
    // Sección 1: Números sin signo (unsigned)
    let my_number1: u8 = 135; // Rango: 0 a 255
    let my_number2: u16 = 32700; // Rango: 0 a 65535
    let my_number3: u32 = 2250355332; // Rango: 0 a 4294967295
    let my_number4: u64 = 9223372036854775807; // Rango: 0 a 18446744073709551615
    let my_number5: u128 = 340282366920938463463374607431768211455; // Rango: 0 a 340282366920938463463374607431768211455
    let my_number6: usize = 5; // Depende de la arquitectura del sistema

    println!("=== Números sin signo (unsigned) ===");
    println!(
        "u8: {}, u16: {}, u32: {}, u64: {}, u128: {}, usize: {}",
        my_number1, my_number2, my_number3, my_number4, my_number5, my_number6
    );
    println!("{}", "-".repeat(50)); // Separador visual

    // Sección 2: Números con signo (signed)
    let my_number7: i8 = -128; // Rango: -128 a 127
    let my_number8: i16 = -32768; // Rango: -32768 a 32767
    let my_number9: i32 = -2147483648; // Rango: -2147483648 a 2147483647
    let my_number10: i64 = -9223372036854775808; // Rango: -9223372036854775808 a 9223372036854775807
    let my_number11: i128 = -170141183460469231731687303715884105728; // Rango: -170141183460469231731687303715884105728 a 170141183460469231731687303715884105727
    let my_number12: isize = -5; // Depende de la arquitectura del sistema

    println!("=== Números con signo (signed) ===");
    println!(
        "i8: {}, i16: {}, i32: {}, i64: {}, i128: {}, isize: {}",
        my_number7, my_number8, my_number9, my_number10, my_number11, my_number12
    );
    println!("{}", "-".repeat(50)); // Separador visual

    // Sección 3: Números de punto flotante (floats)
    let my_float1: f32 = 3.14; // Precisión simple (32 bits)
    let my_float2: f64 = 2.71828; // Precisión doble (64 bits)

    println!("=== Números de punto flotante (floats) ===");
    println!("f32: {}, f64: {}", my_float1, my_float2);
    println!("{}", "-".repeat(50)); // Separador visual

    // Sección 4: Booleanos
    let my_boolean1: bool = true;
    let my_boolean2: bool = false;

    println!("=== Booleanos ===");
    println!("bool: {}, bool: {}", my_boolean1, my_boolean2);
    println!("{}", "-".repeat(50)); // Separador visual

    // Sección 5: Caracteres (char)
    let my_char1: char = 'a';
    let my_char2: char = '🦀'; // Rust soporta caracteres Unicode

    println!("=== Caracteres (char) ===");
    println!("char: {}, char: {}", my_char1, my_char2);
    println!("{}", "-".repeat(50)); // Separador visual

    // Sección 6: Cadenas de texto (Strings)
    let my_string1: &str = "Hola, Rust!"; // Cadena de texto estática
    let my_string2: String = String::from("¡Aprendiendo Rust!"); // Cadena de texto mutable

    println!("=== Cadenas de texto (Strings) ===");
    println!("&str: {}, String: {}", my_string1, my_string2);
    println!("{}", "-".repeat(50)); // Separador visual

    // Sección 7: Tuplas
    let my_tuple: (u8, char, bool) = (1, 'f', true); // Tupla con elementos de diferentes tipos

    println!("=== Tuplas ===");
    println!("Tupla: ({}, {}, {})", my_tuple.0, my_tuple.1, my_tuple.2);
    println!("{}", "-".repeat(50)); // Separador visual

    // Sección 8: Arreglos
    let my_array: [i32; 3] = [1, 2, 3]; // Arreglo de 3 elementos de tipo i32

    println!("=== Arreglos ===");
    println!(
        "Arreglo: [{}, {}, {}]",
        my_array[0], my_array[1], my_array[2]
    );
    println!("{}", "-".repeat(50)); // Separador visual
}
