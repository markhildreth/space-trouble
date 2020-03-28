#![no_std]
mod client;
mod game_screen;
mod states;
mod strings;
mod timing;

use core::fmt::Write;
pub use game_screen::GameScreen;
use st_data::time::*;
use st_data::ClientMessageProducer;

pub use client::Client;

pub trait Panel {
    fn update(&mut self, producer: &mut ClientMessageProducer, now: Instant);
}

pub trait LCD: Sized + Write {
    fn set_cursor_pos(&mut self, row: u8, col: u8);
}

trait ComponentsDef
where
    Self::Panel: Panel,
    Self::LCD: LCD,
{
    type Panel;
    type LCD;
}

pub struct ComponentsDefImpl<TPanel: Panel, TLCD: LCD> {
    _p1: core::marker::PhantomData<TPanel>,
    _p2: core::marker::PhantomData<TLCD>,
}

impl<TPanel: Panel, TLCD: LCD> ComponentsDef for ComponentsDefImpl<TPanel, TLCD> {
    type Panel = TPanel;
    type LCD = TLCD;
}

pub(crate) struct Components<'a, C: ComponentsDef> {
    pub producer: ClientMessageProducer<'a>,
    pub panel: C::Panel,
    pub lcd: C::LCD,
}

pub struct ClientComponents<'a, TPanel, TLCD>
where
    TPanel: Panel,
    TLCD: LCD,
{
    pub producer: ClientMessageProducer<'a>,
    pub panel: TPanel,
    pub lcd: TLCD,
}
