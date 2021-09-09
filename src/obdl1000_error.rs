#![no_std]
#![no_main]

#[derive(Copy, Clone)]
pub enum ErrorType {
    Ok,              // Maybe
    Empty = 1,       // 50  ms
    Jam = 2,         // 100 ms
    BillDouble = 3,  // 150 ms
    NotEmit = 4,     // 200 ms
    LengthLong = 5,  // 250 ms
    LengthShort = 6, // 300 ms
    RejOver = 7,     // 350 ms
    TakeOut = 10,    // 500 ms
    MotorLock = 12,  // 600 ms
    Incline = 14,    // 700 ms
}

pub enum ParseError {
    UnknownShort, // xtime <= 25 ms
    UnknownLong,  // 725 ms <= xtime
    UnknownMid,
}

impl ErrorType {
    pub fn back_to_enum(x: u32) -> Result<ErrorType, ParseError> {
        match (x) {
            0 => Err(ParseError::UnknownShort),
            1 => Ok(ErrorType::Empty),
            2 => Ok(ErrorType::Jam),
            3 => Ok(ErrorType::BillDouble),
            4 => Ok(ErrorType::NotEmit),
            5 => Ok(ErrorType::LengthLong),
            6 => Ok(ErrorType::LengthShort),
            7 => Ok(ErrorType::RejOver),
            10 => Ok(ErrorType::TakeOut),
            12 => Ok(ErrorType::MotorLock),
            14 => Ok(ErrorType::Incline),
            _ => match (14 < x) {
                true => Err(ParseError::UnknownLong),
                false => Err(ParseError::UnknownMid),
            },
        }
    }

    pub fn msec_to_enum(msec: u32) -> Result<ErrorType, ParseError> {
        ErrorType::back_to_enum((msec + 24) / 50)
    }

    pub fn to_serial_code(&self) -> u8 {
        0x80 + *self as u8
    }

    pub fn to_serial_core_data(&self, capital: bool) -> [u8; 3] {
        match capital {
            false => [b's', b'e', self.to_serial_code()],
            true => [b'S', b'E', self.to_serial_code()],
        }
    }

    pub fn to_parallel_time(&self) -> u32 {
        50 * (*self as u32)
    }
}
