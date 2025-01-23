# <h1 align="center"> TX Relayer Blueprint 🌐 </h1>

**A Transaction Relayer Blueprint for Tangle Network implementing EIP-712 permit functionality**

## 📚 Overview

This project implements a Transaction Relayer Blueprint for the Tangle Network, allowing for gas-less transactions through EIP-712 permits. The blueprint consists of two main components:

1. **CallPermit Precompile Contract**: Implements the EIP-712 standard for permitting and dispatching calls on behalf of users.

## 🔑 Key Features

- Gas-less transaction execution using EIP-712 permits
- Secure message signing and verification
- Configurable Allowlist for permitted contracts and functions.

## 📚 Prerequisites

Before running this project, ensure you have:

- [Rust](https://www.rust-lang.org/tools/install)
- [Forge](https://getfoundry.sh)
- [Tangle](https://github.com/tangle-network/tangle?tab=readme-ov-file#-getting-started-)
- [cargo-tangle](https://crates.io/crates/cargo-tangle)

Install cargo-tangle using either:

```bash
# Via installer script
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/tangle-network/gadget/releases/download/cargo-tangle-v0.1.2/cargo-tangle-installer.sh | sh

# Or via cargo
cargo install cargo-tangle --force
```

## 🛠️ Development

### Building the Project

```bash
cargo build
```

### Configuration

The blueprint can be configured using the `config.toml` or `config.json` file. The configuration file contains the following fields:

- `port`: The port on which the relayer will run.
- `allowed_calls`: A mapping of contract addresses to allowed function signatures.

#### Example `config.json`

```json
{
  "port": 3000,
  "allowed_calls": {
    "0x0000000000000000000000000000000000000822": [
      "0xb3c11395",
      "0xa12de0ba"
    ]
  }
}
```

### API Endpoints

See [API.md](./API.md) for a list of available API endpoints and their descriptions.

### Running Tests

See the [tests](./tests) directory for examples on how to use the blueprint.

### Deployment

Deploy the blueprint to the Tangle network:

```bash
cargo tangle blueprint deploy
```

## 📖 Contract Details

### CallPermit Contract
- Address: `0x0000000000000000000000000000000000000805`
- Implements EIP-712 standard for gas-less transactions
- Provides nonce management and domain separation
- Handles permit verification and call dispatching

## 📜 License

Licensed under either:
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

## 📬 Contributing

Contributions are welcome! Please feel free to submit issues and pull requests on our [GitHub repository](https://github.com/tangle-network/blueprint-template/issues).

Unless explicitly stated otherwise, contributions will be dual licensed as above without additional terms or conditions.
