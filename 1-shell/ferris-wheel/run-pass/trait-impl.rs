// FIXME: Make me pass! Diff budget: 25 lines.
#[derive(Debug)]
enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

use Duration::MilliSeconds;
use Duration::Seconds;
use Duration::Minutes;

impl std::convert::From<u64> for Duration {
    fn from(num: u64) -> Self {
        Duration::MilliSeconds(num)
    }
}

impl PartialEq for Duration {
    fn eq(&self, other: &Duration) -> bool {
        let a: u64 = match self {
            &Duration::MilliSeconds(t) => t as u64,
            &Duration::Seconds(t) => t as u64 * 1000,
            &Duration::Minutes(t) => t as u64 * 1000 * 60,
        };
        let b: u64 = match other {
            &Duration::MilliSeconds(t) => t as u64,
            &Duration::Seconds(t) => t as u64 * 1000,
            &Duration::Minutes(t) => t as u64 * 1000 * 60,
        };
        a==b
    }
}

fn main() {
    assert_eq!(Seconds(120), Minutes(2));
    assert_eq!(Seconds(420), Minutes(7));
    assert_eq!(MilliSeconds(420000), Minutes(7));
    assert_eq!(MilliSeconds(43000), Seconds(43));
}
