# dac5578 &emsp;

*Texas Instruments DAC5578 Driver for Rust Embedded HAL*
This is a driver crate for embedded Rust. It's built on top of the Rust
[embedded HAL](https://github.com/rust-embedded/embedded-hal)
It supports sending commands to a TI DAC5578 over I2C.

The driver can be initialized by calling create and passing it an I2C interface.
The device address (set by ADDR0) also needs to be specified.
It can be set by pulling the ADDR0 on the device high/low or floating.

```
# use embedded_hal_mock::i2c::Mock;
# use dac5578::*;
# let mut i2c = Mock::new(&[]);
let mut dac = DAC5578::new(i2c, Address::PinLow);
```

To set the dac output for channel A:
```
# use embedded_hal_mock::i2c::{Mock, Transaction};
# use dac5578::*;
# let mut i2c = Mock::new(&[Transaction::write(98, vec![0x40, 0xff, 0xf0]),]);
# let mut dac = DAC5578::new(i2c, Address::PinLow);
dac.write_channel(Channel::A, 128);
```

## More information
- [DAC5578 datasheet](https://www.ti.com/lit/ds/symlink/dac5578.pdf?ts=1621340690413&ref_url=https%253A%252F%252Fwww.ti.com%252Fproduct%252FDAC5578)
- [API documentation](https://docs.rs/dac5578/)
- [Github repository](https://github.com/chmanie/dac5578)
- [Crates.io](https://crates.io/crates/dac5578)

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
