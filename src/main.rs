// Use clap to parse command line arguments
use clap::{Parser, arg, command};
use neo4rs::{Graph, query};
use std::path::PathBuf;

use egraph_serialize_neo4j::commands_from_serialized_egraph;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    json: PathBuf,

    #[arg(long)]
    uri: String,

    #[arg(long)]
    username: String,

    #[arg(long)]
    password: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    log::debug!("Args: {:?}", args);
    let egraph = egraph_serialize::EGraph::from_json_file(&args.json).unwrap();

    let (query_string, params) = commands_from_serialized_egraph(&egraph);

    let graph = Graph::new(&args.uri, &args.username, &args.password)
        .await
        .unwrap();

    log::debug!("Executing query: {}", query_string);
    for (key, value) in params.value.iter() {
        log::debug!("Param: {} = {:?}", key.value, value);
    }

    let mut result = graph
        .execute(query(&query_string).params(params.value))
        .await
        .unwrap();

    while let Ok(Some(row)) = result.next().await {
        log::info!("Result row: {:?}", row);
    }
}
