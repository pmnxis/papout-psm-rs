#[allow(unused_macros)]
#[macro_export]
macro_rules! cap_u8 {
    ($foo: expr, $is_signed: expr) => {
        ($foo & 0x20) | (0x20 * (!$is_signed as u8))
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! caparr_TTT {
    ($a0: expr, $a1: expr, $a2: expr, $is_signed: expr) => {
        [
            cap_u8!($a0, $is_signed),
            cap_u8!($a1, $is_signed),
            cap_u8!($a2, $is_signed),
        ]
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! caparr_TTd {
    ($a0: expr, $a1: expr, $a2: expr, $is_signed: expr) => {
        [cap_u8!($a0, $is_signed), cap_u8!($a1, $is_signed), $a2]
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! caparr_Tdd {
    ($a0: expr, $a1: expr, $a2: expr, $is_signed: expr) => {
        [cap_u8!($a0, $is_signed), $a1, $a2]
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! caparr_TdT {
    ($a0: expr, $a1: expr, $a2: expr, $is_signed: expr) => {
        [cap_u8!($a0, $is_signed), $a1, cap_u8!($a2, $is_signed)]
    };
}
