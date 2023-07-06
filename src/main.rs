#![allow(dead_code)]
use clap::Parser;
use deku::prelude::*;

#[derive(Debug)]
struct CfsPartition {
    blk_dev: String,
    block_size: u64,
    iam_blocks: u64,
    inode_list_blocks: u64,
    bam_blocks: u64,
    nblocks: u64,
    ninodes: u64,
    data_start: u64,
}

impl CfsPartition {
    fn new(block_size: u64, blk_dev: String) -> Result<Self, Box<dyn std::error::Error>> {
        let blk_dev_metadata = std::fs::metadata(&blk_dev)?;
        let size = blk_dev_metadata.len();
        let nblocks = size / block_size;
        let bam_blocks =
            nblocks + cfs::bits_per_block(block_size) - 1 / cfs::bits_per_block(block_size);
        let inode_list_blocks = (nblocks / 4) / cfs::bits_per_block(block_size);
        let ninodes = inode_list_blocks * cfs::bits_per_block(block_size);
        let iam_blocks =
            (ninodes + cfs::bits_per_block(block_size) - 1) / cfs::bits_per_block(block_size);
        let data_start = cfs::RESERVED_BLOCKS + bam_blocks + iam_blocks + inode_list_blocks;

        Ok(CfsPartition {
            blk_dev,
            block_size,
            iam_blocks,
            inode_list_blocks,
            bam_blocks,
            nblocks,
            ninodes,
            data_start,
        })
    }

    fn bam_start(&self) -> u64 {
        cfs::RESERVED_BLOCKS
    }

    fn iam_start(&self) -> u64 {
        self.bam_start() + self.bam_blocks
    }

    fn inode_start(&self) -> u64 {
        self.iam_start() + self.iam_blocks
    }

    fn data_start(&self) -> u64 {
        self.inode_start() + self.inode_list_blocks
    }

    fn write(&self) -> Result<(), Box<dyn std::error::Error>> {
        // ┌────────────┬─────────────────────────┬─────────────────────────┬────────────┬──────────────┬─────┬──────────────┐
        // │Super Block │ Block Allocation Bitmap │ Inode Allocation Bitmap │ Inode List │ Data Block 0 │ ... │ Data Block N │
        // └────────────┴─────────────────────────┴─────────────────────────┴────────────┴──────────────┴─────┴──────────────┘

        // Super block
        let super_block = cfs::superblock::SuperBlock::new(
            cfs::MAGIC,
            self.block_size as u32,
            self.bam_blocks as u32,
            self.iam_blocks as u32,
            self.inode_list_blocks as u32,
            self.nblocks as u32,
            self.ninodes as u32,
        );

        // BAM - Allocate a bitmap with the first block occupied by the root directory
        // and all other blocks free
        let mut bam = cfs::bitmap::Bitmap::new(self.bam_blocks as usize);
        bam.set(0);

        // IAM - Allocate a bitmap with the first inode occupied by the root directory
        // and all other inodes free
        let mut iam = cfs::bitmap::Bitmap::new(self.iam_blocks as usize);
        iam.set(0);
        iam.set(1);

        // Inode List - Allocate the first inode for the root directory
        let inode_list = cfs::inode::InodeList::new();

        // Create the CFS
        let cfs = cfs::Cfs::new(super_block, bam, iam, inode_list);

        std::fs::write(&self.blk_dev, cfs.to_bytes()?)?;

        Ok(())
    }
}

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

    let cfs_partition = CfsPartition::new(block_size as u64, blk_dev)?;
    cfs_partition.write()?;

    Ok(())
}
