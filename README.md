# rice_bot2
A discord bot written in the programming language [Rust](https://www.rust-lang.org/en-US/)
based off the [Discord-rs](https://github.com/SpaceManiac/discord-rs) library.

[![Travis Build Status](https://travis-ci.org/aarroz/rice_bot2.svg?branch=master)](https://travis-ci.org/aarroz/rice_bot2) 

### Dependencies
Required libraries to install in order to use the bot.

`
pkg-config libsodium opus openssl ffmpeg youtube-dl
`

#### For Ubuntu 16.04 LTS
```sh
sudo apt install pkg-config libsodium-dev libopus-dev openssl ffmpeg youtube-dl
```
Using Pip for youtube-dl is recomended.

#### For Fedora 27
```sh
sudo dnf install pkg-config libsodium-devel opus-devel compat-openssl10-devel ffmpeg youtube-dl
```
Using compat-openssl10-devel instead of openssl package since Fedora is ahead in OpenSSL version.

`ffmpeg` is only avaliable through RPMFusion.

#### For OpenSUSE Tumbleweed
```sh
sudo zypper install pkg-config libsodium-devel libopus-devel libopenssl-devel youtubedl ffmpeg
```

### Installation
- Install [Rust](https://www.rust-lang.org/en-US/install.html).
- Install the required dependencies.
- Either `git clone https://github.com/aarroz/rice_bot2.git` or download the zip and extract.
- Use [Cargo](http://doc.crates.io/) in the folder.
- Use `cargo build` to compile a debug version of the bot.
- Use `cargo run DISCORD_TOKEN=<insert bot token>` to run the bot and connect it to the bot account.
