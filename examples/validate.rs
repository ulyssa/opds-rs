use std::path::PathBuf;

use opds::v2_0::Feed;

#[derive(clap::Subcommand)]
enum ValidatorCommand {
    /// Validate a feed.
    Feed {
        /// File to parse.
        file: PathBuf,
    },
}

impl ValidatorCommand {
    fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::Feed { file } => {
                let json = std::fs::read_to_string(&file)?;
                let feed: Feed<'_> = serde_json::from_str(&json)?;
                let output = serde_json::to_string_pretty(&feed)?;
                println!("{output}");
                Ok(())
            }
        }
    }
}

#[derive(clap::Parser)]
struct Validator {
    #[command(subcommand)]
    command: ValidatorCommand,
}

impl Validator {
    fn run(&self) -> anyhow::Result<()> {
        self.command.run()
    }
}

fn main() -> anyhow::Result<()> {
    let cmd = <Validator as clap::Parser>::parse();
    cmd.run()?;
    Ok(())
}
