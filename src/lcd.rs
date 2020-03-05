use feather_m0 as hal;
use hal::delay::Delay;
use embedded_hal::blocking::i2c::Write;
use embedded_hal::blocking::delay::{DelayUs, DelayMs};

const DISPLAY_ADDRESS_ROWS: [u8; 4] = [0, 0x40, 0x14, 0x54];

pub struct DisplayAddress {
    address: u8
}

impl DisplayAddress {
    pub fn raw(address: u8) -> DisplayAddress {
        DisplayAddress { address }
    }

    pub fn from_row_col(row: u8, col: u8) -> DisplayAddress {
        DisplayAddress {
            address: DISPLAY_ADDRESS_ROWS[row as usize] + col
        }
    }

    fn bits(&self) -> u8 {
        self.address
    }
}

#[derive(PartialEq, Eq)]
pub enum Backlight {
    OFF,
    ON
}

const CLEAR_DISPLAY: u8           = 0b00000001;
const RETURN_HOME: u8             = 0b00000010;
const ENTRY_MODE_SET: u8          = 0b00000100;
const DISPLAY_CONTROL: u8         = 0b00001000;
const CURSOR_OR_DISPLAY_SHIFT: u8 = 0b00010000;
const SET_DISPLAY_ADDRESS: u8     = 0b10000000;

pub enum EntryModes {
    CursorLeft  = 0b00,
    CursorRight = 0b10,
    ShiftLeft   = 0b01,
    ShiftRight  = 0b11
}

pub enum CursorShifts {
    Left  = 0b0000,
    Right = 0b0100 
}

pub enum DisplayShifts {
    Left   = 0b1000,
    Right  = 0b1100 
}

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
        delay.delay_ms(50u8);
        i2c.write(self.addr, &[0u8]);
        delay.delay_ms(1000u16);

        self.pulse_nibble(i2c, delay, 0x03 << 4);
        delay.delay_us(4500u16);
        self.pulse_nibble(i2c, delay, 0x03 << 4);
        delay.delay_us(4500u16);
        self.pulse_nibble(i2c, delay, 0x03 << 4);
        delay.delay_us(150u8);

        // Set 4-bit mode (function set)
        self.pulse_nibble(i2c, delay, 0b0010 << 4);

        // Full Function Set
        self.send(i2c, delay, 0b00101000, false);

        // Display off
        self.display_control(i2c, delay, DisplayControls::empty());

        // Display Clear
        self.clear(i2c, delay);

        // Entry mode set
        self.set_entry_mode(i2c, delay, EntryModes::CursorRight);
        delay.delay_ms(1u8);
    }

    pub fn backlight<I2C>(&mut self, i2c: &mut I2C, delay: &mut Delay, backlight: Backlight)
        where I2C: Write
    {
        self.backlight = backlight;
        // Send a noop just to ensure that the backlight flag is changed 
        self.send(i2c, delay, 0, false);
    }

    pub fn clear<I2C>(&self, i2c: &mut I2C, delay: &mut Delay)
        where I2C: Write
    {
        self.send(i2c, delay, CLEAR_DISPLAY, false);
        delay.delay_ms(2u8);
    }

    pub fn return_home<I2C>(&self, i2c: &mut I2C, delay: &mut Delay)
        where I2C: Write
    {
        self.send(i2c, delay, RETURN_HOME, false);
        delay.delay_ms(2u8);
    }

    pub fn set_entry_mode<I2C>(&self, i2c: &mut I2C, delay: &mut Delay, entry_mode: EntryModes)
        where I2C: Write
    {
        self.send(i2c, delay, ENTRY_MODE_SET | (entry_mode as u8), false);
    }

    pub fn display_control<I2C>(&self, i2c: &mut I2C, delay: &mut Delay, controls: DisplayControls)
        where I2C: Write
    {
        self.send(i2c, delay, DISPLAY_CONTROL | controls.bits(), false);
    }

    pub fn shift_cursor<I2C>(&self, i2c: &mut I2C, delay: &mut Delay, cursor_shift: CursorShifts)
        where I2C: Write
    {
        self.send(i2c, delay, CURSOR_OR_DISPLAY_SHIFT | (cursor_shift as u8), false);
    }

    pub fn shift_display<I2C>(&self, i2c: &mut I2C, delay: &mut Delay, display_shift: DisplayShifts)
        where I2C: Write
    {
        self.send(i2c, delay, CURSOR_OR_DISPLAY_SHIFT | (display_shift as u8), false);
    }

    pub fn set_display_address<I2C>(&self, i2c: &mut I2C, delay: &mut Delay, address: DisplayAddress)
        where I2C: Write
    {
        self.send(i2c, delay, SET_DISPLAY_ADDRESS | address.bits(), false);
    }

    pub fn write_to_ram<I2C>(&self, i2c: &mut I2C, delay: &mut Delay, data: u8)
        where I2C: Write
    {
        self.send(i2c, delay, data, true);
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

        let first_nibble_data = data & 0xf0;
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
