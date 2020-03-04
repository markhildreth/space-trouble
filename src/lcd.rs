use feather_m0 as hal;
use hal::delay::Delay;
use embedded_hal::blocking::i2c::Write;
use embedded_hal::blocking::delay::{DelayUs, DelayMs};

#[derive(PartialEq, Eq)]
pub enum Backlight {
    OFF,
    ON
}

const CLEAR_DISPLAY: u8   = 0b00000001;
const DISPLAY_CONTROL: u8 = 0b00001000;

bitflags! {
    pub struct DisplayControls: u8 {
        const DISPLAY = 0b100;
        const CURSOR  = 0b010;
        const BLINK   = 0b001;
    }
}

bitflags! {
    pub struct NibbleFlags: u8 {
        const BACKLIGHT = 0b1000;
        const E         = 0b0100;
        const RW        = 0b0010;
        const RS        = 0b0001;
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
        self.pulse_nibble(i2c, delay, 0x03);
        delay.delay_ms(10u8);
        self.pulse_nibble(i2c, delay, 0x03);
        delay.delay_ms(1u8);
        self.pulse_nibble(i2c, delay, 0x03);

        // Set 4-bit mode (function set)
        self.pulse_nibble(i2c, delay, 0b0010);

        // Full Function Set
        self.send(i2c, delay, 0b00101000, false);

        // Display off
        self.display_control(i2c, delay, DisplayControls::empty());

        // Display Clear
        self.clear(i2c, delay);

        // Entry mode set
        self.write_nibbles(i2c, delay, 0, false);
        self.write_nibbles(i2c, delay, 0b0111, false);
        delay.delay_ms(1u8);
    }

    pub fn clear<I2C>(&self, i2c: &mut I2C, delay: &mut Delay)
        where I2C: Write
    {
        self.send(i2c, delay, CLEAR_DISPLAY, false);
        delay.delay_ms(2u8);
    }

    pub fn backlight<I2C>(&mut self, i2c: &mut I2C, delay: &mut Delay, backlight: Backlight)
        where I2C: Write
    {
        self.backlight = backlight;
        // Send a noop just to ensure that the backlight flag is changed 
        self.send(i2c, delay, 0, false);
    }

    pub fn display_control<I2C>(&self, i2c: &mut I2C, delay: &mut Delay, controls: DisplayControls)
        where I2C: Write
    {
        self.send(i2c, delay, DISPLAY_CONTROL | controls.bits(), false);
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

    fn send<I2C>(&self, i2c: &mut I2C, delay: &mut Delay, data: u8, command: bool)
        where I2C: Write
    {
        let mut nibble_flags = NibbleFlags::empty();

        if self.backlight == Backlight::ON {
            nibble_flags |= NibbleFlags::BACKLIGHT;
        }

        if command {
            nibble_flags |= NibbleFlags::RS;
        }

        let first_nibble_data = data & 0xf;
        let second_nibble_data = (data & 0x0f) << 4;
        self.pulse_nibble(i2c, delay, first_nibble_data | nibble_flags.bits());
        self.pulse_nibble(i2c, delay, second_nibble_data | nibble_flags.bits());
    }

    fn pulse_nibble<I2C>(&self, i2c: &mut I2C, delay: &mut Delay, data: u8)
        where I2C: Write
    {
        i2c.write(self.addr, &[data | NibbleFlags::E.bits()]);
        delay.delay_us(1u8);
        i2c.write(self.addr, &[data]);
        delay.delay_us(50u8);
    }
}

