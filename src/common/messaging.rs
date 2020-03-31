pub trait Event: core::fmt::Debug {}

pub trait Handler {
    type Context;
}

pub trait Handles<E: Event>: Handler + Sized {
    fn handle(&mut self, e: E, ctx: &mut Self::Context);
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestEventA {
        pub data: u32,
    }

    impl Event for TestEventA {}

    struct TestHandler {
        data: u32,
    }

    struct TestContext {
        data: u32,
    }

    impl Handler for TestHandler {
        type Context = TestContext;
    }

    impl Handles<TestEventA> for TestHandler {
        type ErrorKind = ();

        fn handle(&mut self, ev: TestEventA, ctx: &TestContext) -> Result<(), ()> {
            self.data += ev.data + ctx.data;
            Ok(())
        }
    }

    #[test]
    fn test_handlers_and_messages() {
        let mut handler = TestHandler { data: 1 };
        let ev = TestEventA { data: 2 };
        let ctx = TestContext { data: 3 };

        handler.handle(ev, &ctx);
        assert_eq!(handler.data, 6);
    }
}
