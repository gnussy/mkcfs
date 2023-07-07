# mkcfs

The mkcfs tool is a command line tool for creating a CFS file system image.

## Installation

```bash
cargo install mkcfs
```

## Build

```bash
cargo build --release
```

## Usage

First, you need to create a image file with the desired size. For example, to
create a 1GB image file:

```bash
dd if=/dev/zero of=image.cfs bs=1M count=1024
```

Then, you can create a CFS file system image with the mkcfs tool:

```bash
mkcfs image.cfs -b 4096
```

Where `-b` specifies the block size of the file system. The default block size
is 4096 bytes.

For more information, please refer to the help message:

```bash
mkcfs --help
```
