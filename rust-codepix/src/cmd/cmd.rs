use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
  /// Start server in default port 50051. Use -p for choses port.
  /// example: rust-codepix -p 50052 start
  Start,
  /// Start consuming transactions using Apache kafka
  Kafka,
  /// Start gRpc Server and Kafka
  All,
}

#[derive(StructOpt, Debug)]
#[structopt(
  name = "Codepix Grpc server",
  about = "A command line Codepix Grpc server written in Rust"
)]

//create a struct for commandLineArgs
pub struct CommandLineArgs {
  #[structopt(subcommand)]
  pub action: Action,

  /// Define the port for server Grpc
  #[structopt(short = "p", long, default_value = "50051")]
  pub port: String,
}
