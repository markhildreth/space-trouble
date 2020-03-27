// There is a lot of stuff I'm not entirely sure is idiomatic rust. Basically, I want
// to be able to pass a bunch of components (e.g., a type representing the LCD, Panel,
// etc.) as a struct, but because all of these objects are defined as traits, there
// would be a ton of generics spread out everywhere.
//
// The solution I found was to put all of the type definitions into one trait with type
// associations, and then the function just needs to be generic on that one type.
//
// Of course, getting everything to work involves a bunch of work, such as creating
// a zero-sized struct to represent the implementation of the trait that contains
// only type associations.

use crate::Client;
use crate::{ComponentDef, Panel, LCD};
use st_data::ClientMessageProducer;

pub struct ComponentDefImpl<TPanel, TLCD> {
    _p1: core::marker::PhantomData<TPanel>,
    _p2: core::marker::PhantomData<TLCD>,
}

impl<TPanel: Panel, TLCD: LCD> ComponentDef for ComponentDefImpl<TPanel, TLCD> {
    type Panel = TPanel;
    type LCD = TLCD;
}

pub(crate) struct Components<'a, C: ComponentDef> {
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

impl<'a, TPanel: Panel, TLCD: LCD, CDef: ComponentDef<Panel = TPanel, LCD = TLCD>>
    core::convert::From<ClientComponents<'a, TPanel, TLCD>> for Components<'a, CDef>
{
    fn from(c: ClientComponents<'a, TPanel, TLCD>) -> Components<'a, CDef> {
        Components {
            lcd: c.lcd,
            panel: c.panel,
            producer: c.producer,
        }
    }
}

pub fn build<'a, TPanel, TLCD>(
    components: ClientComponents<'a, TPanel, TLCD>,
) -> Client<'a, ComponentDefImpl<TPanel, TLCD>>
where
    TLCD: LCD,
    TPanel: Panel,
{
    Client::new(components.into())
}
