use std::str;
use std::io::BufReader;
use std::io::BufRead;
use std::i16;
use hound;
use std::time::{Duration, Instant};

const BUFFER_SIZE: usize = 1024;

fn get_port() -> Option<String> {
    let ports = serialport::available_ports().expect("No ports found!");

    for p in ports {
        if p.port_name.contains("tty.usbmodem") {
            return Some(p.port_name);
        }
    }
    None
}

fn main() {
    let port = get_port().expect("device not found");
    println!("found: {}", port);

    let port = serialport::new(port, 115200)
        .timeout(Duration::from_millis(10))
        .open().expect("Failed to open port");
    let mut port = BufReader::new(port);

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 1000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("record.wav", spec).unwrap();

    let mut buf = vec![];
    let mut count = 0;
    let mut sum = 0.;
    let mut max = 0.;
    let mut min = 900000.;
    let start = Instant::now();
    while count < 1000 * 5 {
        let n = port.read_until('\n' as u8, &mut buf).unwrap();
        let sample = str::from_utf8(&buf[..n]).unwrap().trim();
        let sample: f32 = sample.parse().unwrap();
        let sample = sample - 2386.;
        // println!("{}", sample);
        let sample = sample * 133.0;
        sum += sample;
        if max < sample {
            max = sample;
        }
        if min > sample {
            min = sample;
        }

        let amplitude = i16::MAX as f32;
        writer.write_sample((sample / 4096. * amplitude) as i16).unwrap();

        buf.clear();
        count += 1;
    }
    let duration = start.elapsed();

    println!("average: {}", sum / 5000.);
    println!("max: {}", max);
    println!("min: {}", min);
    println!("elapsed time: {:?}", duration);
}
