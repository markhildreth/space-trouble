use feather_m0 as hal;
use hal::delay::Delay;
use embedded_hal::blocking::i2c::Write;
use embedded_hal::blocking::delay::{DelayUs, DelayMs};

#[derive(PartialEq, Eq)]
pub enum Backlight {
    OFF,
    ON
}

const DISPLAY_CONTROL: u8 = 0b00001000;
bitflags! {
    pub struct DisplayControls: u8 {
        const DISPLAY = 0b100;
        const CURSOR  = 0b010;
        const BLINK   = 0b001;
    }
}

pub struct I2CLCD {
    addr: u8,
    backlight: Backlight
}

// Implemented for the SPLC780D I2C LCD chip.
impl I2CLCD {
    pub fn new(addr: u8) -> I2CLCD {
        I2CLCD { addr, backlight: Backlight::OFF }
    }

    pub fn initialize<I2C>(&self, i2c: &mut I2C, delay: &mut Delay)
        where I2C: Write
    {

        // Manually reset the SPLC780D circuit to a 4-bit interface
        i2c.write(self.addr, &[0u8]);
        delay.delay_ms(50u8);
        self.write_nibbles(i2c, delay, 0x03, false);
        delay.delay_ms(10u8);
        self.write_nibbles(i2c, delay, 0x03, false);
        delay.delay_ms(1u8);
        self.write_nibbles(i2c, delay, 0x03, false);

        // Set 4-bit mode (function set)
        self.write_nibbles(i2c, delay, 0b0010, false);

        // Full Function Set
        self.write_nibbles(i2c, delay, 0b0010, false);
        self.write_nibbles(i2c, delay, 0b1000, false);

        // Display off
        self.write_nibbles(i2c, delay, 0, false);
        self.write_nibbles(i2c, delay, 0b1000, false);

        // Display Clear
        self.write_nibbles(i2c, delay, 0, false);
        self.write_nibbles(i2c, delay, 0b0001, false);

        // Entry mode set
        self.write_nibbles(i2c, delay, 0, false);
        self.write_nibbles(i2c, delay, 0b0111, false);
        delay.delay_ms(1u8);
    }

    pub fn clear<I2C>(&self, i2c: &mut I2C, delay: &mut Delay)
        where I2C: Write
    {
        self.write_nibbles(i2c, delay, 0, false);
        self.write_nibbles(i2c, delay, 0b0011, false);
        delay.delay_ms(2u8);
    }

    pub fn backlight<I2C>(&mut self, i2c: &mut I2C, delay: &mut Delay, backlight: Backlight)
        where I2C: Write
    {
        self.backlight = backlight;
        // Send a noop just to ensure that the backlight flag is changed 
        self.write_nibbles(i2c, delay, 0, false);
        self.write_nibbles(i2c, delay, 0, false);
    }

    pub fn display_control<I2C>(&self, i2c: &mut I2C, delay: &mut Delay, controls: DisplayControls)
        where I2C: Write
    {
        self.write_nibbles(i2c, delay, 0, false);
        self.write_nibbles(i2c, delay, DISPLAY_CONTROL | controls.bits(), false);
    }

    fn write_nibbles<I2C>(&self, i2c: &mut I2C, delay: &mut Delay, val: u8, command: bool)
        where I2C: Write
    {
        let e_flag = 0b100;
        let backlight_flag = (self.backlight == Backlight::ON) as u8 * 0b1000;
        let command_flag = command as u8 * 0b1;

        i2c.write(self.addr, &[(val << 4) | e_flag | backlight_flag | command_flag]);
        delay.delay_us(1u8);
        i2c.write(self.addr, &[(val << 4) & !e_flag | backlight_flag | command_flag]);
        delay.delay_us(50u8);
    }

}

