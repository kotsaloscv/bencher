#![cfg(feature = "plus")]

use bencher_billing::Biller;
use bencher_github::GitHub;
use bencher_json::{
    is_bencher_cloud,
    system::config::{JsonCloud, JsonPlus},
};
use bencher_license::Licensor;
use bencher_schema::context::{Indexer, StatsSettings};
use tokio::runtime::Handle;
use url::Url;

pub struct Plus {
    pub github: Option<GitHub>,
    pub indexer: Option<Indexer>,
    pub stats: StatsSettings,
    pub biller: Option<Biller>,
    pub licensor: Licensor,
}

#[derive(Debug, thiserror::Error)]
pub enum PlusError {
    #[error("Failed to handle self-hosted licensing: {0}")]
    LicenseSelfHosted(bencher_license::LicenseError),
    #[error("Failed to handle Bencher Cloud licensing: {0}")]
    LicenseCloud(bencher_license::LicenseError),
    #[error("Tried to init Bencher Cloud for other Console URL: {0}")]
    BencherCloud(Url),
    #[error("Failed to setup billing: {0}")]
    Billing(bencher_billing::BillingError),
    #[error("{0}")]
    Index(#[from] bencher_schema::context::IndexError),
}

impl Plus {
    pub fn new(console_url: &Url, plus: Option<JsonPlus>) -> Result<Self, PlusError> {
        let Some(plus) = plus else {
            return Ok(Self {
                github: None,
                indexer: None,
                stats: StatsSettings::default(),
                biller: None,
                licensor: Licensor::self_hosted().map_err(PlusError::LicenseSelfHosted)?,
            });
        };

        let github = plus
            .github
            .map(|github| GitHub::new(github.client_id, github.client_secret));

        let stats = plus.stats.map(Into::into).unwrap_or_default();

        let Some(JsonCloud {
            billing,
            license_pem,
            index,
            ..
        }) = plus.cloud
        else {
            return Ok(Self {
                github,
                indexer: None,
                stats,
                biller: None,
                licensor: Licensor::self_hosted().map_err(PlusError::LicenseSelfHosted)?,
            });
        };

        // The only Console URL that should be using the `cloud` section is https://bencher.dev
        if !is_bencher_cloud(console_url) {
            return Err(PlusError::BencherCloud(console_url.clone()));
        }

        let indexer = index.map(TryInto::try_into).transpose()?;

        let biller = Some(
            tokio::task::block_in_place(move || {
                Handle::current().block_on(async { Biller::new(billing).await })
            })
            .map_err(PlusError::Billing)?,
        );
        let licensor = Licensor::bencher_cloud(&license_pem).map_err(PlusError::LicenseCloud)?;

        Ok(Self {
            github,
            indexer,
            stats,
            biller,
            licensor,
        })
    }
}
