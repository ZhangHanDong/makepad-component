# Makepad Component - 项目说明

本项目参考 [gpui-component](../gpui-component) 的组件实现 Makepad 版本。

## CRITICAL: Hook-Based Skill Loading

**IMPORTANT:** When you see a message starting with `[makepad-skills]` in the conversation (from UserPromptSubmit hook), you MUST:

1. **Read the routing instruction** - e.g., `[makepad-skills] Routing to: makepad-widgets makepad-layout`
2. **Immediately call the Skill tool** for EACH skill listed before doing anything else
3. **Do not skip this step** - the skills contain essential Makepad knowledge

Example:
```
Hook output: [makepad-skills] Routing to: makepad-widgets makepad-layout
Action: Call Skill(makepad-widgets), then Skill(makepad-layout)
```

The skills are located in `.claude/skills/` directory and contain Makepad-specific patterns, APIs, and best practices.

## 参考源

- **gpui-component**: `/Users/zhangalex/Work/Projects/FW/robius/gpui-component/crates/ui/src/`

## 组件实现进度

### 已实现组件 ✅

| 组件 | gpui 源文件 | makepad 实现 |
|------|------------|--------------|
| Accordion | accordion.rs | widgets/accordion.rs |
| Avatar | avatar/ | widgets/avatar.rs |
| Badge | badge.rs | widgets/badge.rs |
| Button | button/ | widgets/button.rs |
| Card | - | widgets/card.rs |
| Checkbox | checkbox.rs | widgets/checkbox.rs |
| Divider | divider.rs | widgets/divider.rs |
| Dropdown | select.rs | widgets/dropdown.rs |
| Input | input/ | widgets/input.rs |
| List | list/ | widgets/list.rs |
| Modal | dialog.rs | widgets/modal.rs |
| Notification | notification.rs | widgets/notification.rs |
| PageFlip | - | widgets/page_flip.rs |
| Popover | popover.rs | widgets/popover.rs |
| Progress | progress.rs | widgets/progress.rs |
| Radio | radio.rs | widgets/radio.rs |
| Skeleton | skeleton.rs | widgets/skeleton.rs |
| Slider | slider.rs | widgets/slider.rs |
| Switch | switch.rs | widgets/switch.rs |
| Tab | tab/ | widgets/tab.rs |
| Text | text/ | widgets/text.rs |
| Tooltip | tooltip.rs | widgets/tooltip.rs |
| Label | label.rs | widgets/label.rs |

### 待实现组件 📋

| 组件 | gpui 源文件 | 优先级 | 备注 |
|------|------------|--------|------|
| Alert | alert.rs | 高 | 警告/提示框 |
| Breadcrumb | breadcrumb.rs | 中 | 面包屑导航 |
| Chart | chart/ | 低 | 图表组件 |
| Clipboard | clipboard.rs | 中 | 剪贴板操作 |
| Collapsible | collapsible.rs | 中 | 折叠面板 |
| ColorPicker | color_picker.rs | 低 | 颜色选择器 |
| DescriptionList | description_list.rs | 中 | 描述列表 |
| Dock | dock/ | 低 | 停靠面板 |
| Form | form/ | 高 | 表单组件 |
| GroupBox | group_box.rs | 中 | 分组框 |
| Highlighter | highlighter/ | 低 | 代码高亮 |
| Icon | icon.rs | 高 | 图标组件 |
| Kbd | kbd.rs | 低 | 键盘快捷键显示 |
| Link | link.rs | 中 | 链接组件 |
| Menu | menu/ | 高 | 菜单组件 |
| Pagination | pagination.rs | 中 | 分页组件 |
| Plot | plot/ | 低 | 绑图组件 |
| Rating | rating.rs | 低 | 评分组件 |
| Resizable | resizable/ | 中 | 可调整大小 |
| Scroll | scroll/ | 高 | 滚动组件 |
| Select | select.rs | 高 | 选择器（已有 Dropdown，可扩展） |
| Setting | setting/ | 低 | 设置组件 |
| Sheet | sheet.rs | 中 | 底部抽屉 |
| Sidebar | sidebar/ | 中 | 侧边栏 |
| Spinner | spinner.rs | 高 | 加载动画 |
| Stepper | stepper/ | 中 | 步骤条 |
| Table | table/ | 高 | 表格组件 |
| Tag | tag.rs | 中 | 标签 |
| Time | time/ | 低 | 时间选择器 |
| TitleBar | title_bar.rs | 中 | 标题栏 |
| Tree | tree.rs | 中 | 树形组件 |
| VirtualList | virtual_list.rs | 高 | 虚拟列表 |
| WindowBorder | window_border.rs | 低 | 窗口边框 |

## 项目结构

```
makepad-component/
├── crates/
│   ├── ui/                    # 组件库
│   │   └── src/
│   │       ├── widgets/       # 组件实现
│   │       └── theme/         # 主题配置
│   └── component-zoo/         # 组件展示应用
└── assets/                    # 资源文件
```

## 开发指南

1. 参考 gpui-component 中对应组件的实现逻辑
2. 使用 Makepad 的 DSL 和 Widget 系统重新实现
3. 保持 API 设计尽量接近 gpui 版本，便于迁移

## Default Project Settings

```toml
[package]
edition = "2024"

[dependencies]
makepad-widgets = "0.6"

[features]
default = []
nightly = ["makepad-widgets/nightly"]
```
