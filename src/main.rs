// uart_blocking_read.rs - Blocks while waiting for incoming serial data.

use std::error::Error;
use std::time::Duration;

use rppal::uart::{Parity, Uart};
const ASCII_SEPARATOR_CHARACTER: u8 = 28;
//  Command Types
const ASCII_COMMAND_REPORT: u8 = 57;
const ASCII_X_REPORT: u8 = 88;
const ASCII_X_REPORT_ACTION_PRINT: u8 = 84;
//  Status
const ASCII_COMMAND_STATUS: u8 = 56;
//  Status Type
const ASCII_STATUS_TYPE_N: u8 = 78;
//  Z
const ASCII_Z_REPORT: u8 = 90;
// Invoice
const ASCII_COMMAND_INVOICE: u8 = 64;
const ASCII_COMMAND_CREDIT_MEMO: u8 = 68;
const ASCII_FISCALTEXT: u8 = 65;
const ASCII_FISCALROW: u8 = 66;
const ASCII_SUBTOTAL: u8 = 67;
const ASCII_TOTAL: u8 = 69;
const ASCII_NULL_DATA: u8 = 127;
const ASCII_T: u8 = 84;
const ASCII_A: u8 = 65;
const ASCII_B: u8 = 66;
const ASCII_U: u8 = 85;
const ASCII_M: u8 = 77;
const ASCII_m: u8 = 109;
fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the primary UART and configure it for 115.2 kbit/s, no
    // parity bit, 8 data bits and 1 stop bit.
    let mut uart = Uart::new(9600, Parity::None, 8, 1)?;
    let mut _command = vec![];
            _command.push(ASCII_COMMAND_REPORT);
            _command.push(ASCII_SEPARATOR_CHARACTER);
            _command.push(ASCII_Z_REPORT);
            _command.push(ASCII_SEPARATOR_CHARACTER);
            _command.push(ASCII_X_REPORT_ACTION_PRINT);
    uart.write(&_command);
    // Configure read() to block until at least 1 byte is received.
    uart.set_read_mode(1, Duration::default())?;

    let mut buffer = [0u8; 1];
    loop {
        // Fill the buffer variable with any incoming data.
        if uart.read(&mut buffer)? > 0 {
            println!("Received byte: {}", buffer[0]);
        }
    }
}