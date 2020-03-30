#![no_std]
mod client;
mod game_screen;
mod states;
mod strings;
mod timing;

use core::fmt::Write;
pub use game_screen::GameScreen;
use st_common::time::*;
use st_common::ClientMessageProducer;

pub use client::Client;

pub trait Panel {
    fn update(&mut self, producer: &mut ClientMessageProducer, now: Instant);
}

pub trait LCD: Sized + Write {
    fn set_cursor_pos(&mut self, row: u8, col: u8);
}

pub trait ComponentsDef
where
    Self::Panel: Panel,
    Self::LCD: LCD,
{
    type Panel;
    type LCD;
}

pub struct Components<'a, CD: ComponentsDef> {
    pub(crate) producer: ClientMessageProducer<'a>,
    pub(crate) panel: CD::Panel,
    pub(crate) lcd: CD::LCD,
}

impl<'a, TPanel, TLCD, CD> Components<'a, CD>
where
    CD: ComponentsDef<Panel = TPanel, LCD = TLCD>,
    TPanel: Panel,
    TLCD: LCD,
{
    pub fn new(producer: ClientMessageProducer<'a>, panel: TPanel, lcd: TLCD) -> Self {
        Components {
            producer,
            panel,
            lcd,
        }
    }
}

pub struct ComponentsDefImpl<TPanel: Panel, TLCD: LCD> {
    _p1: core::marker::PhantomData<TPanel>,
    _p2: core::marker::PhantomData<TLCD>,
}

impl<TPanel: Panel, TLCD: LCD> ComponentsDef for ComponentsDefImpl<TPanel, TLCD> {
    type Panel = TPanel;
    type LCD = TLCD;
}
