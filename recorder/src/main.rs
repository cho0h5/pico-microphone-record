use std::time::{Duration, Instant};
use serialport::SerialPort;
use std::io::prelude::*;
use std::io::BufReader;

const SAMPLING_RATE: usize = 1000;
const RECORD_DURATION_SECOND: usize = 5;

fn get_port() -> Option<String> {
    let ports = serialport::available_ports().expect("No ports found!");

    for p in ports {
        if p.port_name.contains("tty.usbmodem") {
            return Some(p.port_name);
        }
    }
    None
}

fn start_record(port: Box<dyn SerialPort>) {
    let mut port = BufReader::new(port);

    let mut line = String::new();
    for _ in 0..SAMPLING_RATE * RECORD_DURATION_SECOND {
        port.read_line(&mut line).unwrap();
        let sample: u16 = line.trim().parse().unwrap();
        println!("{}", sample);
        line.clear();
    }
}

fn main() {
    let port = get_port().expect("device not found");
    println!("found: {}", port);

    let port = serialport::new(port, 115200)
        .timeout(Duration::from_millis(10))
        .open().expect("Failed to open port");

    let start = Instant::now();
    start_record(port);
    let duration = start.elapsed();
    println!("elapsed time: {:?}", duration);
}
