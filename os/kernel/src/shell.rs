use stack_vec::StackVec;
use console::{kprint, kprintln, CONSOLE};
use pi::timer::spin_sleep_ms;

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
    TooManyArgs,
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>,
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
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
    let mut command_storage = [0u8; 512];
    let mut raw_command = StackVec::new(&mut command_storage);
    let mut cwd = PathBuf::from("/");

    loop {
        kprint!("{}{}", cwd.display(), prefix);

        let this_command = read_command(&mut raw_command);
        match Command::parse(&this_command, &command_storage) {
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

fn read_command(raw_command: &mut StackVec<StackVec<u8>>) -> [u8] {
    let mut console = CONSOLE.lock();
    let mut cursor = 0;
    let mut command_storage = [0u8; 512];
    let mut raw_command_line = StackVec::new(&mut command_storage);
    let mut raw_command_index = raw_command.len();

    loop {
        match console.read_byte() {
            BS | DEL => {
                // backspace
                if cursor > 0 {
                    cursor -= 1;
                    raw_command_line.remove(cursor);
                    console.write_byte(BS);
                    for bytes in &raw_command_line[cursor..] {
                        console.write_byte(*bytes);
                    }
                    console.write_byte(b' ');
                    for _i in cursor..raw_command_line.len() {
                        console.write_byte(BS);
                    }
                    console.write_byte(BS);
                } else {
                    console.write_byte(BEL);
                }
            }
            CR | LF => {
                // return
                console.write_byte(CR);
                console.write_byte(LF);
                break; // end of loop
            }
            bytes if bytes.is_ascii_graphic() || bytes == b' ' => {
                raw_command_line.insert(cursor, bytes);
                for bytes in &raw_command_line[cursor..] {
                    console.write_byte(*bytes);
                }
                cursor += 1;
                for _i in cursor..raw_command_line.len() {
                    console.write_byte(BS);
                }
            }
            _ => {
                // unrecognized characters
                console.write_byte(BEL);
            }
        }
        raw_command.push(raw_command_line.clone());
        raw_command_line.as_slice()
    }
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
