#![no_std]
#![no_main]

pub enum StateCode {
    Idle,
    WhileDispensing,
    ActionHalted,
    SuccessDispense(u8),
    ProblemDispense(u8),
}

impl StateCode {
    pub fn to_core_data(&self, capital: bool) -> [u8; 3] {
        match self {
            StateCode::Idle => caparr_TTT!(b'S', b'T', b'B', capital),
            StateCode::WhileDispensing => caparr_TTT!(b'S', b'O', b'N', capital),
            StateCode::ActionHalted => caparr_TTd!(b'S', b'H', b'!', capital),
            StateCode::SuccessDispense(x) => caparr_TdT!(b'S', x.clone(), b'O', capital),
            StateCode::ProblemDispense(x) => caparr_TdT!(b'S', x.clone(), b'N', capital),
        }
    }
}
