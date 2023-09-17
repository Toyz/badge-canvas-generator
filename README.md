# IMVU Badge Canvas Image Generator

This tool fetches an IMVU user's badge canvas and creates a single unified image representation of it. Instead of having badges scattered around, you get a single image that visually represents the badge canvas of the user.

## Features

- **Fetch User Badges**: Provide an IMVU user ID and the tool fetches all the badges associated with the user.
- **Badge Placement**: Places each badge at its designated position on the grid.
- **High Quality**: Outputs an image with no compression ensuring the highest quality.
- **Command-line Interface**: Easily specify the CID and output image path through CLI arguments.

## Options

- **`-c, --cid [CID]`**: Specify the user ID (CID) of the IMVU avatar you want to fetch the badge canvas for. You can use this or `-a [AVATAR_NAME]`, but one of them is required.

- **`-a, --avatar-name [AVATAR_NAME]`**: Specify the name of the IMVU avatar. This is an alternative to specifying the user ID (CID). If both are provided, the `-c` option takes precedence.

- **`-o, --output [output_filename]`**: Define the filename for the resulting image. If not provided, the default filename would be `canvas-[AVATAR_NAME].png`.

- **`-g, --grid-color [GRID_COLOR]`**: Customize the grid color in the image. Accepts HEX format (e.g., `#FF5733`). The default grid color is `#ECECEC` with lines being `#D4D4D4`.

- **`-v, --verbose`**: Enable verbose logging to get more detailed information about the process.

- **`-h, --help`**: Display the help message and exit.

Remember to always use one of `-c [CID]` or `-a [AVATAR_NAME]` to specify the avatar whose badge canvas you wish to process.


## Usage

1. **Using User ID (CID) Alone**
    ```bash
    cargo run -- -c [CID]
    ```
   - Replace [CID] with the user ID of the IMVU avatar.
   - The default output filename would be `canvas-[AVATAR_NAME].png`.

2. **Using User ID with Output File Name**
    ```bash
    cargo run -- -c [CID] -o [output_filename]
    ```
   - Replace [CID] with the user ID of the IMVU avatar.
   - Replace [output_filename] with your desired output file name.

3. **Using User ID with Grid Color**
    ```bash
    cargo run -- -c [CID] -g [GRID_COLOR]
    ```
   - Replace [CID] with the user ID of the IMVU avatar.
   - Replace [GRID_COLOR] with your desired grid color in HEX format (e.g., `#FF5733`).

4. **Using User ID with Output File Name and Grid Color**
    ```bash
    cargo run -- -c [CID] -o [output_filename] -g [GRID_COLOR]
    ```
   - Replace [CID] with the user ID of the IMVU avatar.
   - Replace [output_filename] with your desired output file name.
   - Replace [GRID_COLOR] with your desired grid color in HEX format.

5. **Verbose Logging**
   
   Add `-v` to any of the above commands to enable verbose logging. For example:
    ```bash
    cargo run -- -c [CID] -v
    ```

Note: When using the `-a [AVATAR_NAME]` option, it can replace the `-c [CID]` in any of the combinations above.

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
