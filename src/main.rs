extern crate ssd1306;
extern crate tinybmp;
extern crate lazy_static;
extern crate embedded_graphics;
extern crate linux_embedded_hal as hal;

use hal::I2cdev;

use ssd1306::Builder;
use ssd1306::prelude::*;
use ssd1306::mode::GraphicsMode;
use ssd1306::interface::i2c::I2cInterface;

use tinybmp::Bmp;

use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
};

use std::thread::sleep;
use std::time::Duration;

const SSD1306_ADDR: u8 = 0x3C;
const I2C_DEV_LOCATION: &str = "/dev/i2c-1";

type Ssd1306Display = GraphicsMode<I2cInterface<I2cdev>>;

include!(concat!(env!("OUT_DIR"), "/bmps.rs"));

fn main() {
    let i2c_dev = I2cdev::new(I2C_DEV_LOCATION).expect("Failed to open I2C device.");
    let mut display: Ssd1306Display = Builder::new()
        .with_i2c_addr(SSD1306_ADDR)
        .size(DisplaySize::Display128x64)
        .connect_i2c(i2c_dev)
        .into();
    display.init().expect("Failed to initialize the display");
    display.clear();
    display.flush().unwrap();

    for index in 0..BMPS.len() {
        display.clear();
        let raw_image: ImageRaw<BinaryColor> = ImageRaw::new(&BMPS[index].image_data(), 128, 64);
        let image: Image<ImageRaw<BinaryColor>, BinaryColor> = Image::new(&raw_image, Point::zero());
        image.draw(&mut display).unwrap();
        display.flush().unwrap();
        // sleep(Duration::from_millis(33));
    }
}
