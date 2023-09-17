# IMVU Badge Canvas Image Generator

This tool fetches an IMVU user's badge canvas and creates a single unified image representation of it. Instead of having badges scattered around, you get a single image that visually represents the badge canvas of the user.

## Features

- **Fetch User Badges**: Provide an IMVU user ID and the tool fetches all the badges associated with the user.
- **Image Tiling**: Generates a base grid image.
- **Badge Placement**: Places each badge at its designated position on the grid.
- **High Quality**: Outputs an image with no compression ensuring the highest quality.
- **Command-line Interface**: Easily specify the CID and output image path through CLI arguments.

## Usage

1. **Using User ID (CID)**
    ```bash
    cargo run -- -c [CID] -o [output_filename]
    ```

    - Replace `[CID]` with the user ID of the IMVU avatar.
    - Replace `[output_filename]` with your desired output file name. If not provided, it defaults to `canvas.png`.

2. **Using Avatar Name**
    ```bash
    cargo run -- -a [AVATAR_NAME] -o [output_filename]
    ```

    - Replace `[AVATAR_NAME]` with the name of the IMVU avatar.
    - Replace `[output_filename]` with your desired output file name. If not provided, it defaults to `canvas.png`.

**Note**: When using the avatar name, the tool internally fetches the CID using the given name and then generates the image.

## Requirements

- Rust (latest stable version)
- Other dependencies as listed in `Cargo.toml`

## Build & Run

Follow the standard Rust application build process:

```bash
cargo build --release
./target/release/[binary_name] [arguments]
```

## Contributing

Contributions, issues, and feature requests are welcome! Feel free to check [issues page](https://github.com/toyz/badge-canvas-generator/issues).

## License

MIT License. See `LICENSE` for more information.
