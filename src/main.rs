use cfs::partition::CfsPartition;
use clap::Parser;

#[derive(Parser)]
#[command(version = "1.0", author = "CFS Team")]
struct Cli {
    #[arg(short, long, default_value_t = cfs::DEFAULT_BLOCK_SIZE)]
    block_size: usize,
    blk_dev: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Cli {
        block_size,
        blk_dev,
    } = Cli::parse();

    // block size must be a power of 2
    if block_size & (block_size - 1) != 0 {
        return Err("block size must be a power of 2".into());
    }

    // The blk_dev path must be a valid block device
    if !std::path::Path::new(&blk_dev).exists() {
        return Err("block device path does not exist".into());
    }

    // open file for writing
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&blk_dev)?;

    cfs::init_library_logger();
    let mut cfs_partition = CfsPartition::new(file, block_size as u64)?;
    cfs_partition.write_cfs()?;
    cfs_partition.add_dentry_to_inode(cfs::ROOT_INODE, ".", cfs::ROOT_INODE)?;
    cfs_partition.add_dentry_to_inode(cfs::ROOT_INODE, "..", cfs::ROOT_INODE)?;
    let mut file = std::fs::OpenOptions::new().read(true).open("pablo")?;
    cfs_partition.add_file_to_inode(cfs::ROOT_INODE, "pablo", &mut file)?;
    let mut file = std::fs::OpenOptions::new().read(true).open("./yes_sir/")?;
    cfs_partition.add_file_to_inode(cfs::ROOT_INODE, "yes_sir", &mut file)?;

    Ok(())
}
