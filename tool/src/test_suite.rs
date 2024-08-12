use crate::test::{test, Context as _, Ctx, Tag, Test};

pub fn test_suite() -> Vec<Test<'static>> {
    [test("chain head", |ctx| {
        ctx.tag(Tag::Basic);
        ctx.call("Filecoin.ChainHead", ())?;
        Ok(())
    })]
    .into()
}
