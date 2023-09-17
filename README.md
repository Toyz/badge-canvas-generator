# IMVU Badge Canvas Image Generator

This tool fetches an IMVU user's badge canvas and creates a single unified image representation of it. Instead of having badges scattered around, you get a single image that visually represents the badge canvas of the user.

## Features

- **Fetch User Badges**: Provide an IMVU user ID and the tool fetches all the badges associated with the user.
- **Image Tiling**: Generates a base grid image.
- **Badge Placement**: Places each badge at its designated position on the grid.
- **High Quality**: Outputs an image with no compression ensuring the highest quality.
- **Command-line Interface**: Easily specify the CID and output image path through CLI arguments.

## Usage

1. Clone the repository:
```
git clone https://github.com/toyz/badge-canvas-generator.git
```

2. Navigate to the project directory and run:
```
cargo run --release -- -c YOUR_CID -o output.png
```
Replace `YOUR_CID` with the IMVU user CID for which you want to generate the badge canvas image.

## Requirements

- Rust (latest version)
- Access to the internet to fetch badges

## Contributing

Contributions, issues, and feature requests are welcome! Feel free to check [issues page](https://github.com/yourusername/badge-canvas-generator/issues).

## License

MIT License. See `LICENSE` for more information.
