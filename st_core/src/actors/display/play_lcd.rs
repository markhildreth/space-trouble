use crate::device::LCD;

const BLANK_LINE: &str = "                    ";
const BLANK: char = ' ';

// replace it with an asterisk for testing purposes.
#[cfg(not(test))]
const BLOCK: char = 0xff as char;
#[cfg(test)]
const BLOCK: char = '*';

pub(super) struct PlayLCD<T: LCD> {
    lcd: T,
    current_blocks: Option<u8>,
}

impl<T: LCD> PlayLCD<T> {
    pub(super) fn new(mut lcd: T) -> PlayLCD<T> {
        lcd.clear();
        lcd.set_cursor_pos(0, 0);
        lcd.write_str("0 km      Hull: 100%").unwrap();
        PlayLCD {
            lcd,
            current_blocks: None,
        }
    }

    pub(super) fn update_ship_hull_health(&mut self, new_health: u8) {
        self.lcd.set_cursor_pos(0, 16);
        self.lcd
            .write_fmt(format_args!("{: >3}%", new_health))
            .unwrap();
    }

    pub(super) fn update_ship_distance(&mut self, new_distance: u32) {
        self.lcd.set_cursor_pos(0, 0);
        self.lcd
            .write_fmt(format_args!("{} km", new_distance))
            .unwrap();
    }

    pub(super) fn display_directive(&mut self, text1: &str, text2: &str) {
        self.lcd.set_cursor_pos(2, 0);
        self.lcd.write_str(text1).unwrap();
        self.lcd.set_cursor_pos(3, 0);
        self.lcd.write_str(text2).unwrap();
        self.update_countdown(20);
    }

    pub(super) fn update_countdown(&mut self, blocks: u8) {
        // Performance optimization: since we might be constantly
        // attempting to rerender the asterisks, keep a local cache
        // of what is currently displayed so we don't need to utilize
        // the slow display interface too often.
        if self.current_blocks == Some(blocks) {
            return;
        }
        self.current_blocks = Some(blocks);

        self.lcd.set_cursor_pos(1, 0);
        for _ in 0..blocks {
            self.lcd.write_char(BLOCK).unwrap();
        }

        for _ in blocks..20 {
            self.lcd.write_char(BLANK).unwrap();
        }
    }

    pub(super) fn clear_directive(&mut self) {
        self.lcd.set_cursor_pos(2, 0);
        self.lcd.write_str(BLANK_LINE).unwrap();
        self.lcd.set_cursor_pos(3, 0);
        self.lcd.write_str(BLANK_LINE).unwrap();
    }

    pub(super) fn unwrap(self) -> T {
        self.lcd
    }
}

#[cfg(test)]
mod test_play_lcd {
    use super::*;
    use crate::test_helpers::TestLCD;

    #[test]
    fn starts_clean() {
        let (lcd, screen) = TestLCD::new().split();
        PlayLCD::new(lcd);
        screen.assert([
            "0 km      Hull: 100%",
            "                    ",
            "                    ",
            "                    ",
        ]);
    }

    #[test]
    fn can_display_health() {
        let (lcd, screen) = TestLCD::new().split();
        let mut play_lcd = PlayLCD::new(lcd);
        play_lcd.update_ship_hull_health(98);
        screen.assert([
            "0 km      Hull:  98%",
            "                    ",
            "                    ",
            "                    ",
        ]);
        play_lcd.update_ship_hull_health(2);
        screen.assert([
            "0 km      Hull:   2%",
            "                    ",
            "                    ",
            "                    ",
        ]);
    }

    #[test]
    fn can_display_ship_distance() {
        let (lcd, screen) = TestLCD::new().split();
        let mut play_lcd = PlayLCD::new(lcd);
        play_lcd.update_ship_distance(9);
        screen.assert([
            "9 km      Hull: 100%",
            "                    ",
            "                    ",
            "                    ",
        ]);

        play_lcd.update_ship_distance(19);
        screen.assert([
            "19 km     Hull: 100%",
            "                    ",
            "                    ",
            "                    ",
        ]);

        play_lcd.update_ship_distance(199);
        screen.assert([
            "199 km    Hull: 100%",
            "                    ",
            "                    ",
            "                    ",
        ]);

        play_lcd.update_ship_distance(1999);
        screen.assert([
            "1999 km   Hull: 100%",
            "                    ",
            "                    ",
            "                    ",
        ]);

        play_lcd.update_ship_distance(19999);
        screen.assert([
            "19999 km  Hull: 100%",
            "                    ",
            "                    ",
            "                    ",
        ]);
    }

    #[test]
    fn can_display_directive_and_countdown() {
        let (lcd, screen) = TestLCD::new().split();
        let mut play_lcd = PlayLCD::new(lcd);

        play_lcd.display_directive("        Test        ", "        Stuff      ");
        screen.assert([
            "0 km      Hull: 100%",
            "********************",
            "        Test        ",
            "        Stuff       ",
        ]);

        play_lcd.update_countdown(20);
        screen.assert([
            "0 km      Hull: 100%",
            "********************",
            "        Test        ",
            "        Stuff       ",
        ]);

        play_lcd.update_countdown(19);
        screen.assert([
            "0 km      Hull: 100%",
            "******************* ",
            "        Test        ",
            "        Stuff       ",
        ]);

        play_lcd.update_countdown(10);
        screen.assert([
            "0 km      Hull: 100%",
            "**********          ",
            "        Test        ",
            "        Stuff       ",
        ]);

        play_lcd.update_countdown(1);
        screen.assert([
            "0 km      Hull: 100%",
            "*                   ",
            "        Test        ",
            "        Stuff       ",
        ]);

        play_lcd.update_countdown(0);
        screen.assert([
            "0 km      Hull: 100%",
            "                    ",
            "        Test        ",
            "        Stuff       ",
        ]);

        play_lcd.clear_directive();
        screen.assert([
            "0 km      Hull: 100%",
            "                    ",
            "                    ",
            "                    ",
        ]);
    }
}
