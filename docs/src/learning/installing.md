# Installing

### Download binary

You can download the brainease binary from the
[latest release](https://github.com/arthurfiorette/brainease/releases).

There are 3 different OSes available:

- [Linux](https://github.com/arthurfiorette/brainease/releases/latest/download/brainease-x86_64-unknown-linux-musl.zip)
  (`unknown-linux-musl`)
- [MacOS](https://github.com/arthurfiorette/brainease/releases/latest/download/brainease-x86_64-apple-darwin.zip)
  (`apple-darwin`)
- [Windows](https://github.com/arthurfiorette/brainease/releases/latest/download/brainease-x86_64-pc-windows-msvc.zip)
  (`pc-windows-msvc`)

For other than the three mentioned, you can download and install from source.

Then, you can use the binary to run your brainease programs. You can also move the binary
downloaded into your
[bin directory](https://superuser.com/questions/983138/what-is-the-equivalent-of-the-bin-directory-for-windows).

Then, you can just check your installation:

```sh
# Check your installation.
./brainz --version # Or ./brainz.exe if you are on windows

# If you put the binary in your bin directory
brainz --version
```

### Install from cargo

If you have cargo (and rust), you can install brainease for your operating system with:

```sh
# Cargo is the Rust package manager.
cargo install brainease

# Check your installation
brainz --version
```

### Install from source

As the brainease interpreter and compiler are written in Rust, to install them from source
you need to also have [`Rust` and `Cargo`](https://www.rust-lang.org/tools/install)
installed.

Then, you can just follow these steps:

```sh
# Clone the repository
git clone https://github.com/arthurfiorette/brainease ~/.brainease

# Cd into the repository
cd ~/.brainease

# Build the binary
cargo build --release

# Copy the binary to your bin directory
cp ./target/release/brainz /usr/local/bin/brainz

# Apply the executable permissions
sudo chmod +x /usr/local/bin/brainz

# Check your installation
brainz --version
```
