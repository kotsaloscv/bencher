use std::convert::TryFrom;

use async_trait::async_trait;

use crate::{
    cli::{
        clap::CliSub,
        wide::Wide,
    },
    BencherError,
};

mod run;
mod subcmd;
mod testbed;

use run::Run;
pub use subcmd::SubCmd;
use testbed::Testbed;

#[derive(Debug)]
pub enum Sub {
    Run(Run),
    Testbed(Testbed),
}

impl TryFrom<CliSub> for Sub {
    type Error = BencherError;

    fn try_from(sub: CliSub) -> Result<Self, Self::Error> {
        Ok(match sub {
            CliSub::Run(run) => Self::Run(Run::try_from(run)?),
            CliSub::Testbed(testbed) => Self::Testbed(Testbed::try_from(testbed)?),
        })
    }
}

pub fn map_sub(sub: Option<CliSub>) -> Result<Option<Sub>, BencherError> {
    if let Some(sub) = sub {
        Ok(Some(Sub::try_from(sub)?))
    } else {
        Ok(None)
    }
}

#[async_trait]
impl SubCmd for Sub {
    async fn exec(&self, wide: &Wide) -> Result<(), BencherError> {
        match self {
            Self::Run(run) => run.exec(wide).await,
            Self::Testbed(testbed) => testbed.exec(wide).await,
        }
    }
}
