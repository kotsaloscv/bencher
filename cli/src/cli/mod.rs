use std::convert::TryFrom;
use std::process::Command;

use clap::Parser;

mod flag;
mod output;
mod shell;

use crate::adapter;
use crate::adapter::Adapter;
use crate::adapter::Report;
use crate::args::Args;
pub use crate::cli::flag::Flag;
pub use crate::cli::output::Output;
pub use crate::cli::shell::Shell;
use crate::error::CliError;

#[derive(Debug)]
pub struct Cli {
    shell: Shell,
    flag: Flag,
    cmd: String,
    adapter: Adapter,
    tag: Option<Vec<String>>,
}

impl TryFrom<Args> for Cli {
    type Error = CliError;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        Ok(Self {
            shell: Shell::try_from(args.shell)?,
            flag: Flag::try_from(args.flag)?,
            cmd: args.cmd,
            adapter: Adapter::from(args.adapter),
            tag: args.tag,
        })
    }
}

impl Cli {
    pub fn new() -> Result<Self, CliError> {
        let args = Args::parse();
        Self::try_from(args)
    }

    pub fn benchmark(&self) -> Result<Output, CliError> {
        let output = Command::new(self.shell.to_string())
            .arg(self.flag.to_string())
            .arg(&self.cmd)
            .output()?;

        Output::try_from(output)
    }

    pub fn convert(&self, output: Output) -> Result<Report, CliError> {
        match &self.adapter {
            Adapter::Rust => adapter::rust::parse(output),
            Adapter::Custom(adapter) => adapter::custom::parse(&adapter, output),
        }
    }
}
