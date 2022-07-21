#![no_std]
#![no_main]

use super::state_code::StateCode;

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum Request {
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
    const FAULT_LARGE_CAPITAL: [u8; 3] = [b'N', b'S', b'!'];
    const FAULT_SMALL_CAPITAL: [u8; 3] = [b'b', b's', b'!'];

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

    pub fn action_possibility(&self, state: &StateCode) -> bool {
        match (self) {
            // - Say Hi -
            // SayHi always echo action.
            Request::SayHi => true,

            // - Init -
            Request::Init => true, // not sure about this.

            // - Dispense -
            Request::Dispense(0) => false, // Dispnese 0 paper is not allowed.
            Request::Dispense(_) => match (state) {
                // Dispense is not allowed while busy.
                StateCode::WhileDispensing => false,
                // When halted(inhibit mode) not allowed.
                StateCode::ActionHalted => false,
                // If there's problem, not allowed.
                StateCode::ProblemDispense(_) => false,
                _ => true,
            },

            // - HaltAction -
            Request::HaltAction => match (state) {
                // Halted on halt action now allowed (???)
                StateCode::ActionHalted => false,
                _ => true,
            },

            // - HaltActionCancel -
            Request::HaltActionCancel => match (state) {
                // I don't know
                _ => true,
            },

            // - RemoveCount -
            Request::RemoveCount => match (state) {
                // WhileDispensing counting value is locked, thus not allowed.
                StateCode::WhileDispensing => false,
                _ => true,
            },

            // - GetTotalDispensed -
            Request::GetTotalDispensed => match (state) {
                // WhileDispensing counting value is locked, thus not allowed.
                StateCode::WhileDispensing => false,
                _ => true,
            },

            // - RemoveTotalCount -
            Request::RemoveTotalCount => match (state) {
                // WhileDispensing counting value is locked, thus not allowed.
                StateCode::WhileDispensing => false,
                _ => true,
            },

            // - StateCheck & ErrorCheck -
            Request::StateCheck => true, // always allowed to read.
            Request::ErrorCheck => true, // always allowed to read.
        }
    }

    pub fn action_response(
        &self,
        state: &StateCode,
        capital: bool,
        extra: Option<u32>,
    ) -> (bool, Option<[u8; 3]>, Option<[u8; 3]>) {
        let possible = self.action_possibility(state);
        let core_data_1 = match (possible) {
            false => match (capital) {
                true => Request::FAULT_LARGE_CAPITAL,
                false => Request::FAULT_SMALL_CAPITAL,
            },
            true => match (self) {
                &Self::SayHi => caparr_TTd!(b'M', b'E', b'!', capital),
                &Self::Init => caparr_Tdd!(b'I', 0x00, b'A', capital),
                &Self::Dispense(byte) => caparr_TdT!(b'D', byte, b'A', capital),
                &Self::HaltAction => caparr_Tdd!(b'H', 0x00, b'A', capital),
                &Self::HaltActionCancel => caparr_TTd!(b'H', b'C', b'?', capital),
                &Self::RemoveCount => caparr_TdT!(b'R', 0xFF, b'M', capital),
                &Self::GetTotalDispensed => {
                    caparr_Tdd!(
                        b'T',
                        (extra.unwrap_or(0) >> 24) as u8,
                        (extra.unwrap_or(0) >> 16) as u8,
                        capital
                    )
                }
                &Self::RemoveTotalCount => caparr_TTT!(b'C', b'T', b'C', capital),
                &Self::StateCheck => caparr_Tdd!(b's', 0x00, 0x00, capital),
                &Self::ErrorCheck => caparr_TTT!(b'S', b'E', b'R', capital),
            },
        };

        match (self) {
            &Self::GetTotalDispensed => (
                possible,
                Some(core_data_1),
                Some(caparr_Tdd!(
                    b'G',
                    (extra.unwrap_or(0) >> 8) as u8,
                    (extra.unwrap_or(0)) as u8,
                    capital
                )),
            ),
            _ => (possible, Some(core_data_1), None),
        }
    }
}
