# Rust PyTorch Project

This is a Rust project that uses PyTorch (through `tch-rs`) for AI and machine learning development.

## Prerequisites

Before you can build and run this project, you need to install libtorch (PyTorch C++ library).

### Installing libtorch on macOS

1. Download libtorch for macOS:
   ```bash
   wget https://download.pytorch.org/libtorch/cpu/libtorch-macos-2.1.0.zip
   ```

2. Extract the downloaded file:
   ```bash
   unzip libtorch-macos-2.1.0.zip
   ```

3. Set the required environment variables. Add these lines to your `~/.zshrc` or `~/.bash_profile`:
   ```bash
   export LIBTORCH=/path/to/libtorch
   export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH
   ```
   Replace `/path/to/libtorch` with the actual path where you extracted libtorch.

4. Reload your shell configuration:
   ```bash
   source ~/.zshrc  # or source ~/.bash_profile
   ```

## Project Setup

1. Make sure you have Rust installed. If not, install it using:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone this repository:
   ```bash
   git clone <repository-url>
   cd slm
   ```

3. The project uses specific versions of dependencies, particularly:
   - `tch = "0.19.0"` (PyTorch bindings for Rust)
   Make sure not to change this version as it's compatible with the current libtorch setup.

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

If you encounter any linking errors:
1. Verify that the `LIBTORCH` environment variable is correctly set
2. Ensure the `LD_LIBRARY_PATH` includes the libtorch library path
3. Check that the installed libtorch version is compatible with tch-rs version 0.19.0

## License

[Add your license information here]
