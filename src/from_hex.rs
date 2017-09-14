use std;

// JUNK FROM UNSTABLE

#[derive(Copy, Clone, Debug)]
pub enum FromHexError {
    /// The input contained a character not part of the hex format
    InvalidHexCharacter(char, usize),
    /// The input had an invalid length
    InvalidHexLength,
}

impl std::fmt::Display for FromHexError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            FromHexError::InvalidHexCharacter(ch, idx) => {
                write!(f, "Invalid character '{}' at position {}", ch, idx)
            }
            FromHexError::InvalidHexLength => write!(f, "Invalid input length"),
        }
    }
}

impl std::error::Error for FromHexError {
    fn description(&self) -> &str {
        match *self {
            FromHexError::InvalidHexCharacter(..) => "invalid character",
            FromHexError::InvalidHexLength => "invalid length",
        }
    }
}

pub trait FromHex {
    fn from_hex(&self) -> Result<Vec<u8>, FromHexError>;
}

impl FromHex for str {
    /// Convert any hexadecimal encoded string (literal, `@`, `&`, or `~`)
    /// to the byte values it encodes.
    ///
    /// You can use the `String::from_utf8` function to turn a
    /// `Vec<u8>` into a string with characters corresponding to those values.
    fn from_hex(&self) -> Result<Vec<u8>, FromHexError> {
        // This may be an overestimate if there is any whitespace
        let mut b = Vec::with_capacity(self.len() / 2);
        let mut modulus = 0;
        let mut buf = 0;

        for (idx, byte) in self.bytes().enumerate() {
            buf <<= 4;

            match byte {
                b'A'...b'F' => buf |= byte - b'A' + 10,
                b'a'...b'f' => buf |= byte - b'a' + 10,
                b'0'...b'9' => buf |= byte - b'0',
                b' ' | b'\r' | b'\n' | b'\t' => {
                    buf >>= 4;
                    continue;
                }
                _ => {
                    let ch = self[idx..].chars().next().unwrap();
                    return Err(FromHexError::InvalidHexCharacter(ch, idx));
                }
            }

            modulus += 1;
            if modulus == 2 {
                modulus = 0;
                b.push(buf);
            }
        }

        match modulus {
            0 => Ok(b.into_iter().collect()),
            _ => Err(FromHexError::InvalidHexLength),
        }
    }
}
