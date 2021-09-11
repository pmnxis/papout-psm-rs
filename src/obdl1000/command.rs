#![no_std]
#![no_main]

pub enum Command {
    SayHi,
    Init,
    Dispense(u8), // diespense type should contain data.
    HaltAction,
    HaltActionCancel,
    RemoveCount,
    GetTotalDispensed,
    RemoveTotalCount,
    StateCheck,
    ErrorCheck,
}

pub enum Error {
    WrongCommand,
    WrongStart,
    WrongHash,
    WrongStartHash,
    WrongUnknown,
}

impl Request {
    fn hash(array: &[u8; 5]) -> u8 {
        array[1] + array[2] + array[3]
    }

    fn is_valid_hash(array: &[u8; 5]) -> bool {
        Request::hash(array) == array[4]
    }

    fn from_core_data(core_data: (u8, u8, u8)) -> Result<Request, Error> {
        match core_data {
            (b'H', b'I', b'?') => Ok(Request::SayHi),
            (b'I' | b'i', 0x00, 0x00) => Ok(Request::Init),
            (b'D', _, b'S') | (b'd', _, b's') => Ok(Request::Dispense(core_data.1)),
            (b'H' | b'h', 0x00, 0x00) => Ok(Request::HaltAction),
            (b'H', b'C', b'?') | (b'h', b'c', b'?') => Ok(Request::HaltActionCancel),
            (b'R', b'E', b'M') | (b'r', b'e', b'm') => Ok(Request::RemoveCount),
            (b'G', b'T', b'?') | (b'g', b't', b'?') => Ok(Request::GetTotalDispensed),
            (b'C', b'T', b'C') | (b'c', b't', b'c') => Ok(Request::RemoveTotalCount),
            (b'S' | b's', 0x00, 0x00) => Ok(Request::StateCheck),
            (b'S', b'E', b'R') | (b's', b'e', b'r') => Ok(Request::ErrorCheck),
            _ => Err(Error::WrongCommand),
        }
    }

    fn to_core_data(&self, capital: bool) -> [u8; 3] {
        match self {
            &Self::SayHi => caparr_TTd!(b'H', b'I', b'?', capital),
            &Self::Init => caparr_Tdd!(b'I', 0x00, 0x00, capital),
            &Self::Dispense(byte) => caparr_TdT!(b'D', byte, b'S', capital),
            &Self::HaltAction => caparr_Tdd!(b'H', 0x00, 0x00, capital),
            &Self::HaltActionCancel => caparr_TTd!(b'H', b'C', b'?', capital),
            &Self::RemoveCount => caparr_TTT!(b'R', b'E', b'M', capital),
            &Self::GetTotalDispensed => caparr_TTd!(b'G', b'T', b'?', capital),
            &Self::RemoveTotalCount => caparr_TTT!(b'C', b'T', b'C', capital),
            &Self::StateCheck => caparr_Tdd!(b's', 0x00, 0x00, capital),
            &Self::ErrorCheck => caparr_TTT!(b'S', b'E', b'R', capital),
        }
    }

    pub fn from_array(array: &[u8; 5]) -> Result<Request, Error> {
        match (array[0], Request::is_valid_hash(array)) {
            (b'$', true) => Request::from_core_data((array[1], array[2], array[3])),
            (_, true) => Err(Error::WrongStart),
            (b'$', false) => Err(Error::WrongHash),
            _ => Err(Error::WrongStartHash),
        }
    }
}
