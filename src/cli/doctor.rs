use color_eyre::eyre::{eyre, Result};

use crate::cli::command::Command;
use crate::config::Config;
use crate::env;
use crate::output::Output;

/// Check rtx installation for possible problems.
#[derive(Debug, clap::Args)]
#[clap(verbatim_doc_comment, after_long_help = AFTER_LONG_HELP)]
pub struct Doctor {}

impl Command for Doctor {
    fn run(self, config: Config, _out: &mut Output) -> Result<()> {
        let mut checks = Vec::new();
        for plugin in config.ts.list_plugins() {
            if !plugin.is_installed() {
                checks.push(format!("plugin {} is not installed", plugin.name));
                continue;
            }
        }

        if env::__RTX_DIR.is_none() {
            checks.push(
                "rtx is not activated, run `rtx help activate` for setup instructions".to_string(),
            );
        }

        for check in &checks {
            error!("{}", check);
        }

        if checks.is_empty() {
            Ok(())
        } else {
            Err(eyre!("{} problems found", checks.len()))
        }
    }
}

const AFTER_LONG_HELP: &str = r#"
Examples:
  $ rtx doctor
  [WARN] plugin nodejs is not installed
"#;

#[cfg(test)]
mod test {
    use crate::cli::test::cli_run;

    #[test]
    fn test_doctor() {
        let _ = cli_run(
            &vec!["rtx", "doctor"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<String>>(),
        );
    }
}