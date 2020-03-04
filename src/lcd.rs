use feather_m0 as hal;
use hal::delay::Delay;
use embedded_hal::blocking::i2c::Write;
use embedded_hal::blocking::delay::{DelayUs, DelayMs};

pub struct I2CLCD {
    addr: u8,
}

// Implemented for the SPLC780D I2C LCD chip.
impl I2CLCD {
    pub fn new(addr: u8) -> I2CLCD {
        I2CLCD { addr }
    }

    pub fn initialize<I2C>(&self, i2c: &mut I2C, delay: &mut Delay)
        where I2C: Write
    {

        // Manually reset the SPLC780D circuit to a 4-bit interface
        i2c.write(self.addr, &[0u8]);
        delay.delay_ms(50u8);
        self.write_nibbles(i2c, delay, 0x03, false, false);
        delay.delay_ms(10u8);
        self.write_nibbles(i2c, delay, 0x03, false, false);
        delay.delay_ms(1u8);
        self.write_nibbles(i2c, delay, 0x03, false, false);

        // Set 4-bit mode (function set)
        self.write_nibbles(i2c, delay, 0b0010, false, false);

        // Full Function Set
        self.write_nibbles(i2c, delay, 0b0010, false, false);
        self.write_nibbles(i2c, delay, 0b1000, false, false);

        // Display off
        self.write_nibbles(i2c, delay, 0, false, false);
        self.write_nibbles(i2c, delay, 0b1000, false, false);

        // Display Clear
        self.write_nibbles(i2c, delay, 0, false, false);
        self.write_nibbles(i2c, delay, 0b0001, false, false);

        // Entry mode set
        self.write_nibbles(i2c, delay, 0, false, false);
        self.write_nibbles(i2c, delay, 0b0111, false, false);
        delay.delay_ms(1u8);
    }

    pub fn clear<I2C>(&self, i2c: &mut I2C, delay: &mut Delay)
        where I2C: Write
    {
        self.write_nibbles(i2c, delay, 0, true, false);
        self.write_nibbles(i2c, delay, 0b0011, true, false);
        delay.delay_ms(2u8);
    }

    pub fn display_control<I2C>(&self, i2c: &mut I2C, delay: &mut Delay)
        where I2C: Write
    {
        self.write_nibbles(i2c, delay, 0, true, false);
        self.write_nibbles(i2c, delay, 0b1111, true, false);
        delay.delay_ms(1u8);
    }

    fn write_nibbles<I2C>(&self, i2c: &mut I2C, delay: &mut Delay, val: u8, backlight: bool, command: bool)
        where I2C: Write
    {
        i2c.write(self.addr, &[(val << 4) | 0b100 | (backlight as u8 * 0b1000) | (command as u8 * 0b1)]);
        delay.delay_us(1u8);
        i2c.write(self.addr, &[(val << 4) & !0b100 | (backlight as u8 * 0b1000) | (command as u8 * 0b1)]);
        delay.delay_us(50u8);
    }

}

