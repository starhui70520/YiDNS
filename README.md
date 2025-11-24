# YiDNS

### Project Introduction

YiDNS is a lightweight local DNS resolution tool designed for configuring local domain names for multiple devices within a LAN. With YiDNS, you can easily set custom domain names for various devices in your local network, making it convenient for devices to communicate with each other.

### Features

- 🖥️ **Cross-Platform Support**: Supports Windows, macOS, Linux, and Docker
- 🔧 **Simple Configuration**: Configure via a simple `config.conf` file
- 🌐 **Local LAN Domain Names**: Provides local domain name resolution for devices in the LAN
- 📦 **Lightweight**: Low resource consumption and high efficiency

### Supported Platforms

- ✅ **Windows** - Supported
- ✅ **macOS** - Supported
- ✅ **Linux** - Supported
- ✅ **Docker** - Supported
- 🚧 **ReactOS** - In Development

### Installation & Usage

#### Getting Started

1. Download the executable for your platform
2. Create a `config.conf` file in the same directory as the executable
3. Configure the parameters as needed (see below)
4. Run YiDNS

#### Configuration File

Create a `config.conf` file in the same directory as the executable with the following format:

```ini
[settings]
domain = "example.com" # custom domain, e.g., example.com
groupid = 0 # custom group ID
```

**Configuration Parameters:**
- `domain`: The local domain name you want to set (e.g., `mylocal.network`, `office.local`, etc.)
- `groupid`: Group ID to distinguish different device groups, value must be a non-negative integer

#### Example Configuration

```ini
[settings]
domain = "office.local" # Office intranet domain name
groupid = 1
```

### How It Works

YiDNS discovers and registers local domain names within a LAN using the SSDP protocol, allowing devices on the same network to communicate with each other using custom domain names without modifying the system hosts file or configuring a complex DNS server.

### License

See the LICENSE file for details
