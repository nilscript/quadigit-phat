use core::fmt;
use ht16k33::ValidationError;

#[derive(Debug)]
pub enum Error {
    NoMapping { character: char },
    HT16K33(ValidationError),
}

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            NoMapping { character } => write!(
                f,
                "'{:?}' has no mapping in 'fourletter_phat::bitmap::BITMAP'",
                character
            ),
            HT16K33(e) => write!(f, "{}", e),
        }
    }
}
