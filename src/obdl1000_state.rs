#![no_std]
#![no_main]

macro_rules! cap_u8 {
    ($foo: expr, $is_signed: expr) => {
        ($foo & 0x20) | (0x20 * (!$is_signed as u8))
    };
}

macro_rules! caparr_TTT {
    ($a0: expr, $a1: expr, $a2: expr, $is_signed: expr) => {
        [
            cap_u8!($a0, $is_signed),
            cap_u8!($a1, $is_signed),
            cap_u8!($a2, $is_signed),
        ]
    };
}

macro_rules! caparr_TTd {
    ($a0: expr, $a1: expr, $a2: expr, $is_signed: expr) => {
        [cap_u8!($a0, $is_signed), cap_u8!($a1, $is_signed), $a2]
    };
}

macro_rules! caparr_Tdd {
    ($a0: expr, $a1: expr, $a2: expr, $is_signed: expr) => {
        [cap_u8!($a0, $is_signed), $a1, $a2]
    };
}

macro_rules! caparr_TdT {
    ($a0: expr, $a1: expr, $a2: expr, $is_signed: expr) => {
        [cap_u8!($a0, $is_signed), $a1, cap_u8!($a2, $is_signed)]
    };
}

pub enum StateKind {
    Idle,
    WhileDispensing,
    ActionHalted,
    SuccessDispense(u8),
    ProblemDispense(u8),
}

impl StateKind {
    pub fn to_core_data(&self, capital: bool) -> [u8; 3] {
        match self {
            StateKind::Idle => caparr_TTT!(b'S', b'T', b'B', capital),
            StateKind::WhileDispensing => caparr_TTT!(b'S', b'O', b'N', capital),
            StateKind::ActionHalted => caparr_TTd!(b'S', b'H', b'!', capital),
            StateKind::SuccessDispense(x) => caparr_TdT!(b'S', x.clone(), b'O', capital),
            StateKind::ProblemDispense(x) => caparr_TdT!(b'S', x.clone(), b'N', capital),
        }
    }
}
