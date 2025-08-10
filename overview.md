# Project Overview

## Matter Protocol Controller Library (rust-matc)

### Purpose
A Rust library implementing the controller side of the Matter protocol, enabling communication with and control of Matter-compliant smart home devices.

### Architecture

```
┌─────────────────────────────────────────────────┐
│                Demo Application                  │
│              (examples/demo.rs)                  │
└─────────────────────┬───────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────┐
│                  Core Library                    │
│                  (src/lib.rs)                    │
├──────────────────────────────────────────────────┤
│ ┌──────────────┐  ┌──────────────┐             │
│ │   Session    │  │  Controller  │             │
│ │  Management  │  │   Logic      │             │
│ └──────────────┘  └──────────────┘             │
│                                                  │
│ ┌──────────────┐  ┌──────────────┐             │
│ │   Security   │  │  Discovery   │             │
│ │  (PASE/CASE) │  │    (mDNS)    │             │
│ └──────────────┘  └──────────────┘             │
│                                                  │
│ ┌──────────────┐  ┌──────────────┐             │
│ │  Certificate │  │   Message    │             │
│ │  Management  │  │   Protocol   │             │
│ └──────────────┘  └──────────────┘             │
└──────────────────────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────┐
│              Network Transport                   │
│                (UDP/IP Stack)                    │
└──────────────────────────────────────────────────┘
```

### Core Modules

#### Security & Authentication
- **spake2p.rs**: SPAKE2+ protocol implementation for passcode authentication
- **sigma.rs**: SIGMA protocol implementation for certificate authentication
- **cert_matter.rs & cert_x509.rs**: Certificate handling and validation

#### Communication
- **transport.rs**: Network transport layer
- **messages.rs**: Message formatting and parsing
- **tlv.rs**: TLV (Tag-Length-Value) encoding/decoding
- **retransmit.rs**: Message retransmission logic

#### Device Management
- **controller.rs**: Main controller logic
- **commission.rs**: Device commissioning procedures
- **onboarding.rs**: Device onboarding support
- **fabric.rs**: Fabric management

#### Discovery & Session
- **mdns.rs**: mDNS service discovery
- **discover.rs**: Device discovery mechanisms
- **session.rs**: Session establishment and management

#### Clusters
- **clusters/**: Cluster definitions and operations
  - Auto-generated cluster code
  - Name definitions and mappings

#### Utilities
- **util/**: Utility functions
  - **asn1.rs**: ASN.1 encoding/decoding
  - **cryptoutil.rs**: Cryptographic utilities

### Workflow Example

1. **Device Discovery**
   - Use mDNS to discover commissionable devices on network
   
2. **Commissioning**
   - Establish PASE session using passcode
   - Exchange certificates
   - Create operational credentials
   
3. **Operational Control**
   - Establish CASE session using certificates
   - Send commands (on/off, etc.)
   - Read device attributes
   - Manage fabric settings

### Key Features

- **Secure Communication**: End-to-end encrypted communication using industry-standard protocols
- **Certificate Management**: Full PKI support with CA, controller, and device certificates
- **Device Discovery**: Automatic device discovery via mDNS
- **Command & Control**: Rich API for device interaction
- **Multi-fabric Support**: Manage devices across multiple fabrics
- **IPv4/IPv6 Support**: Dual-stack networking capability

### Testing & Examples

- **Unit Tests**: Found in modules (messages.rs, onboarding.rs, tlv.rs, util/asn1.rs)
- **Demo Application**: Comprehensive CLI tool demonstrating all library features
- **Example Scripts**: Simple usage examples (simple.rs, simple2.rs)

### Future Enhancements

- Bluetooth/BLE support for initial commissioning
- Extended cluster support
- Enhanced error handling and recovery
- Performance optimizations
- Additional test coverage