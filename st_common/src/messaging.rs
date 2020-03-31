pub trait Message {}
pub trait Handler {
    type Context;
}

pub trait Handles<M: Message>: Handler + Sized {
    type ErrorKind;

    fn handle(&mut self, m: M, ctx: &<Self as Handler>::Context) -> Result<(), Self::ErrorKind>;
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestMessageA {
        pub data: u32,
    }

    impl Message for TestMessageA {}

    struct TestHandler {
        data: u32,
    }

    struct TestContext {
        data: u32,
    }

    impl Handler for TestHandler {
        type Context = TestContext;
    }

    impl Handles<TestMessageA> for TestHandler {
        type ErrorKind = ();

        fn handle(&mut self, msg: TestMessageA, ctx: &TestContext) -> Result<(), ()> {
            self.data += msg.data + ctx.data;
            Ok(())
        }
    }

    #[test]
    fn test_handlers_and_messages() {
        let mut handler = TestHandler { data: 1 };
        let msg = TestMessageA { data: 2 };
        let ctx = TestContext { data: 3 };

        handler.handle(msg, &ctx);
        assert_eq!(handler.data, 6);
    }
}
