use blocks_iterator::bitcoin::consensus::serialize;
use blocks_iterator::{periodic_log_level, Config};
use env_logger::Env;
use log::{info, log};
use std::error::Error;
use std::io;
use std::io::Write;
use std::sync::mpsc::sync_channel;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("start");

    let config = Config::from_args();
    let (send, recv) = sync_channel(config.channels_size.into());
    let handle = blocks_iterator::iterate(config, send);
    while let Some(block_extra) = recv.recv()? {
        log!(
            periodic_log_level(block_extra.height, 10_000),
            "# {:7} {} {:?}",
            block_extra.height,
            block_extra.block_hash,
            block_extra.fee()
        );
        let ser = serialize(&block_extra);
        io::stdout().write_all(&ser)?;
    }
    handle.join().expect("couldn't join");
    info!("end");
    Ok(())
}
