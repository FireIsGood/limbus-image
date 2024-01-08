# Limbus Image

<p align="center">
    <img src="./img/limbus-image-image.png" alt="" height="320" />
</p>

> This odyssey had its purpose.

A CLI image generator to make Tier List style images of all the sinners. It works based on a
`config.toml` file that contains sinners along with each of their IDs.

This was made because it seems the tier list maker website hasn't been updated in a while.

If you see any errors, that's totally on me. I wrote this in like 8 hours when I should have been
sleeping. Make an issue or pull request if it bothers you.

## Installation

I can't easily compile to Windows so you'll have to compile this from the source. Sorry!

To build from the source, clone the repo and run the cargo command:

```bash
cargo build --release
```

The binary will be created at `./target/release/limbus-image`.

## Usage

The binary requires a configuration [TOML](https://toml.io/en/) file in the
following format:

```toml
[[sinner]]
name = "Yi Sang" # Sinner name (for bottom text)
path = "yi_sang" # Path of folder
id = [
    { name = "LCB Sinner", rarity = 1, image = "lcb.png" },
    { name = "Blade Lineage Salsu", rarity = 3, image = "blade_lineage.png" },
    ...
]
...
# Repeat for every sinner
```

The images are placed in relative directories to the config file. Honestly it's not helpful to
explain through text, so just check out the `/test` directory and go from there.

After updating the `cargo.toml`, run the binary with a path to the config file. Input and output
directories are inferred based on the folder containing the config.

If no location is given, the config will try to target `./config.toml`.

Assuming you just compiled the program and haven't installed it:

```bash
# Run on `./config.toml` (if it exists)
./target/release/limbus-image

# Run on a specific config file
./target/release/limbus-image ./test/config.toml
```

## Limitations

Images are hard coded at 600 by 600 pixels due to the libraries used. The provided images are at a
lower resolution, though that is because they were taken from a specific game asset.

## Contributing

Feel free to add issues or make pull requests regarding new IDs. Remember that the binary needs to
be run on the config file in `./test/config.toml` to update the repository's images.

## Disclaimer

This project is unaffiliated with Limbus Company's creators/distributors. Images have been taken
from the **Organized Limbus Company Files** from the [ProjectMoon Community Hub](https://discord.gg/pmooncommunityfanhub)
Discord server.

## License

[MIT](https://choosealicense.com/licenses/mit/)
