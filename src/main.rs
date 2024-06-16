#![feature(portable_simd)]

use std::io::Write;
use std::simd::num::SimdUint;
use std::simd::u16x8;

const SIZE: usize = 8;

const fn create_mult() -> u16x8 {
    let mut multiplicator = [0u16; SIZE];
    let mut i = 0;
    while i < SIZE {
        multiplicator[i] = i as u16 + 1;
        i += 1;
    }
    u16x8::from_array(multiplicator)
}

const MULT: u16x8 = create_mult();

const ALPHABET: [char; 29] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'm',
    'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'z',
];

const ASCII_TABLE: [u16; 128] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 10, 11, 12, 0, 13, 14, 15, 0, 16, 17, 0, 18, 19, 0, 20, 21, 22, 23, 24, 0, 25, 26, 27, 0,
    28, 0, 0, 0, 0, 0,
];

fn fill_from_str(bytes: &mut [u16], s: &[u8]) {
    for (index, i) in s.iter().enumerate() {
        bytes[index] = *i as u16;
    }
}

fn compute(bytes: &mut [u16; SIZE], result_char: &mut u16, chunk_number: usize) {
    for byte in bytes.iter_mut() {
        *byte = ASCII_TABLE[*byte as usize];
        if *byte == 0 {
            break;
        }
    }

    let simd_bytes = u16x8::from_slice(bytes);
    let chunk_number = u16x8::splat((chunk_number * SIZE) as u16);
    let new_mult = chunk_number + MULT;
    let product = simd_bytes * new_mult;
    let sum = product.reduce_sum();

    *result_char += sum;
}

fn main() {
    let mut stdout = std::io::stdout();
    let naan = "cb32752361";

    for _ in 0..100_000_000 {
        let naan_bytes = naan.as_bytes();
        let chunks = naan_bytes.chunks_exact(SIZE);
        let remainder = chunks.remainder();

        let mut chunk_number = 0;
        let mut char = 0;

        for chunk_data in chunks {
            let mut input: [u16; SIZE] = [0; SIZE];
            fill_from_str(&mut input, chunk_data);

            compute(&mut input, &mut char, chunk_number);
            chunk_number += 1;
        }

        if !remainder.is_empty() {
            let mut input: [u16; SIZE] = [0; SIZE];

            fill_from_str(&mut input, remainder);
            compute(&mut input, &mut char, chunk_number);
        }

        let result_char = ALPHABET[(char % 29) as usize];
        let _ = stdout.write(&[result_char as u8]);
    }
}

