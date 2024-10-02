use std::fmt::Write as _;

use clap::Command;
use stackstack::Stack;

/// Create a markdown description for the given [`Command`].
pub fn markdown(cmd: &Command) -> String {
    let mut buf = String::new();
    _markdown(&mut buf, Stack::new(), cmd);
    buf
}

fn _markdown(buf: &mut String, path: Stack<&str>, cmd: &Command) {
    if !path.is_empty() {
        buf.push('\n')
    }
    let path = path.pushed(cmd.get_name());
    for _ in 0..path.len() {
        buf.push('#')
    }
    for component in &path {
        buf.write_fmt(format_args!(" `{component}`")).unwrap();
    }
    buf.push('\n');

    let mut cmd = cmd
        .clone()
        .disable_help_subcommand(true)
        .disable_help_flag(true);
    std::fmt::write(buf, format_args!("\n```\n{}\n```", cmd.render_long_help())).unwrap();
    for sub in cmd.get_subcommands() {
        _markdown(buf, path, sub)
    }
}
