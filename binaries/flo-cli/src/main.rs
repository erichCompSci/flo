use structopt::StructOpt;

mod client;
mod env;
mod game;
mod grpc;
mod kinesis;
mod lan;
mod observer;
mod replay;
mod server;

pub use anyhow::Result;

#[derive(Debug, StructOpt)]
enum Opt {
  Client {
    player_id: i32,
    #[structopt(subcommand)]
    cmd: client::Command,
  },
  Server {
    #[structopt(subcommand)]
    cmd: server::Command,
  },
  Lan {
    #[structopt(subcommand)]
    cmd: lan::Command,
  },
  Observer {
    #[structopt(subcommand)]
    cmd: observer::Command,
  },
  Kinesis {
    #[structopt(subcommand)]
    cmd: kinesis::Command,
  },
  Replay {
    #[structopt(subcommand)]
    cmd: replay::Command,
  },
}

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  // flo_log_subscriber::init_env_override("debug,h2=error,async_dnssd=error");
  flo_log_subscriber::init();

  let opt = Opt::from_args();

  match opt {
    Opt::Client { player_id, cmd } => {
      cmd.run(player_id).await?;
    }
    Opt::Server { cmd } => {
      cmd.run().await?;
    }
    Opt::Lan { cmd } => {
      cmd.run().await?;
    }
    Opt::Observer { cmd } => {
      cmd.run().await?;
    }
    Opt::Kinesis { cmd } => {
      cmd.run().await?;
    }
    Opt::Replay { cmd } => {
      cmd.run().await?;
    }
  }

  Ok(())
}
