use crate::test::{test, Context as _, Tag, Test};

pub fn test_suite() -> Vec<Test<'static>> {
    [test("chain head", |cfg| {
        let mut run = cfg.tag(Tag::SchemaCoverage).begin_test()?;

        Ok(())
    })]
    .into()
}
