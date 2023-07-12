#![no_main]
#![no_std]
use src_microbit as _;

use microbit::{hal::prelude::*, hal::twim, pac::twim0::frequency::FREQUENCY_A};

const ACCELEROMETER_ADDR: u8 = 0b0011001;
const MAGNETOMETER_ADDR: u8 = 0b0011110;

const ACCELEROMETER_ID_REG: u8 = 0x0f;
const MAGNETOMETER_ID_REG: u8 = 0x4f;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("i2c!");
    let board = microbit::Board::take().unwrap();
    let mut i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut acc = [0];
    let mut mag = [0];

    // First write the address + register onto the bus, then read the chip's responses
    i2c.write_read(ACCELEROMETER_ADDR, &[ACCELEROMETER_ID_REG], &mut acc)
        .unwrap();
    i2c.write_read(MAGNETOMETER_ADDR, &[MAGNETOMETER_ID_REG], &mut mag)
        .unwrap();

    defmt::println!("The accelerometer chip's id is: {:#b}", acc[0]);
    defmt::println!("The magnetometer chip's id is: {:#b}", mag[0]);

    loop {}
}
