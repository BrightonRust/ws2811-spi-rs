//! # Use ws2811 leds via spi
//!
//! FIXME this is very preliminary for testing only FIXME
//!
//! - For usage with `smart-leds`
//! - Implements the `SmartLedsWrite` trait
//!
//! Needs a type implementing the `spi::FullDuplex` trait.
//!
//! References - Worldsemi WS2811 datasheet v1.4:
//! http://www.world-semi.com/DownLoadFile/129
//!
//! This uses data output on the spi bus MOSI line to create a data stream
//! which satisfies the timing requirements of the WS2811
//!
//! The WS2811 encodes a zero as:
//! a high logic state for 220-380ns, followed by:
//! a low logic state for 580-1000ns.
//!
//! The WS2811 encodes a one as:
//! a high logic state for 580-1000ns, followed by:
//! a low logic state for 580-1000ns.
//!
//! To satisfy these constraits, we select a 3MHz SPI bus speed, this results
//! in:
//! a single bit time of 333ns,
//! a double bit time of 666ns, and
//! a three bit time of 1000ns.
//!
//! Therefore we can encode each ws2811 bit as four spi bits,
//! we can encode a zero as: b1000 (333 ns high, 1000ns low).
//! we can encode a one as: b1100 (666 ns high, 666ns low).
//!
//! The spi peripheral should run at between 3MHz and 3.44MHz
//! below 3MHz the low portion of the zero bit send goes over 1000ns,
//! above 3.44MHz both portions of the one bit send go below 580ns,
//!
#![no_std]

extern crate embedded_hal as hal;

pub mod prerendered;

use hal::spi::{FullDuplex, Mode, Phase, Polarity};

use smart_leds_trait::{SmartLedsWrite, RGB8};

use nb;
use nb::block;

/// SPI mode that can be used for this crate
///
/// Provided for convenience
/// Doesn't really matter
pub const MODE: Mode = Mode {
    polarity: Polarity::IdleLow,
    phase: Phase::CaptureOnFirstTransition,
};

pub struct Ws2811<SPI> {
    spi: SPI,
}

impl<SPI, E> Ws2811<SPI>
where
    SPI: FullDuplex<u8, Error = E>,
{
    /// The SPI bus should run with 3 Mhz, otherwise this won't work.
    ///
    /// You may need to look at the datasheet and your own hal to verify this.
    ///
    /// Please ensure that the mcu is pretty fast, otherwise weird timing
    /// issues will occur
    pub fn new(spi: SPI) -> Ws2811<SPI> {
        Self { spi }
    }

    /// Write a single byte for ws2811 devices
    fn write_byte(&mut self, mut data: u8) -> Result<(), E> {
        let mut serial_bits: u16 = 0;
        for _ in 0..=3 {
            //let bit = data & 0b10000000;
            let ws2811_nibble = if (data & 0b10000000) == 0b10000000 { 0b1100 } else { 0b1000 };
            serial_bits = ws2811_nibble | (serial_bits << 4);
            data <<= 1;
        }
        block!(self.spi.send((serial_bits) as u8))?;
        self.spi.read().ok();
        block!(self.spi.send((serial_bits >> 8) as u8))?;
        self.spi.read().ok();
        // Split this up to have a bit more lenient timing
        for _ in 4..=7 {
            let ws2811_nibble = if (data & 0b10000000) == 0b10000000 { 0b1100 } else { 0b1000 };
            serial_bits = ws2811_nibble | (serial_bits << 4);
            data <<= 1;
        } 
        block!(self.spi.send((serial_bits) as u8))?;
        self.spi.read().ok();
        block!(self.spi.send((serial_bits >> 8) as u8))?;
        self.spi.read().ok();
        Ok(())
    }

    fn flush(&mut self) -> Result<(), E> {
        for _ in 0..20 {
            block!(self.spi.send(0))?;
            self.spi.read().ok();
        }
        Ok(())
    }
}

impl<SPI, E> SmartLedsWrite for Ws2811<SPI>
where
    SPI: FullDuplex<u8, Error = E>,
{
    type Error = E;
    type Color = RGB8;
    /// Write all the items of an iterator to a ws2811 strip
    fn write<T, I>(&mut self, iterator: T) -> Result<(), E>
    where
        T: Iterator<Item = I>,
        I: Into<Self::Color>,
    {
        if cfg!(feature = "mosi_idle_high") {
            self.flush()?;
        }

        for item in iterator {
            let item = item.into();
            self.write_byte(item.r)?;
            self.write_byte(item.g)?;
            self.write_byte(item.b)?;
        }
        self.flush()?;
        Ok(())
    }
}
