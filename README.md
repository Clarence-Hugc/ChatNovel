# ChatNovel

**在办公室、实验室、任何不方便看小说的场景中，把阅读伪装成聊天。**

ChatNovel 不是阅读器，而是一种「隐身阅读方式」。它将 TXT 小说伪装成聊天消息，以气泡形式定时推送——旁人看到的只是你在回消息。

<!-- TODO: 添加 GIF 截图 -->
<!-- ![demo](screenshots/demo.gif) -->

---

## 使用场景

- 🏢 上班摸鱼但不想被发现
- 📚 图书馆、自习室低调阅读
- 💻 不方便打开阅读器的软件环境

## 特性

**核心阅读**
- TXT 上传，自动识别章节
- 段落模式 / 聊天模式（短句 + 随机间隔）
- 推送间隔、每段字数可调
- 章节结束自动进入下一章
- 长按「下一段」连续快速推送
- 书架管理，多本书独立进度
- GBK 编码自动检测
- 支持百万字级别长篇网文

**伪装系统**
- 默认打开停在普通联系人界面，无任何阅读痕迹
- 左侧预览只显示 `[文件]`，不泄露小说内容
- Esc 一键隐藏到系统托盘（任务栏也消失）
- Ctrl+Shift+S 全局热键从托盘唤回
- 唤回后自动重置到安全界面
- 预设 5 个联系人 + 真实聊天记录

**自定义**
- 气泡颜色、主题色、字体大小
- 发送者名称、头像均可更换
- 自建联系人 + 自定义聊天记录
- 关闭按钮行为可配置

## 体积对比

|  | Electron 版 | Tauri 版 |
|---|---|---|
| 安装包 | ~180 MB | **1.8 MB** |
| 内存占用 | ~150 MB | ~30-50 MB |

## 安装

### 方式一：下载安装包（推荐）

从 [Releases](https://github.com/Clarence-Hugc/ChatNovel/releases) 下载最新版 `ChatNovel_x.x.x_x64-setup.exe`，双击安装即可。

> 需要 Windows 10/11（64位）。Windows 11 已预装 WebView2；Windows 10 可能需要安装 [WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)。

### 方式二：从源码构建

```bash
# 前置条件：Rust、Visual Studio C++ Build Tools、Tauri CLI 2.x

git clone https://github.com/Clarence-Hugc/ChatNovel.git
cd ChatNovel/src-tauri
cargo tauri build
```

产物在 `target/release/bundle/nsis/` 目录下。

## 快捷键

| 快捷键 | 功能 |
|---|---|
| Esc | 隐藏到系统托盘 |
| Ctrl+Shift+S | 从托盘唤回窗口 |
| Ctrl+Space | 推送开关 |
| Ctrl+↓ | 下一段 |
| Ctrl+→ | 下一章 |

## 技术栈

- **前端**：React 18 + Babel（单文件 HTML）
- **桌面封装**：Tauri 2（Rust 后端 + 系统 WebView2）
- **书籍存储**：本地文件系统（AppData）
- **设置存储**：localStorage

## 免责声明

1. 本软件仅供个人学习与合法阅读使用，不得用于任何违法目的。
2. 用户需自行确保所阅读内容不侵犯他人著作权。本软件不提供任何书源或小说内容。
3. 不建议在明确禁止使用此类软件的工作或学习环境中使用。
4. 因使用本软件产生的任何后果由使用者自行承担，开发者不承担任何责任。

## 支持

如果觉得这个项目有用，欢迎请作者喝杯咖啡：

<!-- TODO: 替换为你的 Buy Me a Coffee 链接 -->
<!-- [![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-support-yellow?style=flat&logo=buy-me-a-coffee)](https://buymeacoffee.com/yourname) -->

## License

[MIT](LICENSE)
