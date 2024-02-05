use std::{fmt::Display, path::PathBuf};

use clap::Args;
use derive_more::From;
use futures::FutureExt;
use hotshot_types::traits::{node_implementation::NodeType, signature_key::SignatureKey};
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use tagged_base64::TaggedBase64;
use tide_disco::{api::ApiError, method::ReadState, Api, RequestError, StatusCode};

use crate::{
    api::load_api,
    data_source::BuilderDataSource,
};

#[derive(Args, Default)]
pub struct Options {
    #[arg(long = "builder-api-path", env = "HOTSHOT_BUILDER_API_PATH")]
    pub api_path: Option<PathBuf>,

    /// Additional API specification files to merge with `builder-api-path`.
    ///
    /// These optional files may contain route definitions for application-specific routes that have
    /// been added as extensions to the basic builder API.
    #[arg(
        long = "builder-extension",
        env = "HOTSHOT_BUILDER_EXTENSIONS",
        value_delimiter = ','
    )]
    pub extensions: Vec<toml::Value>,
}

#[derive(Clone, Debug, Snafu, Deserialize, Serialize)]
#[snafu(visibility(pub))]
pub enum BuildError {
    /// The requested resource does not exist or is not known to this builder service.
    NotFound,
    /// The requested resource exists but is not currently available.
    Missing,
    /// There was an error while trying to fetch the requested resource.
    #[snafu(display("Failed to fetch requested resource: {message}"))]
    Error { message: String },
}

#[derive(Clone, Debug, From, Snafu, Deserialize, Serialize)]
#[snafu(visibility(pub))]
pub enum Error {
    Request {
        source: RequestError,
    },
    #[snafu(display("error building block from {resource}: {source}"))]
    #[from(ignore)]
    BlockAvailable {
        source: BuildError,
        resource: String,
    },
    #[snafu(display("error claiming block {resource}: {source}"))]
    #[from(ignore)]
    BlockClaim {
        source: BuildError,
        resource: String,
    },
    Custom {
        message: String,
        status: StatusCode,
    },
}

pub fn define_api<State, Types: NodeType>(options: &Options) -> Result<Api<State, Error>, ApiError>
where
    State: 'static + Send + Sync + ReadState,
    <State as ReadState>::State: Send + Sync + BuilderDataSource<Types>,
    Types: NodeType,
    <<Types as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType:
        for<'a> TryFrom<&'a TaggedBase64> + Into<TaggedBase64> + Display,
    for<'a> <<<Types as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType as TryFrom<
        &'a TaggedBase64,
    >>::Error: Display,
{
    let mut api = load_api::<State, Error>(
        options.api_path.as_ref(),
        include_str!("../api/builder.toml"),
        options.extensions.clone(),
    )?;
    api.with_version("0.0.1".parse().unwrap())
        .get("available_blocks", |req, state| {
            async move {
                let hash = req.blob_param("parent_hash")?;
                state
                    .get_available_blocks(&hash)
                    .await
                    .context(BlockAvailableSnafu {
                        resource: hash.to_string(),
                    })
            }
            .boxed()
        })?
        .get("claim_block", |req, state| {
            async move {
                let hash = req.blob_param("block_hash")?;
                let signature = req.blob_param("signature")?;
                state
                    .claim_block(&hash, &signature)
                    .await
                    .context(BlockClaimSnafu {
                        resource: hash.to_string(),
                    })
            }
            .boxed()
        })?;
    Ok(api)
}
