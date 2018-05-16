#![feature(duration_extras)]
extern crate serial;
extern crate structopt;
extern crate xmodem;
#[macro_use] extern crate structopt_derive;

use std::time::Instant;
use std::path::PathBuf;
use std::time::Duration;

use structopt::StructOpt;
use serial::core::{CharSize, BaudRate, StopBits, FlowControl, SerialDevice, SerialPortSettings};
use xmodem::{Xmodem, Progress};

mod parsers;

use parsers::{parse_width, parse_stop_bits, parse_flow_control, parse_baud_rate};

#[derive(StructOpt, Debug)]
#[structopt(about = "Write to TTY using the XMODEM protocol by default.")]
struct Opt {
    #[structopt(short = "i", help = "Input file (defaults to stdin if not set)", parse(from_os_str))]
    input: Option<PathBuf>,

    #[structopt(short = "b", long = "baud", parse(try_from_str = "parse_baud_rate"),
                help = "Set baud rate", default_value = "115200")]
    baud_rate: BaudRate,

    #[structopt(short = "t", long = "timeout", parse(try_from_str),
                help = "Set timeout in seconds", default_value = "10")]
    timeout: u64,

    #[structopt(short = "w", long = "width", parse(try_from_str = "parse_width"),
                help = "Set data character width in bits", default_value = "8")]
    char_width: CharSize,

    #[structopt(help = "Path to TTY device", parse(from_os_str))]
    tty_path: PathBuf,

    #[structopt(short = "f", long = "flow-control", parse(try_from_str = "parse_flow_control"),
                help = "Enable flow control ('hardware' or 'software')", default_value = "none")]
    flow_control: FlowControl,

    #[structopt(short = "s", long = "stop-bits", parse(try_from_str = "parse_stop_bits"),
                help = "Set number of stop bits", default_value = "1")]
    stop_bits: StopBits,

    #[structopt(short = "r", long = "raw", help = "Disable XMODEM")]
    raw: bool,
}

fn main() {
    use std::fs::File;
    use std::io::{self, BufReader};

    let opt = Opt::from_args();
    let mut serial = serial::open(&opt.tty_path).expect("path points to invalid TTY");

    // set the serial settings
    let mut serial_settings = serial.read_settings().unwrap();
    // println!("current serial_settings: {:?}", serial_settings);
    serial_settings.set_baud_rate(opt.baud_rate).expect("set baud_rate ok");
    serial_settings.set_char_size(opt.char_width);
    serial_settings.set_flow_control(opt.flow_control);
    serial_settings.set_stop_bits(opt.stop_bits);
    serial.set_timeout(Duration::from_secs(opt.timeout)).expect("set timeout ok");
    serial.write_settings(&serial_settings).expect("write settings ok");
    println!("raw: {}", &opt.raw);
    // serial_settings = serial.read_settings().unwrap();
    // println!("current serial_settings: {:?}", serial_settings);

    // transmit stdin in the raw or via the XMODEM protocol
    if opt.raw == true {
        if opt.input == None {
            let mut input_stdin = io::stdin();
            io::copy(&mut input_stdin, &mut serial).expect("copy ok");
        } else {
            let mut input_file = BufReader::new(File::open(&opt.input.unwrap()).expect("open input_file ok"));
            io::copy(&mut input_file, &mut serial).expect("copy ok");
        }
    } else {
        if opt.input == None {
            let input_stdin = io::stdin();
            let tx_thread = std::thread::spawn(
                move || Xmodem::transmit_with_progress(input_stdin, &mut serial, progress_fn));
            println!("wrote {} bytes to input", 
                     tx_thread.join().expect("tx join ok").expect("tx ok"));
        } else {
            let input_file = BufReader::new(File::open(&opt.input.unwrap()).expect("open input_file ok"));
            let tx_thread = std::thread::spawn(
                move || Xmodem::transmit_with_progress(input_file, &mut serial, progress_fn));
            println!("wrote {} bytes to input", 
                     tx_thread.join().expect("tx join ok").expect("tx ok"));
        }
    }
}

fn progress_fn(progress: Progress) {
    println!("Progress: {:?}", progress);
    static mut LAST_TIME: Option<Instant> = None;
    static mut BYTES_SENT: u64 = 0;
    unsafe {
        BYTES_SENT += 128;
        LAST_TIME = match LAST_TIME {
            Some(last_time) => {
                let now = Instant::now();
                let duration = now - last_time;
                let nanos = duration.as_secs() * 1_000_000_000 + duration.subsec_nanos() as u64;
                println!(
                    "Progress: {} bytes sent at {:.2} Kib/s",
                    BYTES_SENT,
                    128.0 * 1_000_000_000.0 / 1024.0 / nanos as f64
                    );
                Some(now)
            }
            None => Some(Instant::now()),
        };
    }
}
