use crate::test_harness::prelude::*;

pub fn test_suite() -> Vec<Test<'static>> {
    [v0read("chain head", [Tag::SchemaCoverage], |client| {
        client.Filecoin_ChainHead()?;
        Ok(())
    })]
    .into()
}
