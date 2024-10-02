use assert_cmd::Command;

#[test]
fn openrpc_select_is_up_to_date() -> anyhow::Result<()> {
    let repo_root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let stdout = Command::cargo_bin("cna-tool")?
        .args(["openrpc", "select"])
        .args([
            "schemas/forest-6ba5c966097-path-v0.json",
            "misc/method-list.json",
        ])
        .current_dir(repo_root) // root of repo
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let stdout = String::from_utf8(stdout)?;
    expect_test::expect_file![format!("{repo_root}/spec.json")].assert_eq(&stdout);
    Ok(())
}
