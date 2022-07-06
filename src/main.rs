extern crate clap;
extern crate serialport;

use std::io::{self, Write};
use std::time::Duration;

use clap::{App, AppSettings, Arg, SubCommand};

/**
 * entrypoint for app
 */
fn main() {
    let matches = App::new("Recieve input from a serial device")
        .about("Reads data from a serial port and echoes it to stdout")
        .setting(AppSettings::DisableVersion)
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .subcommand(
            SubCommand::with_name("watch")
                .about("Watch a specific device")
                .arg(
                    Arg::with_name("port")
                        .help("The device path to a serial port")
                        .use_delimiter(false)
                        .required(true),
                )
                .arg(
                    Arg::with_name("baud")
                        .help("The baud rate to connect at")
                        .use_delimiter(false)
                        .required(false)
                        .validator(valid_baud)
                        .default_value("115200"),
                ),
        )
        .subcommand(SubCommand::with_name("list").about("Lists available devices"))
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("watch") {
        let port_name = matches.value_of("port").unwrap();
        let baud_rate = matches.value_of("baud").unwrap().parse::<u32>().unwrap();
        let port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(100))
            .open();
        match port {
            Ok(mut port) => {
                let mut serial_buf: Vec<u8> = vec![0; 1000];
                println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
                loop {
                    match port.read(serial_buf.as_mut_slice()) {
                        Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
                        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                        Err(e) => {
                            eprintln!("{:?}", e);
                            ::std::process::exit(1);
                        },
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
                ::std::process::exit(1);
            }
        }
    } else if let Some(_sub_matches) = matches.subcommand_matches("list") {
        list_ports();
    } else{
        println!("Enter a subcommand");
        ::std::process::exit(0);
    }
}

/**
 * validate that the baudrate is an unsigned 32 bit number
 */
fn valid_baud(val: String) -> Result<(), String> {
    val.parse::<u32>()
        .map(|_| ())
        .map_err(|_| format!("Invalid baud rate '{}' specified", val))
}

/**
 * list all the serial ports connected to the pc (numbers and types)
 */
fn list_ports() {
    let ports = serialport::available_ports().unwrap();
    if ports.len() > 0 {
        println!("Ports found: {}", ports.len());
    } else {
        println!("No serial ports found");
    }
    for port in ports {
        println!("\t{0}\t{1}", port.port_name, get_port_type(port.port_type));
    }
}

/**
 * returns a String with a brief decriptor for the port
 */
fn get_port_type(port_type: serialport::SerialPortType) -> String {
    return match port_type{
        serialport::SerialPortType::UsbPort(_) => {String::from("usb")}
        serialport::SerialPortType::PciPort => {String::from("pci")}
        serialport::SerialPortType::BluetoothPort => {String::from("bluetooth")}
       _ => {String::from("unknown")}
    };
}
