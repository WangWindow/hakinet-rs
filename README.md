# 🐱 Hakinet - 网络抓包工具

一个可爱的命令行网络数据包捕获工具，以小猫为吉祥物！

```
    ╭─────────────────────────────────────────╮
    │                                         │
    │        🐱 Welcome to Hakinet! 🐱        │
    │     Your cute network sniffer cat       │
    │                                         │
    │      /\_/\    Meow! Let's catch some    │
    │     ( o.o )   packets together! 📦      │
    │      > ^ <                              │
    │                                         │
    ╰─────────────────────────────────────────╯
```

## 特性

- 🔍 实时网络数据包捕获
- 🌈 彩色终端输出
- 📁 JSON 格式输出文件
- 🔧 BPF 过滤器支持
- 🖥️ 多网络接口支持
- 🐱 可爱的小猫界面

## 安装

### 前置要求

在 Linux 系统上，您需要安装 libpcap 开发库：

```bash
# Ubuntu/Debian
sudo apt-get install libpcap-dev

# CentOS/RHEL/Fedora
sudo yum install libpcap-devel
# 或者对于较新的系统
sudo dnf install libpcap-devel

# Arch Linux
sudo pacman -S libpcap
```

### 编译安装

```bash
# 克隆项目
git clone <repository-url>
cd hakinet-rs

# 编译
cargo build --release

# 安装到系统路径（可选）
cargo install --path .
```

## 使用方法

### 基本用法

```bash
# 查看帮助
hakinet --help

# 列出可用的网络接口
hakinet interfaces

# 开始捕获数据包（默认接口，无限制）
sudo hakinet capture

# 捕获指定数量的数据包
sudo hakinet capture --count 100

# 在指定接口上捕获
sudo hakinet capture --interface eth0

# 使用过滤器（只捕获 HTTP 流量）
sudo hakinet capture --filter "tcp port 80"

# 保存到文件
sudo hakinet capture --output packets.json

# 详细输出
sudo hakinet capture --verbose
```

### 过滤器示例

Hakinet 支持标准的 BPF (Berkeley Packet Filter) 语法：

```bash
# 捕获特定主机的流量
sudo hakinet capture --filter "host 192.168.1.1"

# 捕获 TCP 流量
sudo hakinet capture --filter "tcp"

# 捕获特定端口
sudo hakinet capture --filter "port 443"

# 捕获 HTTP 和 HTTPS 流量
sudo hakinet capture --filter "tcp port 80 or tcp port 443"

# 捕获 DNS 查询
sudo hakinet capture --filter "udp port 53"

# 排除特定流量
sudo hakinet capture --filter "not host 192.168.1.1"
```

### 输出格式

终端输出示例：
```
🔍 Capturing packets on interface: eth0
📊 Capturing unlimited packets (Ctrl+C to stop)

[1] 14:30:15      TCP    74 bytes 192.168.1.100:54321 → 142.250.185.142:443
[2] 14:30:15      UDP    53 bytes 192.168.1.100:12345 → 8.8.8.8:53
[3] 14:30:15     ICMP    84 bytes 192.168.1.1 → 8.8.8.8 Type: 8, Code: 0
```

JSON 输出格式：
```json
[
  {
    "timestamp": 1684567815,
    "length": 74,
    "protocol": "TCP",
    "src_addr": "192.168.1.100",
    "dst_addr": "142.250.185.142",
    "src_port": 54321,
    "dst_port": 443,
    "info": "Flags: 24"
  }
]
```

## 权限要求

由于网络数据包捕获需要访问原始套接字，通常需要管理员权限：

```bash
# 使用 sudo 运行
sudo hakinet capture

# 或者给二进制文件设置特殊权限（Linux）
sudo setcap cap_net_raw,cap_net_admin=eip ./target/release/hakinet
```

## 开发

### 项目结构

```
src/
├── main.rs      # 主程序和命令行界面
├── capture.rs   # 数据包捕获逻辑
├── packet.rs    # 数据包信息结构
├── filter.rs    # 过滤器相关功能
└── output.rs    # 输出处理
```

### 运行测试

```bash
cargo test
```

### 开发模式运行

```bash
cargo run -- capture --help
```

## 协议支持

目前支持的协议：
- ✅ Ethernet
- ✅ IPv4
- ✅ IPv6
- ✅ TCP
- ✅ UDP
- ✅ ICMP
- ✅ ARP

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

本项目采用 MIT 许可证。

## 小猫说

```
   /\_/\
  ( ^.^ ) "感谢使用 Hakinet！记住要负责任地使用网络工具喵！"
   > ^ <
```

---

**注意**: 请确保您有权在目标网络上进行数据包捕获，并遵守相关法律法规。
