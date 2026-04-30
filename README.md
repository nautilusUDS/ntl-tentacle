# ntl-tentacle

A standalone relay service for the Nautilus ecosystem. It acts as a bridge between Unix Domain Sockets (UDS) and TCP streams, allowing for flexible service routing and connectivity.

## Features

- **UDS to TCP Forwarding**: Listens on a Unix Domain Socket and forwards traffic to a target TCP address.
- **Connection Pooling**: Manages connections using a semaphore-based pool to prevent resource exhaustion.
- **Health Probing**: Automatically detects if the target service is online before starting the UDS listener.
- **Graceful Shutdown**: Handles shutdown signals to clean up socket files.

## Configuration

Configuration is handled via environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `NAUTILUS_SERVICE_NAME` | Name of the service (used for socket path) | Required |
| `NAUTILUS_TARGET_ADDR` | Target TCP address (e.g., `localhost:80`) | Required |
| `NAUTILUS_SOCKET_NAME` | Filename for the UDS socket | `node-0.sock` |
| `NAUTILUS_SERVICES_DIR` | Base directory for service sockets | `/var/run/nautilus/services` |
| `NAUTILUS_MAX_CONNS` | Maximum concurrent connections | `1024` |

## Getting Started

### Prerequisites

- Rust 1.80+ (for building from source)
- Docker (optional)

### Building from Source

```bash
cargo build --release
```

### Running

```bash
export NAUTILUS_SERVICE_NAME=myapp
export NAUTILUS_TARGET_ADDR=localhost:8080
./target/release/ntl-tentacle
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
