# YiDNS

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
