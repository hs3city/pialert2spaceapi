# pialert2spaceapi

This is a very simple (and not very smart) project which reads the PiAlert
database and exposes a SpaceAPI endpoint with the current status of your
hackerspace (or makerspace or any other space).

Lots of values are hardcoded for now.

## Setting up the development environment

The quickest way to setup the development enviornment is using dotenv and Nix.
Run `direnv allow` in the root directory of the project and everything will
set itself up. You also need a working Docker installation.

Alternatively, you may need to install rustup, cargo, and the cross crate
individually.

## Building

To build, use `cargo build --release`.

To cross-compile to Raspberry Pi, use
`cross build --target=arm-unknown-linux-gnueabihf --release`.

## Running

Just copy the `pialert2spaceapi` binary you find in `target/release/` to the
machine you want to run it on (use
`target/arm-unknown-linux-gnueabihf/release` for Raspberry Pi). Also, copy
the template `spaceapi.json`.

On the target machine make sure the binary, the JSON template, and the
`pialert.db` are in the same directory, then run `./pialert2spaceapi`.
The endpoint is now listening on port 5000.

## License

(c) 2022 Piotr Gaczkowski

[MIT License](https://choosealicense.com/licenses/mit/)
