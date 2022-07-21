#![no_std]
#![no_main]

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ErrorCode {
    // Name             Parallel    Serial
    Ok,              // noSig       0x80
    Empty = 1,       // 50  ms      0x81
    Jam = 2,         // 100 ms      0x82
    BillDouble = 3,  // 150 ms      0x83
    NotEmit = 4,     // 200 ms      0x84
    LengthLong = 5,  // 250 ms      0x85
    LengthShort = 6, // 300 ms      0x86
    RejOver = 7,     // 350 ms      0x87
    TakeOut = 10,    // 500 ms      0x8A
    MotorLock = 12,  // 600 ms      0x8C
    Incline = 14,    // 700 ms      0x8E
}

pub enum ParseError {
    UnknownShort, // xtime <= 25 ms
    UnknownLong,  // 725 ms <= xtime
    UnknownMid,
}

impl ErrorCode {
    pub fn back_to_enum(x: u32) -> Result<ErrorCode, ParseError> {
        match (x) {
            x if x == ParseError::UnknownShort as u32 => Err(ParseError::UnknownShort),
            x if x == ErrorCode::Empty as u32 => Ok(ErrorCode::Empty),
            x if x == ErrorCode::Jam as u32 => Ok(ErrorCode::Jam),
            x if x == ErrorCode::BillDouble as u32 => Ok(ErrorCode::BillDouble),
            x if x == ErrorCode::NotEmit as u32 => Ok(ErrorCode::NotEmit),
            x if x == ErrorCode::LengthLong as u32 => Ok(ErrorCode::LengthLong),
            x if x == ErrorCode::LengthShort as u32 => Ok(ErrorCode::LengthShort),
            x if x == ErrorCode::RejOver as u32 => Ok(ErrorCode::RejOver),
            x if x == ErrorCode::TakeOut as u32 => Ok(ErrorCode::TakeOut),
            x if x == ErrorCode::MotorLock as u32 => Ok(ErrorCode::MotorLock),
            x if x == ErrorCode::Incline as u32 => Ok(ErrorCode::Incline),
            _ => match (14 < x) {
                true => Err(ParseError::UnknownLong),
                false => Err(ParseError::UnknownMid),
            },
        }
    }

    pub fn msec_to_enum(msec: u32) -> Result<ErrorCode, ParseError> {
        ErrorCode::back_to_enum((msec + 24) / 50)
    }

    pub fn to_serial_code(&self) -> u8 {
        0x80 + *self as u8
    }

    pub fn to_serial_core_data(&self, capital: bool) -> [u8; 3] {
        caparr_TTd!(b's', b'e', self.to_serial_code(), capital)
    }

    pub fn to_parallel_time(&self) -> u32 {
        50 * (*self as u32)
    }
}
