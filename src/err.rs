use std::process;

/// Print an error message and exit with code 1
pub fn abort_with(err_msg: &str) -> ! {
    eprintln!(
r#"Aborting 'tool-sync' with error:

    * {}"#,
        err_msg
    );

    process::exit(1);
}

/// Print an error message, suggesting opening an issue and exit with code 1
pub fn abort_suggest_issue(err_msg: &str) -> ! {
    eprintln!(
r#"Aborting 'tool-sync' with error:

    * {}

Please, open an issue in the 'tool-sync' repository and provide as many
details as possible to diagnose the problem if you want to get help with
this issue:

    * https://github.com/chshersh/tool-sync/issues/new"#,
        err_msg
    );

    process::exit(1);
}
