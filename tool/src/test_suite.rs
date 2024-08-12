use crate::test::{test, Context as _, Ctx, Tag, Test};

pub fn test_suite() -> Vec<Test<'static>> {
    [test("chain head", |ctx| {
        ctx.tag(Tag::Basic);
        ctx.call::<crate::test::types::Tipset>("Filecoin.ChainHead", ())?;
        Ok(())
    })]
    .into()
}
