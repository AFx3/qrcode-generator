
/*
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
*/

// import widgets and utils from druid
use druid::widget::{Button, Flex, TextBox};
use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};
// import QrCodeEcc from qrcode_generator
use qrcode_generator::QrCodeEcc;

// define a struct AppState to hold the application state
#[derive(Clone, Data, Lens)]
struct AppState {
    url: String,        // string to store the URL
    status: String,     // string to store the status message of QR code generation
}

fn main() {

    // create a window description (WindowDesc) for the main window of the app
    let main_window = WindowDesc::new(build_ui())
        // set the title of the window
        .title("QR Code Generator")
        // set ititial window size
        .window_size((350.0, 120.0));

    // define the initial state of the application
    let initial_state = AppState {

        url: String::new(),
        status: String::new(),
    };

    // Launch the app thu AppLauncher with the main window and initial state
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application"); // handle any errors that occur during the launch
}


// define a function to build the UI of the app
fn build_ui() -> impl Widget<AppState> {

    // text box to enter the URL from which the QR code will be generated
    let url_input = TextBox::new()
        .with_placeholder("Enter URL here") // set placeholder text
        .fix_width(500.0) // set width of the text box
        .lens(AppState::url); // lens to bind the text box to the url field of AppState

    // button to generate QR code
    let generate_button = Button::new("Generate")

        // define an event handler when the button is clicked
        .on_click(|_ctx, data: &mut AppState, _env| {

            // Generate the QR code and handle any errors
            if let Err(err) = qrcode_generator::to_png_to_file(
                &data.url,
                QrCodeEcc::Low,
                1024,
                "QR_code.png",
            ) {
                // update the status message with the error
                data.status = format!("Error: {}", err);
            } else {

                // update the status message when QR code is generated successfully
                data.status = "Success: QR creted in project folder!".to_string();
            }
        })
        .fix_width(100.0); // fix the width of the button

    // display status message
    let status_message = TextBox::new()
    
        .with_text_size(16.0)    // set the text size of the status message
        .expand_width()         // expand the width of the status message to fill available space 
        .lens(AppState::status);                // use the AppState's status field for data binding

    // Flex layout to arrange widgets vertically
    Flex::column()
        .with_child(url_input)    // Add the URL input text box to the layout
        .with_child(generate_button)// Add the generate button to the layout
        .with_child(status_message)// // Add the status message text box to the layout
        .padding(20.0)  // Add padding around the layout
}



