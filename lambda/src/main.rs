use std::env;
use std::sync::OnceLock;

use anyhow::{anyhow, Context};
use aws_config::BehaviorVersion;
use aws_sdk_location::Client;
use lambda_runtime::{LambdaEvent, run, service_fn};
use serde::Serialize;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

#[derive(Serialize)]
struct Pin {
    lat: f64,
    lon: f64,
}

#[derive(Serialize)]
struct Response {
    success: bool,
    error_msg: Option<String>,
    num_records: Option<usize>,
    results: Option<Vec<Option<String>>>,
}

static INDEX_NAME: OnceLock<String> = OnceLock::new();

static CLIENT: OnceLock<Client> = OnceLock::new();

async fn function_handler(
    LambdaEvent { payload: input, .. }: LambdaEvent<serde_json::Value>,
) -> anyhow::Result<String> {
    tracing::info!("Received input: {input:?} ");

    let client = CLIENT.get().expect("uninitialized client");
    let index_name = INDEX_NAME.get().expect("uninitialized INDEX_NAME");

    let mut success = true;
    let mut error_msg = "".into();
    let mut results = vec![];

    let mut batch: Vec<Vec<String>> = vec![];
    if let Some(arguments) = input.get("arguments") {
        batch = serde_json::from_value(arguments.clone()).context("could not deserialize the arguments")?
    };

    for arguments in batch {
        match client
            .search_place_index_for_text()
            .index_name(index_name)
            .text(&arguments[0])
            .send()
            .await
        {
            Err(error) => {
                success = false;
                error_msg = error.to_string();
                break;
            }
            Ok(output) => {
                let mut result: Vec<Pin> = vec![];
                for search_for_text_result in output.results() {
                    if let Some(point) = search_for_text_result
                        .place()
                        .and_then(|p| p.geometry())
                        .map(|g| g.point())
                    {
                        result.push(Pin {
                            lon: point[0],
                            lat: point[1],
                        });
                    }
                }
                results.push(serde_json::to_string(&result).ok());
            }
        }
    }
    if !success {
        tracing::error!("error_msg: {error_msg}");
    }
    Ok(serde_json::to_string(&Response {
        success,
        error_msg: (!success).then_some(error_msg),
        num_records: success.then_some(results.len()),
        results: success.then_some(results),
    })?)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with_target(false)
        .without_time()
        .init();

    INDEX_NAME
        .set(env::var("INDEX_NAME").context("INDEX_NAME name env var not defined")?)
        .map_err(|_| anyhow!("could not initialize INDEX_NAME"))?;

    CLIENT
        .set(Client::new(
            &aws_config::load_defaults(BehaviorVersion::latest()).await,
        ))
        .map_err(|_| anyhow!("could not initialize client"))?;

    run(service_fn(function_handler))
        .await
        .map_err(|e| anyhow!(e).context("service_fn failed"))
}
