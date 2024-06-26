use std::time::{Duration, Instant};
use serialport::SerialPort;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::fs::File;
use std::f32::consts::PI;
use std::i16;
use hound;
use hound::WavWriter;

const SAMPLING_RATE: usize = 40000;
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

fn start_record(port: Box<dyn SerialPort>, mut writer: WavWriter<BufWriter<File>>) {
    let mut port = BufReader::new(port);

    let mut buf = [0; 2];
    for _ in 0..SAMPLING_RATE * RECORD_DURATION_SECOND {
        port.read_exact(&mut buf).unwrap();
        let sample = (buf[0] as i16) << 8 | buf[1] as i16;
        let sample = sample - 2382;
        println!("{}", sample);

        let sample = sample as f32 * 32768.0 / 4096.0 * 64.0;
        writer.write_sample(sample as i16).unwrap();
    }
}

fn main() {
    let port = get_port().expect("device not found");
    println!("found: {}", port);

    let port = serialport::new(port, 115200)
        .timeout(Duration::from_millis(10))
        .open().expect("Failed to open port");

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLING_RATE as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let writer = hound::WavWriter::create("record.wav", spec).unwrap();

    let start = Instant::now();
    start_record(port, writer);
    let duration = start.elapsed();
    println!("elapsed time: {:?}", duration);
}
