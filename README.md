# QR Code Generator

Simple Rust program that generates a QR code from a user-provided URL.

## Prerequisites

Before running the application, ensure you have Rust installed on your system:
you can install Rust by following the instructions on the official Rust website: [Install Rust](https://www.rust-lang.org/tools/install).

Need also GTK3 for runnung Druid (for UI)
**Install GTK+3 (Linux)**: On Linux, Druid requires GTK+3. Install it with the following commands:
    - **Ubuntu**:
        ```bash
        sudo apt-get install libgtk-3-dev
        ```
    - **Fedora**:
        ```bash
        sudo dnf install gtk3-devel glib2-devel
        ```

## How to Run


1. **Clone the Repository**: clone this repository to your local machine using Git.

    ```bash
    git clone https://github.com/AFx3/qrcode-generator.git
    ```

2. **Navigate to the Project Directory**: change your current directory to the project directory.

    ```bash
    cd qrcode-rust
    ```

3. **Compile and Run the Application**: use Cargo, the Rust package manager, to compile and run the application.

    ```bash
    cargo run
    ```

4. **Enter URL**: when prompted, enter the URL you want to encode into the QR code.

5. **QR Code Generation**: the application will generate a QR code based on the provided URL and save it as `QR_code.png` in the `qrcode-generator/qrcode-rust` directory of the project.

6. **View QR Code**: Navigate to the `qrcode-generator/qrcode-rust` directory to view the generated QR code image (png format).
