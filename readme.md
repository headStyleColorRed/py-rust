# Rust PyTorch Project

This is a Rust project that uses PyTorch (through `tch-rs`) for AI and machine learning development.

## Prerequisites

Before you can build and run this project, you need to install libtorch (PyTorch C++ library).

### Installing libtorch on macOS

Simply run:
```bash
make setup
```

This will automatically:
1. Download the appropriate version of libtorch for macOS
2. Extract it to the correct location
3. Set up the necessary environment variables

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
