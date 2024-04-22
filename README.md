# mapback-rs

`mapback-rs` is a minimalistic command-line tool designed to generate unzoomed level images from map tiles organized in a quadtree structure.

## Installation

To install `mapback-rs`, you need to have Rust and Cargo installed on your system. If you don't have them installed, please follow the official installation guide: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

Once you have Rust and Cargo set up, you can install `mapback-rs` by running the following command:

```shell
cargo install mapback-rs
```

This will download and compile the `mapback-rs` package from crates.io and install it in your Cargo binary directory.

## Usage

To use `mapback-rs`, run the following command:

```shell
mapback-rs <folder>
```

Replace `<folder>` with the path to the folder containing your map tile images.

### Options

- `--max-zoom <level>`: Specifies the most detailed zoom level to consider. Default is 18.
- `--min-zoom <level>`: Specifies the least detailed zoom level to consider. Default is 0.

### Example

```shell
mapback-rs ../map_tiles --max-zoom 16 --min-zoom 10
```

This command will process the map tiles in the `../map_tiles` folder, starting from zoom level 16 and unzooming until zoom level 10.

## File Structure

The map tile images should be organized in the following quadtree structure:

```
<folder>
├── <zoom_level>
│   ├── <x>
│   │   ├── <y>.png
│   │   └── ...
│   └── ...
└── ...
```

- `<folder>`: The root folder containing the map tile images.
- `<zoom_level>`: The zoom level directory (e.g., 16, 15, 14, etc.).
- `<x>`: The X-coordinate directory.
- `<y>.png`: The map tile image file, where `<y>` represents the Y-coordinate.

## Acknowledgements

- [Rust](https://www.rust-lang.org/)
- [clap](https://crates.io/crates/clap) - A simple to use, efficient, and full-featured command-line argument parser
- [image](https://crates.io/crates/image) - Imaging library written in Rust
- [indicatif](https://crates.io/crates/indicatif) - A command line progress reporting library for Rust
