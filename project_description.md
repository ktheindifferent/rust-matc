# Project Description

## Overview
This is a Rust implementation of the Matter protocol library, focusing on the controller side functionality. Matter is an open-source connectivity standard for smart home and IoT devices.

## Current Work Summary

### Implemented Features
- **PASE (Passcode Authenticated Session Establishment)**: Implementation of SPAKE2+ variant for secure device pairing using passcodes
- **CASE (Certificate Authenticated Session Establishment)**: SIGMA variant implementation for certificate-based authentication
- **Commissioning Procedure**: Ability to sign and push certificates to devices
- **Basic Device Interactions**:
  - Read attributes from devices
  - Invoke commands on devices
  - Device control (on/off commands)
  - Fabric management operations

### Key Components
- **Certificate Management**: X.509 certificate handling and Matter-specific certificate structures
- **Network Discovery**: mDNS-based device discovery for both commissionable and commissioned devices
- **Message Protocol**: TLV (Tag-Length-Value) encoding/decoding for Matter protocol messages
- **Session Management**: Secure session establishment and maintenance
- **Cluster Support**: Basic cluster definitions and operations

### Technology Stack
- Language: Rust
- Networking: UDP/IP (IPv4 and IPv6 support)
- Discovery: mDNS
- Cryptography: SPAKE2+, SIGMA, X.509 certificates
- Command-line Interface: Clap for demo application

### Current Limitations
- No Bluetooth/BLE support (requires pre-commissioning via other controller)
- Limited to IP-based communication
- Protocol library is in prototype stage

### Demo Application
A comprehensive demo application is provided with capabilities for:
- CA certificate bootstrapping
- Controller certificate creation
- Device discovery
- Device commissioning
- Command invocation
- Attribute reading
- Fabric label updates

## Development Status
The project is actively maintained with recent updates including:
- Dependency updates (matc version bump to 0.1.3)
- Support for different endpoints in demo commands
- Logging improvements
- Bug fixes and maintenance

## Documentation
- API documentation available via GitHub Pages
- Examples provided in the examples directory
- Comprehensive README with usage instructions