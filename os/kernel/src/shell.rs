// use stack_vec::StackVec;
use console::{kprint, kprintln, CONSOLE};
use std::path
use pi::timer::spin_sleep_ms;
use std::str;
#[cfg(not(test))]
use std::io::Read;

const BEL: u8 = 0x07;
const BS:  u8 = 0x08;
const LF:  u8 = 0x0a;
const CR:  u8 = 0x0d;
const ESC: u8 = 0x1b;
const DEL: u8 = 0x7f;

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    // TooManyArgs,
}

/// A structure representing a single shell command.
struct Command<'a> {
    // args: StackVec<'a, &'a str>
    args: Vec<&'a str>,
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    // fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
    //     let mut args = StackVec::new(buf);
    fn parse(s: &str) -> Result<Command, Error> {
        let mut args = Vec::with_capacity(64);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg);
        }
        if args.is_empty() {
            return Err(Error::Empty);
        }
        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
        self.args[0]
    }
}

/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns: it is perpetually in a shell loop.
pub fn shell(prefix: &str) -> ! {
    spin_sleep_ms(200);
    let mut raw_command = Vec::new();

    loop {
        let this_command = read_command(&mut raw_command);
        match Command::parse(&this_command) {
            Ok(command) => {
                let command_path = command.path();
                match command_path {
                    "echo" => mini_echo(&command),
                        _      => kprintln!("unknown command: {}", command_path),
                }
            }
            Err(Error::Empty) => {
            }
        }
    }
}

fn read_command(raw_command: &mut Vec<Vec<u8>>) -> String {
    let mut console = CONSOLE.lock();
    let mut cursor = 0;
    let mut line_vec = Vec::with_capacity(512);
    let mut history_index = raw_command.len();
    loop {
        match console.read_byte() {
            BS | DEL => {
                // backspace
                if cursor > 0 {
                    cursor -= 1;
                    line_vec.remove(cursor);
                    console.write_byte(BS);
                    // for bytes in &line_vec[cursor..] {
                    //     console.write_byte(*bytes);
                    // }
                    console.write_byte(b' ');
                    // for _ in cursor..line_vec.len() {
                    // console.write_byte(BS);
                    // }
                    console.write_byte(BS);
                } else {
                    console.write_byte(BEL);
                }
            }
            CR | LF => {
                // return
                console.write_byte(CR);
                console.write_byte(LF);
                break;
            }
            ESC => {
                match console.read_byte() {
                    b'[' => {
                    }
                    _ => {
                        console.write_byte(BEL);
                    }
                }
            }
            byte if byte.is_ascii_graphic() || byte == b' ' => {
                line_vec.insert(cursor, byte);
                for byte in &line_vec[cursor..] {
                    console.write_byte(*byte);
                }
                cursor += 1;
                for _i in cursor..line_vec.len() {
                    console.write_byte(BS);
                }
            }
            _ => {
                // unrecognized characters
                console.write_byte(BEL);
            }
        }
    }
    raw_command.push(line_vec.clone());
    String::from_utf8(line_vec).unwrap_or_default()
}
    fn mini_echo(command: &Command) {
        if command.args.len() > 1 {
            kprint!("{}", command.args[1]);
            if command.args.len() > 2 {
                for command_para in &command.args[2..] {
                    kprint!(" {}", *command_para);
                }
            }
        }
        kprintln!();
    }
