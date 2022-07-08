extern crate clap;
extern crate serialport;

use std::io::{self, Write};
use std::ops::RangeInclusive;
use std::time::Duration;

use clap::{Arg, Command};

const BAUD_DEFAULT: &str = "115200";
const BAUD_RANGE: RangeInclusive<usize> = 50..=921600;
const TIMEOUT_DEFAULT: &str = "100";
const TIMEOUT_RANGE: RangeInclusive<usize> = 10..=30000;

/**
 * entrypoint for app
 */
fn main() {
    let command = Command::new("wolffserial")
        .about("a command line tool for working with serial devices")
        .disable_version_flag(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("watch")
                .about("watch a specific device")
                .arg(
                    Arg::with_name("port")
                        .help("the device path to a serial port")
                        .use_value_delimiter(false)
                        .required(true),
                )
                .arg(
                    Arg::new("baud")
                        .short('b')
                        .long("baud")
                        .help("the baud rate to connect at")
                        .use_value_delimiter(false)
                        .required(false)
                        .default_value(BAUD_DEFAULT)
                        .value_parser(baud_in_range)
                ).arg(
                    Arg::with_name("timeout")
                        .short('t')
                        .long("timeout")
                        .help("the timeout for opening the port")
                        .required(false)
                        .default_value(TIMEOUT_DEFAULT)
                        .value_parser(timeout_in_range)
                ),
        )
        .subcommand(
            Command::new("list")
                .about("lists available devices")
                .long_about(
                    "shows the number of connected/detected serial devices as well as a list of devices, \
                    what their paths/names are and what type of device they are detected as"
                )
        );

    let matches = command.get_matches();

    match matches.subcommand() {
        Some(("watch", matches)) => {
            // matches here is shadowing the matches defined above
            let port_name = match matches.get_one::<String>("port") {
                Some(val) => val,
                None => {
                    println!("port is required");
                    ::std::process::exit(0);
                }
            };
            let baud: u32 = match matches.get_one::<u32>("baud") {
                Some(baud) => {
                    // not the most concise way of getting the value out of an Option
                    *baud // but i am practicing matching
                }
                _ => unreachable!(),
            };
            let timeout: u64 = match matches.get_one::<u64>("timeout") {
                Some(timeout) => {
                    // not the most concise way of getting the value out of an Option
                    *timeout // but i am practicing matching
                }
                _ => unreachable!(),
            };
            let port = serialport::new(port_name, baud)
                .timeout(Duration::from_millis(timeout))
                .open();
            match port {
                Ok(mut port) => {
                    let mut serial_buf: Vec<u8> = vec![0; 1000];
                    println!("Receiving data on {} at {} baud:", &port_name, &baud);
                    loop {
                        match port.read(serial_buf.as_mut_slice()) {
                            Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
                            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                            Err(e) => {
                                eprintln!("{:?}", e);
                                ::std::process::exit(1);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
                    ::std::process::exit(1);
                }
            }
        }
        Some(("list", _)) => {
            list_ports();
        }
        None => {
            println!("no matching subcommand found");
            ::std::process::exit(0);
        }
        _ => unreachable!(),
    }
}

/**
	* value parser function to make sure baud is a number and in the range we expect
	*/
fn baud_in_range(s: &str) -> Result<u32, String> {
    let baud: usize = s.parse().map_err(|_| format!("`{}` isn't a number", s))?;
    if BAUD_RANGE.contains(&baud) {
        Ok(baud as u32)
    } else {
        Err(format!(
            "baud not in range {}-{}, if this is an error please post an issue @ github.com/wolffshots/wolffserial-rs/issues",
            BAUD_RANGE.start(),
            BAUD_RANGE.end()
        ))
    }
}

/**
* value parser function to make sure the timeout is a number or in the range we expect
*/
fn timeout_in_range(s: &str) -> Result<u64, String> {
    let timeout: usize = s.parse().map_err(|_| format!("`{}` isn't a number", s))?;
    if TIMEOUT_RANGE.contains(&timeout) {
        Ok(timeout as u64)
    } else {
        Err(format!(
            "timeout not in range {}-{}",
            TIMEOUT_RANGE.start(),
            TIMEOUT_RANGE.end()
        ))
    }
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
    return match port_type {
        serialport::SerialPortType::UsbPort(_) => String::from("usb"),
        serialport::SerialPortType::PciPort => String::from("pci"),
        serialport::SerialPortType::BluetoothPort => String::from("bluetooth"),
        _ => String::from("unknown"),
    };
}
