# SIM - 3D Physics Simulation

一个基于 Bevy 游戏引擎的 3D 物理仿真项目，支持 WebAssembly 在浏览器中运行。

## Demo

访问在线演示：[https://leido.github.io/sim/](https://leido.github.io/sim/)

## 技术栈

- **Rust** - 系统编程语言
- **Bevy 0.17.3** - 数据驱动的游戏引擎
- **Avian3D 0.4.1** - 3D 物理引擎
- **bevy_egui** - ImGui 风格的 UI 框架
- **Trunk** - WASM 打包工具

## 功能特性

- 3D 物理仿真
- 汽车动力学模拟
- 实时 UI 面板
- 相机控制系统
- 音效系统
- 支持 WebAssembly 运行

## 本地运行

### 前置要求

- [Rust](https://www.rust-lang.org/tools/install) (2024 edition)
- [Trunk](https://trunkrs.dev/) - WASM 打包工具

### 安装 Trunk

```bash
cargo install trunk
```

### 本地运行（原生）

```bash
cargo run
```

### 本地运行（WASM 模式）

```bash
trunk serve --open
```

访问 http://127.0.0.1:8080 查看应用。



## 开发

### 项目结构

```
sim/
├── src/
│   ├── main.rs           # 应用入口
│   ├── camera.rs         # 相机控制
│   ├── car_dynamics.rs   # 汽车动力学
│   ├── init.rs           # 初始化
│   ├── input.rs          # 输入处理
│   ├── panel.rs          # UI 面板
│   ├── sound.rs          # 音效系统
│   └── utils.rs          # 工具函数
├── assets/               # 资源文件
├── index.html            # HTML 入口
├── index.scss            # 样式文件
├── Cargo.toml            # Rust 依赖配置
└── Trunk.toml            # Trunk 配置
```

## 许可证

[MIT](LICENSE)
