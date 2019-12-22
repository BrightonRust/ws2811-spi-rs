# Ws2811 driver for embedded-hal spi traits

Preliminary proof of concept, adapted from [ws2812-spi-rs](https://github.com/smart-leds-rs/ws2812-spi-rs)

For usage with the [smart-leds](https://github.com/smart-leds-rs/smart-leds)
crate.

An embedded-hal driver for ws2811 leds using spi as the timing provider. 

  Your spi peripheral has to run at 3MHz & the SPI data is created on-the-fly. 
  This means that your core has to be reasonably fast (~48 MHz).

  **Important** Because the SPI data is computed on the fly, and sent a byte at
  a time, you may need to build with `--release` in order to generate a valid
  data stream.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
