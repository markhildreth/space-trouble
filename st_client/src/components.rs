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
use crate::{ComponentsDef, Panel, LCD};
use st_data::ClientMessageProducer;

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
