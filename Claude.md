# Claude.md - Codebase Documentation

## Project Overview

**rust-matc** is a Rust implementation of the Matter protocol library focused on the controller side functionality. Matter is an open-source connectivity standard for smart home and IoT devices that enables seamless interoperability between devices from different manufacturers.

## Repository Information
- **Repository**: https://github.com/tom-code/rust-matc
- **Version**: 0.1.3
- **License**: BSD-2-Clause
- **Language**: Rust (Edition 2021)
- **Status**: Prototype/Active Development

## Codebase Structure

```
rust-matc/
├── Cargo.toml                   # Project manifest with dependencies
├── LICENSE                      # BSD-2-Clause license
├── readme.md                    # Main documentation with usage examples
├── project_description.md       # Detailed project description
├── overview.md                  # Project overview
├── todo.md                      # Development tasks tracking
├── examples/                    # Example applications
│   ├── demo.rs                  # Comprehensive demo CLI application
│   ├── simple.rs                # Basic usage example
│   └── simple2.rs               # Additional simple example
└── src/                         # Source code
    ├── lib.rs                   # Library entry point with API documentation
    ├── controller.rs            # Matter controller implementation
    ├── transport.rs             # UDP/IP transport layer
    ├── session.rs               # Session management
    ├── commission.rs            # Device commissioning logic
    ├── fabric.rs                # Fabric management
    ├── onboarding.rs            # Device onboarding
    ├── discover.rs              # mDNS-based device discovery
    ├── mdns.rs                  # mDNS implementation details
    ├── messages.rs              # Matter protocol messages
    ├── retransmit.rs            # Message retransmission logic
    ├── tlv.rs                   # TLV encoding/decoding
    ├── spake2p.rs               # SPAKE2+ implementation for PASE
    ├── sigma.rs                 # SIGMA protocol for CASE
    ├── cert_matter.rs           # Matter certificate handling
    ├── cert_x509.rs             # X.509 certificate utilities
    ├── certmanager.rs           # Certificate management interface
    ├── clusters/                # Matter cluster definitions
    │   ├── mod.rs               # Cluster module exports
    │   ├── defs.rs              # Cluster definitions and constants
    │   ├── names.rs             # Cluster name mappings
    │   ├── readme.md            # Cluster documentation
    │   └── gen/                 # Code generation scripts
    │       ├── gen.py           # Python generator
    │       └── gen2.py          # Alternative generator
    └── util/                    # Utility modules
        ├── mod.rs               # Utility exports
        ├── cryptoutil.rs        # Cryptographic utilities
        └── asn1.rs              # ASN.1 parsing utilities
```

## Core Functionality

### Implemented Features

1. **Authentication Methods**
   - **PASE (Passcode Authenticated Session Establishment)**: Secure device pairing using numeric passcodes via SPAKE2+ protocol
   - **CASE (Certificate Authenticated Session Establishment)**: Certificate-based authentication using SIGMA protocol variant

2. **Device Operations**
   - Device commissioning with certificate provisioning
   - Attribute reading from Matter devices
   - Command invocation on devices
   - Fabric management and updates
   - Device discovery via mDNS (both commissionable and commissioned devices)

3. **Protocol Support**
   - TLV (Tag-Length-Value) encoding/decoding for Matter messages
   - Message retransmission and reliability
   - Session establishment and management
   - Transport layer over UDP/IP (IPv4 and IPv6)

### Key Components

- **Controller** (`controller.rs`): Central controller logic for device interaction
- **Transport** (`transport.rs`): Network transport abstraction for UDP communication
- **CertManager** (`certmanager.rs`): Certificate storage and management interface
- **Session** (`session.rs`): Secure session establishment and maintenance
- **TLV** (`tlv.rs`): Matter protocol data encoding/decoding
- **Discovery** (`discover.rs`): mDNS-based device discovery

## Dependencies

### Core Dependencies
- **Cryptography**:
  - `p256` (0.13.0): Elliptic curve operations
  - `ecdsa` (0.16.8): Digital signatures
  - `sha2` (0.10.8), `sha1` (0.10.6): Hash functions
  - `hkdf` (0.12.4): Key derivation
  - `hmac` (0.12.1): Message authentication
  - `pbkdf2` (0.12.2): Password-based key derivation
  - `ccm` (0.5.0), `aes` (0.8.4): Encryption

- **Certificates**:
  - `x509-cert` (0.2.4): X.509 certificate handling
  - `pem` (3.0.*): PEM format support

- **Async Runtime**:
  - `tokio` (1.42.*): Asynchronous runtime with networking support
  - `tokio-util` (0.7.*): Tokio utilities

- **Networking**:
  - `socket2` (0.5.8): Low-level socket operations
  - `if-addrs` (0.13): Network interface enumeration

- **Utilities**:
  - `anyhow` (1.0.*): Error handling
  - `byteorder` (1.5.0): Byte order conversions
  - `hex` (0.4.*): Hex encoding/decoding
  - `log` (0.4), `env_logger` (0.11): Logging

### Development Dependencies
- `clap` (4.4.*): Command-line argument parsing for demo application

## Usage Examples

### Certificate Setup
```rust
let fabric_id = 1000;
let controller_id = 100;
let cm = FileCertManager::new(fabric_id, "./pem");
cm.bootstrap()?;  // Initialize CA
cm.create_user(controller_id)?;  // Create controller certificate
```

### Device Commissioning
```rust
let controller = Controller::new(&cm, &transport, fabric_id)?;
let connection = transport.create_connection("192.168.1.100:5540").await;
let mut connection = controller.commission(&connection, pin, device_id, controller_id).await?;
```

### Command Invocation
```rust
// Turn device on
connection.invoke_request(
    1,  // endpoint
    CLUSTER_ID_ON_OFF,
    CLUSTER_ON_OFF_CMD_ID_ON,
    &[]
).await?;
```

## Demo Application

The demo application (`examples/demo.rs`) provides a comprehensive CLI for:
- CA certificate bootstrapping
- Controller certificate creation
- Device discovery (commissionable and commissioned)
- Device commissioning with passcode
- Command invocation (on/off, level control)
- Attribute reading
- Fabric label updates
- Manual pairing code decoding

### Running the Demo
```bash
# Build the demo
cargo build --example demo

# Bootstrap CA certificates
./target/debug/examples/demo ca-bootstrap

# Create controller certificate
./target/debug/examples/demo ca-create-controller 100

# Discover devices
./target/debug/examples/demo discover commissionable --timeout 3

# Commission a device
./target/debug/examples/demo commission 192.168.1.100:5540 100 300 123456

# Send commands
./target/debug/examples/demo command invoke-command-on --device-address 192.168.1.100:5540 --controller-id 100 --device-id 300
```

## Current Limitations

1. **No Bluetooth/BLE Support**: Devices requiring BLE commissioning must be pre-commissioned using another controller
2. **IP-Only Communication**: Limited to IPv4/IPv6, no Thread or other transport protocols
3. **Prototype Stage**: Library is under active development and APIs may change
4. **Limited Cluster Support**: Only basic clusters implemented

## Development Status

The project is actively maintained with recent updates including:
- Unit test additions for session and SPAKE2P modules
- Dependency updates (matc version 0.1.3)
- Bug fixes and stability improvements
- Documentation enhancements

## Testing

Recent test coverage additions include:
- Comprehensive unit tests for session management (`session.rs`)
- SPAKE2P protocol implementation tests (`spake2p.rs`)
- Tests cover cryptographic operations, message handling, and protocol flows

## Build Instructions

```bash
# Build the library
cargo build

# Run tests
cargo test

# Build with all features
cargo build --all-features

# Generate documentation
cargo doc --no-deps --open

# Build examples
cargo build --examples
```

## Architecture Notes

- **Async Design**: Built on Tokio for efficient async I/O operations
- **Modular Structure**: Clear separation between transport, session, and application layers
- **Trait-Based Abstractions**: CertManager trait allows custom certificate storage implementations
- **Protocol Compliance**: Implements Matter specification for controller operations

## Future Roadmap

Based on `todo.md` and current limitations:
- Bluetooth/BLE transport support
- Additional cluster implementations
- Thread protocol support
- Enhanced error handling and recovery
- Performance optimizations
- Extended test coverage

## Contributing

The project welcomes contributions. Key areas for contribution:
- Additional cluster implementations
- Transport protocol extensions
- Test coverage improvements
- Documentation enhancements
- Bug fixes and performance improvements

## References

- [Matter Specification](https://csa-iot.org/developer-resource/specifications-download-request/)
- [SPAKE2+ RFC](https://datatracker.ietf.org/doc/rfc9383/)
- [SIGMA Protocol](https://www.iacr.org/cryptodb/archive/2003/CRYPTO/1495/1495.pdf)
- [API Documentation](https://tom-code.github.io/rust-matc/matc/)