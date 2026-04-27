# ChatNovel 开发日志

> 版本：v10-tauri
> 日期：2026-03-25 ~ 2026-04-18
> 开发方式：Claude 辅助开发

---

## 一、Electron → Tauri 迁移

### 背景
原 Electron 版本打包体积 268MB，启动慢、内存占用高。迁移到 Tauri 2 以利用系统 WebView2，大幅缩减体积。

### 环境搭建
1. 安装 Rust 工具链（rustc 1.94.0）
2. 安装 Visual Studio C++ Build Tools（解决 `link.exe` 缺失）
3. 安装 Tauri CLI 2.10.1

### 迁移过程
- `main.js`（Node.js）→ `src-tauri/src/main.rs`（Rust）
- `require('electron').ipcRenderer` → `window.__TAURI__.core.invoke`
- 事件监听 `ipc.on()` → `window.__TAURI__.event.listen()`
- 图标统一为 ico 格式
- 解决多个编译问题：
  - `ShortcutState` vs `ShortcutEvent` API 差异
  - `Image::from_path` 不存在，改用 `app.default_window_icon()`
  - `tray:default` 权限不存在，从 capabilities 移除
  - `devUrl` 配置导致等待开发服务器，改为直接加载静态文件

### 成果

| 指标 | Electron v9 | Tauri v10 |
|---|---|---|
| 解压体积 | 268 MB | 8.8 MB |
| 安装包 | ~180 MB | 1.8 MB |
| 内存占用 | ~150 MB | ~30-50 MB |

体积缩小 **30 倍**。

---

## 二、UI/UX 改进

### 联系人头像预设
- 5 个初始联系人（张伟、王芳、刘洋-HR、秦总、项目交流群）全部内嵌 base64 真人风格头像
- 项目交流群使用四宫格拼接头像（Python Pillow 生成）
- 头像经过居中裁剪 + 压缩至 128×128 JPEG，控制体积

### 联系人调整
- "李明" → "秦总"（避免与用户长辈重名）
- 对话内容调整为工作场景（周报确认）

### 文件助手初始界面
- 灰色空白提示 → 模拟聊天消息风格
- 显示三条消息："文件传输服务已就绪" / "收到" / "如需传输文件，点击下方 📎"

### 设置面板
- 设置按钮（⚙️）从 `showNovel` 条件中提出，不上传书也能访问

### 退出确认弹窗
- 新增「取消」按钮
- 遮罩层可点击关闭

### 书架改进
- 每本书显示阅读进度（`当前章节 · 进度%`），未开始的书显示 `章节数 · 未开始`
- 删除按钮从 ✕ 改为垃圾桶图标
- 删除前弹确认框，显示书名 + "删除后阅读进度将无法恢复"

### 长按加速推送
- 按住"下一段"按钮 300ms 后进入加速模式
- 每 150ms 推一段，松开立即停止
- 单击行为不变（推一段）

### 免责声明
- 加入使用指南弹窗
- 四条声明：合法阅读、著作权自查、不在禁止环境使用、后果自负

---

## 三、架构升级：书籍存储迁移至文件系统

### 背景
localStorage 5MB 上限，百万字网文无法存储。

### 方案
- 书籍内容存储到文件系统（`AppData/ChatNovel/books/{id}.json`）
- localStorage 仅存元数据（书名、章节标题列表、阅读进度）
- 章节内容按需加载

### Rust 端新增命令

| 命令 | 功能 |
|---|---|
| `save_book` | 保存书籍章节内容到 JSON 文件 |
| `load_chapter` | 按章节索引按需加载内容 |
| `delete_book` | 删除书籍文件 |

### 前端改动
- `books` 状态从 `[{id, title, chapters: [{title, content}]}]` 改为 `[{id, title, chapters: [{title}]}]`
- 新增 `curChContent` 状态缓存当前章节内容
- `loadCh` / `autoNextCh` 改为 async，通过 IPC 从文件系统读取
- 上传时通过 IPC 存文件，删除时通过 IPC 清理
- 示例小说拆分为 `SAMPLE_META`（元数据）和 `SAMPLE_CHAPTERS`（内容）

### 成果
- 636 万字小说成功加载（1821 章）
- 存储容量无上限

---

## 四、Bug 修复

### 已修复

| 编号 | 级别 | 问题 | 修复方式 |
|---|---|---|---|
| BUG-01 | P0 | localStorage 双重存储溢出 | `bookProgress` 存最近 50 条 nMsgs；`nMsgs` 持久化上限 200 条 |
| BUG-02 | P1 | 默认推送间隔 4s 无对应 UI 按钮 | 默认值改为 3s |
| BUG-03 | P1 | 退出确认弹窗无取消按钮 | 添加取消按钮 + 遮罩可关闭 |
| BUG-04 | P1 | 空章节触发 undefined 消息 | `loadCh` / `autoNextCh` 增加 `s.length > 0` 检查 |
| BUG-06 | P2 | 重复上传同名文件无提示 | 弹确认框"已在书架中，是否替换？" |
| BUG-KB | P1 | 键盘快捷键 useEffect 缺依赖数组 | 补全 `[modal,aBook,resumed,si,segs,pushNext,autoNextCh]` |
| BUG-RC | P1 | autoNextCh 中 400ms 竞态 | 切章时先 `setRun(false)` 暂停定时器 |
| BUG-MEM | P2 | nMsgs 内存无上限增长 | 超过 300 条时截断到 200 条 |
| BUG-QT | P1 | 前引号粘在上一条消息末尾 | 从 `CLOSING_QUOTES` 移除左双引号 `\u201C` |
| BUG-PRG | P1 | 上传新书覆盖旧书进度 | 上传前先调用 `saveProgress()` |
| BUG-RST | P1 | 重启后阅读进度丢失 | 启动时正确加载 `curChContent`，不重置 `si` |
| BUG-DEL | P2 | 删除书籍无确认，✕ 按钮易误触 | 改为垃圾桶图标 + 二次确认弹窗 |

### 保留现有行为

| 编号 | 问题 | 结论 |
|---|---|---|
| BUG-05 | 从托盘唤回后重置阅读状态 | 有意设计：保护伪装安全 |

---

## 五、模式切换进度映射

### 问题
段落模式和聊天模式的分割方式不同，切换时阅读进度会错乱（重复或跳过内容）。

### 解决方案
- 按已读**字符数**在新 segs 中定位
- 回退约 100 个字符作为上下文衔接（"宁可重复，不跳过"策略）
- 使用 `prevSplitKey` ref 区分"首次加载/新章节"和"模式切换"：
  - 首次加载 / 新章节：只设 segs，保留 si
  - 模式切换：字符偏移映射 + 100 字符回退

---

## 六、当前文件结构

```
chatnovel-tauri/
├── dist/
│   └── index.html              # 前端应用（~724 行）
├── src-tauri/
│   ├── Cargo.toml               # Rust 依赖
│   ├── build.rs                 # 构建脚本
│   ├── tauri.conf.json          # Tauri 配置
│   ├── capabilities/
│   │   └── main.json            # 权限声明
│   ├── icons/
│   │   └── icon.ico             # 应用图标
│   └── src/
│       └── main.rs              # Rust 后端（~180 行）
├── README.md
└── CHANGELOG.md
```

---

## 七、待办事项

### 高优先级
- README 加场景描述、GIF 截图、slogan
- 窗口标题可自定义（解决 Alt+Tab 暴露"ChatNovel"）

### 中优先级
- 联系人聊天删除单条消息
- 搜索功能

### 低优先级
- EPUB 格式支持
- 深色模式
- macOS 打包
- 表情面板、通讯录等 WIP 功能

### 不做
- Vite + React 架构重构（当前 724 行单文件尚可维护）
- 假聊天生成器（侵权/伦理风险）
- 阅读人格化（属于另一个产品形态）
- 按字数动态调整推送节奏（过度工程化）

---

## 八、构建命令

```powershell
# 开发模式
cd src-tauri
cargo tauri dev

# 正式打包
cargo tauri build

# 产物
# 单文件：src-tauri/target/release/chatnovel.exe (~8.8 MB)
# 安装包：src-tauri/target/release/bundle/nsis/ChatNovel_1.0.0_x64-setup.exe (~1.8 MB)
```

---

*最后更新：2026-04-18*
