# YiDNS

<div align="center">
  <img src="res/logo.png" alt="YiDNS Logo" width="200" height="200">
</div>

<div align="center">

[中文](#中文) | [English](#english)

</div>

---

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

[返回英文](#english)

---

## <a id="中文">中文</a>

### 项目简介

YiDNS 是一个轻量级的本地域名解析工具，专为局域网内多设备的本地域名配置而设计。通过 YiDNS，您可以轻松为局域网内的各个设备设置自定义域名，方便不同设备之间的互相访问。

### 功能特性

- 🖥️ **跨平台支持**：支持 Windows、macOS、Linux 和 Docker
- 🔧 **简单配置**：通过简洁的 `config.conf` 文件配置
- 🌐 **局域网本地域名**：为局域网内设备提供本地域名解析
- 📦 **轻量级**：资源占用少，运行效率高

### 支持的平台

- ✅ **Windows** - 已支持
- ✅ **macOS** - 已支持
- ✅ **Linux** - 已支持
- ✅ **Docker** - 已支持
- 🚧 **ReactOS** - 开发中

### 安装与使用

#### 基本步骤

1. 下载对应平台的可执行文件
2. 在可执行文件所在目录下创建 `config.conf` 文件
3. 根据需要配置参数（见下文）
4. 运行 YiDNS

#### 配置文件

在可执行文件的同级目录下创建 `config.conf` 文件，按照以下格式配置：

```ini
[settings]
domain = "example.com" # 自定义域名，例如：example.com
groupid = 0 # 自定义组 ID
```

**配置说明：**
- `domain`：你要设置的本地域名（例如：`mylocal.network`、`office.local` 等）
- `groupid`：组 ID，用于区分不同的设备组，值为非负整数

#### 示例配置

```ini
[settings]
domain = "office.local" # 办公室内网域名
groupid = 1
```

### 工作原理

YiDNS 通过 SSDP 协议在局域网内发现和注册本地域名，使同一网络中的设备可以通过自定义域名相互访问，无需修改系统 hosts 文件或配置复杂的 DNS 服务器。

### 许可证

详见 LICENSE 文件
