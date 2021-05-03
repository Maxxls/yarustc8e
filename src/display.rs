pub enum Display {
    Default([[bool; 64]; 32]),
    Big([[bool; 128]; 64]),
}

pub const DEFAULT_FONT: [[u8; 5]; 16] = [
    [0b11110000, 0b10010000, 0b10010000, 0b10010000, 0b11110000], // 0
    [0b00100000, 0b01100000, 0b00100000, 0b00100000, 0b01110000], // 1
    [0b11110000, 0b00010000, 0b11110000, 0b10000000, 0b11110000], // 2
    [0b11110000, 0b00010000, 0b11110000, 0b00010000, 0b11110000], // 3
    [0b10010000, 0b10010000, 0b11110000, 0b00010000, 0b00010000], // 4
    [0b11110000, 0b10000000, 0b11110000, 0b00010000, 0b11110000], // 5
    [0b11110000, 0b10000000, 0b11110000, 0b10010000, 0b11110000], // 6
    [0b11110000, 0b00010000, 0b00100000, 0b01000000, 0b01000000], // 7
    [0b11110000, 0b10010000, 0b11110000, 0b10010000, 0b11110000], // 8
    [0b11110000, 0b10010000, 0b11110000, 0b00010000, 0b11110000], // 9
    [0b11110000, 0b10010000, 0b11110000, 0b10010000, 0b10010000], // A
    [0b11100000, 0b10010000, 0b11100000, 0b10010000, 0b11100000], // B
    [0b11110000, 0b10000000, 0b10000000, 0b10000000, 0b11110000], // C
    [0b11100000, 0b10010000, 0b10010000, 0b10010000, 0b11100000], // D
    [0b11110000, 0b10000000, 0b11110000, 0b10000000, 0b11110000], // E
    [0b11110000, 0b10000000, 0b11110000, 0b10000000, 0b10000000], // F
];
