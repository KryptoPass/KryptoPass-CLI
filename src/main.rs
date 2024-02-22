use std::io::{self, Write};
use unicode_width::UnicodeWidthStr;

fn main() {
    let mut input = String::new();

    print!("Enter your password: ");
    io::stdout().flush().expect("Failed to flush stdout");

    // Aquí se oculta la entrada de contraseña
    input = input_with_hidden_input().expect("Failed to read password");

    println!("\nYour password is: {}", input);
}

fn input_with_hidden_input() -> Result<String, io::Error> {
    let rb = rustbox::RustBox::init(Default::default())?;

    let mut input = String::new();
    let mut width = 0;

    loop {
        if let Ok(rustbox::Event::KeyEvent(key)) = rb.poll_event(false) {
            match key {
                rustbox::Key::Backspace if !input.is_empty() => {
                    input.pop();
                    width -= 1;
                }
                rustbox::Key::Enter => break,
                _ => {
                    if let Some(key) = key.to_char() {
                        input.push(key);
                        width += UnicodeWidthStr::width(key.to_string().as_str());
                    }
                }
            }
        }

        // Imprimir asteriscos para cada carácter de la contraseña
        let password_mask: String = std::iter::repeat('*').take(input.chars().count()).collect();
        rb.print(
            1,
            1,
            rustbox::RB_NORMAL,
            rustbox::Color::White,
            rustbox::Color::Black,
            &password_mask,
        );

        // Limpiar la pantalla si la longitud de la entrada cambia
        if width != input.chars().count() {
            rb.clear();
            width = input.chars().count();
        }

        rb.present();
    }

    Ok(input)
}

// use std::io::{self, Write};

// fn main() {
//     let mut input = String::new();

//     print!("Enter your password: ");
//     io::stdout().flush().expect("Failed to flush stdout");

//     io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read line");

//     // Eliminar el carácter de nueva línea del final
//     input = input.trim_end().to_string();

//     // Ahora puedes realizar tu propia validación de contraseña aquí
//     // Por ejemplo, verificar la longitud mínima, la complejidad, etc.

//     println!("Your password is: {}", input);
// }

// use std::ffi::c_void;
// use windows::Win32::Security::Cryptography::*;

// fn protect_memory(data: String) -> Option<Vec<u8>> {
//     let data_bytes = data.clone().into_bytes();
//     let padded_data = pad_data(data_bytes, CRYPTPROTECTMEMORY_BLOCK_SIZE as usize);
//     let data_bytes = padded_data.as_slice();

//     let mut buffer: Vec<u8> = vec![0; data_bytes.len()];
//     buffer.copy_from_slice(data_bytes);

//     let buffer_len = buffer.len() as u32;

//     let result = unsafe {
//         CryptProtectMemory(
//             buffer.as_mut_ptr() as *mut c_void,
//             buffer_len,
//             CRYPTPROTECTMEMORY_SAME_PROCESS,
//         )
//     };

//     match result {
//         Ok(()) => {
//             println!("Memoria cifrada correctamente");
//             Some(buffer) // Devuelve los datos cifrados
//         }
//         Err(e) => {
//             println!("Error al cifrar la memoria: {}", e);
//             None // Devuelve un valor nulo en caso de error
//         }
//     }
// }

// fn unprotect_memory(ciphertext: Vec<u8>) -> Option<Vec<u8>> {
//     let ciphertext_len = ciphertext.len() as u32;

//     let mut plaintext = ciphertext.clone(); // Hacemos una copia para usar como buffer para los datos desencriptados

//     let result = unsafe {
//         CryptUnprotectMemory(
//             plaintext.as_mut_ptr() as *mut c_void,
//             ciphertext_len,
//             CRYPTPROTECTMEMORY_SAME_PROCESS,
//         )
//     };

//     match result {
//         Ok(()) => {
//             println!("Memoria descifrada correctamente");
//             Some(plaintext) // Devuelve los datos descifrados
//         }
//         Err(e) => {
//             println!("Error al descifrar la memoria: {}", e);
//             None // Devuelve un valor nulo en caso de error
//         }
//     }
// }

// fn main() {
//     println!("Ingrese un texto: ");
//     let mut password = String::new();

//     let _ = std::io::stdin().read_line(&mut password);

//     let ciphertext = match protect_memory(password.clone()) {
//         Some(ciphertext) => ciphertext,
//         None => {
//             println!("No se pudieron cifrar los datos.");
//             return;
//         }
//     };

//     print_decoded(&ciphertext);

//     let plaintext = match unprotect_memory(ciphertext) {
//         Some(plaintext) => unpad_data(plaintext),
//         None => {
//             println!("No se pudieron descifrar los datos.");
//             return;
//         }
//     };

//     print_decoded(&plaintext);
// }

// // Función para aplicar relleno de bytes nulos
// fn pad_data(mut data: Vec<u8>, block_size: usize) -> Vec<u8> {
//     let current_len = data.len();
//     let remaining = block_size - (current_len % block_size);

//     for _ in 0..remaining {
//         data.push(0);
//     }

//     data
// }

// fn unpad_data(mut data: Vec<u8>) -> Vec<u8> {
//     // Encontrar el índice del último byte no nulo
//     let last_non_zero_index = data.iter().rposition(|&x| x != 0);

//     // Truncar el vector de datos para eliminar los bytes nulos de relleno
//     if let Some(index) = last_non_zero_index {
//         data.truncate(index + 1);
//     }

//     data
// }

// fn print_decoded(data: &[u8]) {
//     match std::str::from_utf8(data) {
//         Ok(text) => println!("Texto: {}", text),
//         Err(_) => {
//             println!("Datos como array: {:?}", data);
//         }
//     }
// }
