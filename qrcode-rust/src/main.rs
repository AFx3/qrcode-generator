
use qrcode_generator::QrCodeEcc;
use std::io;

fn main() {

    // wait for user input
    println!("Enter the URL to encode into QR code:");

    // store the user input
    let mut input_string = String::new();

    // check if the user input is valid
    io::stdin().read_line(&mut input_string).expect("Failed to read line");

    let url = input_string.trim(); // remove trailing newline

    if let Err(err) = qrcode_generator::to_png_to_file(url, QrCodeEcc::Low, 1024, "QR_code_generated.png") {
        eprintln!("Error: {}", err);
    } else {
        println!("QR code generated and saved successfully in qrcode-rust/QR_code_generated.png");
    }
}



