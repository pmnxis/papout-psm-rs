#![no_std]
#![no_main]

mod obdl1000_serial_request;
mod obdl1000_state;

pub struct SerialAnswer {
    request: SerialRequest,
}

struct SerialAnswer {
    capital: bool,
}

impl SerialAnswer {
    use crate::obdl1000_serial_request::*;
    use crate::obdl1000_state::*;

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
