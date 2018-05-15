// use stack_vec::StackVec;
use console::{kprint, kprintln, CONSOLE};
use std::path::{Path, PathBuf};
use pi::timer::{current_time, spin_sleep_ms};
use pi::gpio::Gpio;
#[cfg(not(test))]
use std::io::Read;
use std::str;

const BEL: u8 = 0x07;
const BS:  u8 = 0x08;
const LF:  u8 = 0x0a;
const CR:  u8 = 0x0d;
const ESC: u8 = 0x1b;
const DEL: u8 = 0x7f;
const BANNER: &str = r#"
                 _ _ ___         
             __-         `-_    
         ___/__        〇   \ 
     - '     _/             /
   '_'             /
 / _- ---            __ -
/`     |          _ / \  \
       |       -       \ |
        \    /          V
          \  |
            \ \
              \"#;


/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    // TooManyArgs,
}

/// A structure representing a single shell command.
struct Command<'a> {
    // args: StackVec<'a, &'a str>,
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
            // args.push(arg).map_err(|_| Error::TooManyArgs)?;
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
    spin_sleep_ms(200); // wait a little time for client to attach
    kprintln!("{}", BANNER);

    let mut history = Vec::new();
    let mut cwd = PathBuf::from("/");
    loop {
        kprint!("{}{}", cwd.display(), prefix);

        let line = read_line(&mut history);
        match Command::parse(&line) {
            Ok(command) => {
                let path = command.path();
                match path {
                    "echo" => echo(&command),
                        _ => kprintln!("unknown command: {}", path),
                }
            }
            Err(Error::Empty) => {
                // Ignore
            }
        }
    }
}

fn read_line(history: &mut Vec<Vec<u8>>) -> String {
    let mut console = CONSOLE.lock();
    let mut cursor = 0;
    let mut line_vec = Vec::with_capacity(512);
    let mut history_index = history.len();
    loop {
        match console.read_byte() {
            BS | DEL => {
                // Backspace
                if cursor > 0 {
                    cursor -= 1;
                    line_vec.remove(cursor);

                    console.write_byte(BS);
                    for byte in &line_vec[cursor..] {
                        console.write_byte(*byte);
                    }
                    console.write_byte(b' ');
                    for _i in cursor..line_vec.len() {
                        console.write_byte(BS);
                    }
                    console.write_byte(BS);
                } else {
                    console.write_byte(BEL);
                }
            }
            CR | LF => {
                // Return
                console.write_byte(CR);
                console.write_byte(LF);
                break;
            }
            ESC => {
                match console.read_byte() {
                    b'[' => {
                        match console.read_byte() {
                            b'D' => {
                                // Left arrow
                                if cursor > 0 {
                                    cursor -= 1;
                                    console.write_byte(ESC);
                                    console.write_byte(b'[');
                                    console.write_byte(b'D');
                                } else {
                                    console.write_byte(BEL);
                                }
                            }
                            b'C' => {
                                // Right arrow
                                if cursor < line_vec.len() {
                                    cursor += 1;
                                    console.write_byte(ESC);
                                    console.write_byte(b'[');
                                    console.write_byte(b'C');
                                } else {
                                    console.write_byte(BEL);
                                }
                            }
                            direction @ b'A' | direction @ b'B' => {
                                if direction == b'A' && history_index > 0 {
                                    // Up arrow
                                    history_index -= 1;
                                } else if direction == b'B' && history.len() > 0 // usize underflow
                                    && history_index < history.len() - 1
                                    {
                                        // Down arrow
                                        history_index += 1;
                                    } else {
                                        console.write_byte(BEL);
                                        continue;
                                    }

                                for _ in 0..line_vec.len() {
                                    console.write_byte(BS);
                                }
                                for _ in 0..line_vec.len() {
                                    console.write_byte(b' ');
                                }
                                for _ in 0..line_vec.len() {
                                    console.write_byte(BS);
                                }
                                line_vec = history[history_index].clone();
                                cursor = line_vec.len();
                                for byte in &line_vec {
                                    console.write_byte(*byte);
                                }
                            }
                            _ => {
                                console.write_byte(BEL);
                            }
                        }
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

    history.push(line_vec.clone());
    String::from_utf8(line_vec).unwrap_or_default()
}
    fn echo(command: &Command) {
        if command.args.len() > 1 {
            kprint!("{}", command.args[1]);
            if command.args.len() > 2 {
                for arg in &command.args[2..] {
                    kprint!(" {}", *arg);
                }
            }
        }

        kprintln!();
    }
