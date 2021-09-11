#![no_std]
#![no_main]

use crate::obdl1000::request::Request;

pub struct Reponse {
    request: Request,
}

impl Reponse {
    const FAULT_LARGE_CAPITAL: [u8; 3] = [b'N', b'S', b'!'];
    const FAULT_SMALL_CAPITAL: [u8; 3] = [b'b', b's', b'!'];

    // TX       : Always    WhileDisp   ActionHalted    SuccessDisp ProblemDisp
    // SayHi    : O
    // Init     :
    // Halt     :
    // HaltCancel:
    // RemCnt   :
    // GetTotal :
    // RemTotal :
    // StateCheck:
    // ErrorCheck:
}
