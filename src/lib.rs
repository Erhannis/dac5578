//! *Texas Instruments DAC5578 Driver for Rust Embedded HAL*
//! This is a driver crate for embedded Rust. It's built on top of the Rust
//! [embedded HAL](https://github.com/rust-embedded/embedded-hal)
//! It supports sending commands to a TI DAC5578 over I2C.
//!
//! The driver can be initialized by calling create and passing it an I2C interface.
//! The device address (set by ADDR0) also needs to be specified.
//! It can be set by pulling the ADDR0 on the device high/low or floating.
//!
//! ```
//! # use embedded_hal_mock::i2c::Mock;
//! # use dac5578::*;
//! # let mut i2c = Mock::new(&[]);
//! let mut dac = DAC5578::new(i2c, Address::PinLow);
//! ```
//!
//! To set the dac output for channel A:
//! ```
//! # use embedded_hal_mock::i2c::{Mock, Transaction};
//! # use dac5578::*;
//! # let mut i2c = Mock::new(&[Transaction::write(98, vec![0x40, 0xff, 0xf0]),]);
//! # let mut dac = DAC5578::new(i2c, Address::PinLow);
//! dac.write_channel(Channel::A, 128);
//! ```
//!
//! ## More information
//! - [DAC5578 datasheet](https://www.ti.com/lit/ds/symlink/dac5578.pdf?ts=1621340690413&ref_url=https%253A%252F%252Fwww.ti.com%252Fproduct%252FDAC5578)
//! - [API documentation](https://docs.rs/dac5578/)
//! - [Github repository](https://github.com/chmanie/dac5578)
//! - [Crates.io](https://crates.io/crates/dac5578)
//!
#![no_std]
#![warn(missing_debug_implementations, missing_docs)]

use core::fmt::Debug;
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};

/// user_address can be set by pulling the ADDR0 pin high/low or leave it floating
#[derive(Debug)]
#[repr(u8)]
pub enum Address {
    /// ADDR0 is low
    PinLow = 0x48,
    /// ADDR0 is high
    PinHigh = 0x4a,
    /// ADDR0 is floating
    PinFloat = 0x4c,
}

/// Defines the output channel to set the voltage for
#[derive(Debug)]
#[repr(u8)]
pub enum Channel {
    /// DAC output channel A
    A,
    /// DAC output channel B
    B,
    /// DAC output channel C
    C,
    /// DAC output channel D
    D,
    /// DAC output channel E
    E,
    /// DAC output channel F
    F,
    /// DAC output channel G
    G,
    /// DAC output channel H
    H,
    /// Targets all DAC output channels
    All = 0xf,
}

impl From<u8> for Channel {
    fn from(index: u8) -> Self {
        match index {
            0 => Channel::A,
            1 => Channel::B,
            2 => Channel::C,
            3 => Channel::D,
            4 => Channel::E,
            5 => Channel::F,
            6 => Channel::G,
            7 => Channel::H,
            _ => panic!("Unkown channel number {}", index),
        }
    }
}

/// The type of the command to send for a Write Command
#[derive(Debug)]
#[repr(u8)]
pub enum WriteCommandType {
    /// Write to the channel's DAC input register
    WriteToChannel = 0x00,
    /// Selects DAC channel to be updated
    UpdateChannel = 0x10,
    /// Write to DAC input register for a channel and update channel DAC register
    WriteToChannelAndUpdate = 0x30,
    /// Write to Selected DAC Input Register and Update All DAC Registers (Global Software LDAC)
    WriteToChannelAndUpdateAll = 0x20,
}

/// The type of the command to send for a Read Command
#[derive(Debug)]
#[repr(u8)]
pub enum ReadCommandType {
    /// Read from the channel's DAC *actual* register (not input)
    ReadFromChannel = 0x10,
}

/// Two bit flags indicating the reset mode for the DAC5578
#[derive(Debug)]
#[repr(u8)]
pub enum ResetMode {
    /// Software reset (default). Same as power-on reset (POR).
    Por = 0b00,
    /// Software reset that sets device into High-Speed mode
    SetHighSpeed = 0b01,
    /// Software reset that maintains High-Speed mode state
    MaintainHighSpeed = 0b10,
}

/// DAC5578 driver. Wraps an I2C port to send commands to a DAC5578
#[derive(Debug)]
pub struct DAC5578<I2C>
where
    I2C: Read + Write + WriteRead, //CHECK I don't know whether we actually need WriteRead
{
    i2c: I2C,
    address: u8,
}

impl<I2C, E> DAC5578<I2C>
where
    I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
{
    /// Construct a new DAC5578 driver instance.
    /// i2c is the initialized i2c driver port to use, address depends on the state of the ADDR0 pin (see [`Address`])
    pub fn new(i2c: I2C, address: Address) -> Self {
        DAC5578 {
            i2c,
            address: address as u8,
        }
    }

    /// Write to the channel's DAC input register
    pub fn write(&mut self, channel: Channel, data: u16) -> Result<(), E> {
        let bytes = Self::encode_write_command(WriteCommandType::WriteToChannel, channel as u8, data);
        self.i2c.write(self.address, &bytes)
    }

    //RAINY Also permit read input registers?
    /// Read the channel's DAC *actual* register (not input)
    pub fn read(&mut self, channel: Channel) -> Result<u16, E> {
        let bytes = Self::encode_read_command(ReadCommandType::ReadFromChannel, channel as u8);
        let mut response: [u8; 2] = [0, 0];
        self.i2c.write_read(self.address, &bytes, &mut response)?;
        Ok(u16::from_be_bytes(response))
    }
  
    /// Selects DAC channel to be updated
    pub fn update(&mut self, channel: Channel, data: u16) -> Result<(), E> {
        let bytes = Self::encode_write_command(WriteCommandType::UpdateChannel, channel as u8, data);
        self.i2c.write(self.address, &bytes)
    }

    /// Write to DAC input register for a channel and update channel DAC register
    pub fn write_and_update(&mut self, channel: Channel, data: u16) -> Result<(), E> {
        let bytes = Self::encode_write_command(WriteCommandType::WriteToChannelAndUpdate, channel as u8, data);
        self.i2c.write(self.address, &bytes)
    }

    /// Write to Selected DAC Input Register and Update All DAC Registers (Global Software LDAC)
    pub fn write_and_update_all(&mut self, channel: Channel, data: u16) -> Result<(), E> {
        let bytes =
            Self::encode_write_command(WriteCommandType::WriteToChannelAndUpdateAll, channel as u8, data);
        self.i2c.write(self.address, &bytes)
    }

    /// Perform a software reset using the selected mode
    pub fn reset(&mut self, mode: ResetMode) -> Result<(), E> {
        let bytes = [0x70, mode as u8, 0];
        self.i2c.write(self.address, &bytes)
    }

    /// Send a wake-up command over the I2C bus.
    /// WARNING: This is a general call command and can wake-up other devices on the bus as well.
    pub fn wake_up_all(&mut self) -> Result<(), E> {
        self.i2c.write(0x00, &[0x06u8])?;
        Ok(())
    }

    /// Send a reset command on the I2C bus.
    /// WARNING: This is a general call command and can reset other devices on the bus as well.
    pub fn reset_all(&mut self) -> Result<(), E> {
        self.i2c.write(0x00, &[0x09u8])?;
        Ok(())
    }

    /// Destroy the DAC5578 driver, return the wrapped I2C
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Encode command type, channel and data into a three byte command
    fn encode_write_command(command: WriteCommandType, access: u8, value: u16) -> [u8; 3] {
        let value_bytes = value.to_be_bytes();
        [command as u8 | access, value_bytes[0], value_bytes[1]]
    }

    /// Encode command type and channel into a one-byte command
    fn encode_read_command(command: ReadCommandType, access: u8) -> [u8; 1] {
        [command as u8 | access]
    }
}
