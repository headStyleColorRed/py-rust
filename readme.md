# Rust Torch Project

This is a Rust project that uses PyTorch (through `tch-rs`) for AI and machine learning development.

## Project Setup

1. Make sure you have Rust installed. If not, install it using:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone this repository:
   ```bash
   git clone <repository-url>
   cd rust-torch
   ```

3. The project uses specific versions of dependencies, particularly:
   - `tch = "0.19.0"` (PyTorch bindings for Rust)
   Make sure not to change this version as it's compatible with the current libtorch setup.


### Installing libtorch on macOS

The good thing is that there's no need to install libtorch manually and globally. Simply run:
```bash
make setup
```

This will automatically:
1. Download the appropriate version of libtorch for macOS
2. Extract it to the correct location within this project
3. Set up the necessary environment variables in the .cargo/config.toml file

That way this project is self-contained, doesn't require any global installations and you don't have to worry about version conflicts.

## Building the Project

After setting up libtorch and the environment variables, you can build the project:

```bash
cargo build
```

## Project Structure

This project is set up for AI and machine learning development using PyTorch in Rust. It provides a foundation for building:
- Machine Learning models
- Neural Networks
- Deep Learning applications
- AI inference and training pipelines

## Troubleshooting

If you encounter any linking errors open an issue on GitHub and I'll help you out.

## License

MIT License
