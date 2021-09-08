#![no_std]

pub enum ErrorKind {
    UnknownShort = 0, // xtime <= 25 ms
    Empty = 1,        // 50  ms
    Jam = 2,          // 100 ms
    BillDouble = 3,   // 150 ms
    NotEmit = 4,      // 200 ms
    LengthLong = 5,   // 250 ms
    LengthShort = 6,  // 300 ms
    RejOver = 7,      // 350 ms
    TakeOut = 10,     // 500 ms
    MotorLock = 12,   // 600 ms
    Incline = 14,     // 700 ms
    UnknownLong = 15, // 725 ms <= xtime
    UnknownMid,
    Ok, //?Maybe?
}

impl ErrorKind {
    pub fn back_to_enum(x: u32) -> ErrorKind {
        match (x) {
            0 => ErrorKind::UnknownShort,
            1 => ErrorKind::Empty,
            2 => ErrorKind::Jam,
            3 => ErrorKind::BillDouble,
            4 => ErrorKind::NotEmit,
            5 => ErrorKind::LengthLong,
            6 => ErrorKind::LengthShort,
            7 => ErrorKind::RejOver,
            10 => ErrorKind::TakeOut,
            12 => ErrorKind::MotorLock,
            14 => ErrorKind::Incline,
            _ => {
                if 14 < x {
                    ErrorKind::UnknownLong
                } else {
                    ErrorKind::UnknownMid
                }
            }
        }
    }

    pub fn msec_to_enum(msec: u32) -> ErrorKind {
        ErrorKind::back_to_enum((msec + 24) / 50)
    }
}
