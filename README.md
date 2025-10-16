**扫雷矿工**

[![GitHub release (latest by date)](https://img.shields.io/github/v/release/StudentWeis/minesweeper?style=flat-square)](https://github.com/StudentWeis/minesweeper/releases/latest)

# 简明介绍

这是一个用 Rust 编写的跨平台扫雷游戏，支持 Windows 和 macOS（Linux 开发中）。

## 安装和更新

Shell：

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/StudentWeis/minesweeper/releases/download/0.1.3/minesweeper-installer.sh | sh
```

PowerShell：

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/StudentWeis/minesweeper/releases/download/0.1.3/minesweeper-installer.ps1 | iex"
```

更新：

```sh
minesweeper-update
```

# TODO

- [ ] 长宽可配置
- [ ] 添加重置按钮
- [ ] 跨平台编译
- [ ] 优化性能（降低内存占用）
