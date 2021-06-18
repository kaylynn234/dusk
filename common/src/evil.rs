// I am not truly cognizant of why these work, or how.
const DE_BRUIJN_BIT_POSITION: [u32; 32] = [
    0, 9, 1, 10, 13, 21, 2, 29, 11, 14, 16, 18, 22, 25, 3, 30, 8, 12, 20, 28, 15, 17, 24, 7, 19,
    27, 23, 6, 26, 5, 4, 31,
];

const MAGIC_NUMBER: u32 = 0x07C4ACDD;

pub fn log2(mut value: u32) -> u32 {
    for shift in [1, 2, 4, 8, 16] {
        value |= value >> shift;
    }

    DE_BRUIJN_BIT_POSITION[(value.wrapping_mul(MAGIC_NUMBER) >> 27) as usize]
}

const POWERS_OF_10: [u32; 10] = [
    1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
];

pub fn log10(value: u32) -> u32 {
    let cursed_index = (log2(value) + 1) * 1233 >> 12;
    cursed_index - (value < POWERS_OF_10[cursed_index as usize]) as u32
}
