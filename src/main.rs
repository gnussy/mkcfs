use cfs;
use clap::Parser;

#[derive(Parser)]
#[command(version = "1.0", author = "CFS Team")]
struct Cli {
    #[arg(short, long, default_value_t = cfs::DEFAULT_BLOCK_SIZE)]
    block_size: usize,

    /// Sets the path to the linux block device
    #[arg(short, long)]
    blk_dev: String,
}

fn main() {
    let _opts: Cli = Cli::parse();
}
