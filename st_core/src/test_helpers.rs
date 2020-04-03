use crate::device::LCD;
use core::fmt;
use core::fmt::Write;
use std::cell::RefCell;
use std::rc::Rc;

struct FakeScreen {
    rows: [String; 4],
}

impl FakeScreen {
    fn new() -> FakeScreen {
        FakeScreen {
            rows: [
                " ".repeat(20),
                " ".repeat(20),
                " ".repeat(20),
                " ".repeat(20),
            ],
        }
    }

    fn write_str(&mut self, pos: (u8, u8), s: &str) {
        let row = pos.0 as usize;
        let col = pos.1 as usize;
        let row = &mut self.rows[row];
        row.replace_range(col..col + s.len(), s);
    }
}

impl fmt::Debug for FakeScreen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\n").unwrap();
        self.rows.iter().for_each(|x| {
            f.write_fmt(format_args!("{:?}\n", x)).unwrap();
            ()
        });
        Ok(())
    }
}

impl PartialEq for FakeScreen {
    fn eq(&self, rhs: &Self) -> bool {
        self.rows == rhs.rows
    }
}

pub(crate) struct TestOutput {
    screen: Rc<RefCell<FakeScreen>>,
}

impl TestOutput {
    pub(crate) fn assert(&self, rows: [&str; 4]) {
        let rows: [String; 4] = [
            rows[0].to_string(),
            rows[1].to_string(),
            rows[2].to_string(),
            rows[3].to_string(),
        ];
        let expected = FakeScreen { rows };
        assert_eq!(expected, *self.screen.borrow());
    }
}

pub(crate) struct TestLCD {
    screen: Rc<RefCell<FakeScreen>>,
    cursor_pos: (u8, u8),
}

impl TestLCD {
    pub(crate) fn new() -> TestLCD {
        TestLCD {
            screen: Rc::new(RefCell::new(FakeScreen::new())),
            cursor_pos: (0, 0),
        }
    }

    pub(crate) fn split(self) -> (Self, TestOutput) {
        let screen = TestOutput {
            screen: self.screen.clone(),
        };
        (self, screen)
    }
}

impl LCD for TestLCD {
    fn set_cursor_pos(&mut self, row: u8, col: u8) {
        self.cursor_pos = (row, col);
    }
}

impl Write for TestLCD {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        let mut screen = self.screen.borrow_mut();
        screen.write_str(self.cursor_pos, s);
        self.cursor_pos = (self.cursor_pos.0, self.cursor_pos.1 + s.len() as u8);
        Ok(())
    }
}
