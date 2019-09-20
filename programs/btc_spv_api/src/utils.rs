use serde_derive::{Deserialize, Serialize};
use std::{fmt, num::ParseIntError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeHexError {
    InvalidLength(LengthError),
    ParseInt(ParseIntError),
}

impl From<ParseIntError> for DecodeHexError {
    fn from(e: ParseIntError) -> Self {
        DecodeHexError::ParseInt(e)
    }
}

impl fmt::Display for DecodeHexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DecodeHexError::InvalidLength(LengthError::OddLength) => {
                "input hex string length is odd ".fmt(f)
            }
            DecodeHexError::InvalidLength(LengthError::Maximum(e)) => {
                "input exceeds the maximum length".fmt(f)
            }
            DecodeHexError::InvalidLength(LengthError::Minimum(e)) => {
                "input does not meet the minimum length".fmt(f)
            }
            DecodeHexError::ParseInt(e) => e.fmt(f),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum LengthError {
    OddLength,
    Maximum(u32),
    Minimum(u32),
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, DecodeHexError> {
    if s.len() % 2 != 0 {
        Err(DecodeHexError::InvalidLength(LengthError::OddLength))
    } else {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.into()))
            .collect()
    }
}

pub fn measure_variable_int(vint: &[u8]) -> Result<usize, DecodeHexError> {
    let ln = vint.len();
    if ln > 9 {
        return Err(DecodeHexError::InvalidLength(LengthError::Maximum(9)));
    }

    let val: usize = match vint[0] {
        0..=252 => 1,
        253 => 2,
        254 => 5,
        255 => 9,
    };
    Ok(val)
}

pub fn decode_variable_int(vint: &[u8]) -> Result<u64, DecodeHexError> {
    let ln = vint.len();
    if ln > 9 {
        return Err(DecodeHexError::InvalidLength(LengthError::Maximum(9)));
    }

    let val: u64 = match vint[0] {
        0..=252 => u64::from(vint[0]),
        253 => u64::from(vint[1]),
        254 => {
            let mut val: [u8; 4] = [0; 4];
            val.copy_from_slice(&vint[1..5]);
            u64::from(u32::from_le_bytes(val))
        }
        255 => {
            let mut val: [u8; 8] = [0; 8];
            val.copy_from_slice(&vint[1..9]);
            u64::from_le_bytes(val)
        }
    };
    Ok(val)
}