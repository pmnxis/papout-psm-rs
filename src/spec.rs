const OBDL1000_TX_DATA: [[[u8; 3]; 10]; 2] = [
    [
        [b'H', b'I', b'?'], // 0, Check Protocol
        [b'i', 0x00, 0x00], // 1, Init
        [b'd', 0xFF, b's'], // 2, Export
        [b'h', 0x00, 0x00], // 3, Set Abort
        [b'h', b'c', b'?'], // 4, Clear Abort
        [b'r', b'e', b'm'], // 5, Clear Out Paper count.
        [b'g', b't', b'?'], // 6, 2Packet Style, Get Total Paper Count
        [b'c', b't', b'c'], // 7, 누적 배출 수량 Clear
        [b's', 0x00, 0x00], // 8, Get Current State
        [b's', b'e', b'r'], // 9, Get Error Code
    ],
    [
        [b'H', b'I', b'?'], // 0, Check Protocol
        [b'I', 0x00, 0x00], // 1, Init
        [b'D', 0xFF, b'S'], // 2, Export
        [b'H', 0x00, 0x00], // 3, Set Abort
        [b'H', b'C', b'?'], // 4, Clear Abort
        [b'R', b'E', b'M'], // 5, Clear Out Paper count.
        [b'G', b'T', b'?'], // 6, 2Packet Style, Get Total Paper Count
        [b'C', b'T', b'C'], // 7, 누적 배출 수량 Clear
        [b'S', 0x00, 0x00], // 8, Get Current State
        [b'S', b'E', b'R'], // 9, Get Error Code
    ],
];

const OBDL1000_RX_FAULT: [u8; 3] = [b'N', b'S', b'!'];

const OBDL1000_rx_data: [[u8; 3]; 11] = [
    [b'M', b'E', b'!'], // 0, Check Protocol
    [b'I', 0x00, b'A'], // 1, Init
    [b'D', 0xFF, b'A'], // 2, Export
    [b'H', 0x00, b'A'], // 3, Set Abort
    [b'H', b'C', b'!'], // 4, Clear Abort
    [b'R', 0xFF, b'O'], // 5, Clear Out Paper count.
    [b'T', 0xFF, 0xFF], // 6, 1-2Packet Style, Get Total Paper Count
    [b'C', b'T', b'!'], // 7, 누적 배출 수량 Clear
    [b'S', 0xFF, 0xFF], // 8, Get Current State
    [b'S', b'E', 0xFF], // 9, Get Error Code
    [b'G', 0xFF, 0xFF], // 10,2-2Packet Style, Get Total Paper Count
];
