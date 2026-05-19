# agentMarket

终端**营销仪表盘**（Rust + [Ratatui](https://ratatui.rs)），把 knowledge-work
`marketing` 插件（`marketing@knowledge-work-plugins` v1.2.0，通过
`.claude/settings.json` 在项目级启用）的能力领域可视化呈现。界面为中文。

## 运行

```sh
cargo run --release
```

## 按键

| 按键              | 操作                     |
| ----------------- | ------------------------ |
| `↑`/`k`           | 上一个频道               |
| `↓`/`j`           | 下一个频道               |
| `Tab`/`Shift+Tab` | 循环切换频道             |
| `r`               | 刷新（演示用效果序列）   |
| `q`/`Esc`/`Ctrl-C`| 退出                     |

## 频道 → 插件 skill

每个面板把一个营销能力领域映射到支撑真实工作流的插件 skill：

| 频道       | Skill                                |
| ---------- | ------------------------------------ |
| 营销活动   | `campaign-plan`                      |
| 内容创作   | `draft-content` / `content-creation` |
| SEO 优化   | `seo-audit`                          |
| 效果分析   | `performance-report`                 |
| 品牌审查   | `brand-review`                       |
| 竞品分析   | `competitive-brief`                  |
| 邮件序列   | `email-sequence`                     |
| 营销汇报   | `talk-markets`                       |

界面指标仅作示意；把每个面板接到对应 skill 即可获得实时数据。

## 营销汇报（talk-markets）

`talk-markets` 是本项目自带的**项目级 skill**（`.claude/skills/talk-markets/`），
而非来自外部 `marketing` 插件——它在 `talk-html` 之上加了一层「按受众汇报」：
把营销成果做成自包含中文 HTML，并按要报告的对象重排同一批真实证据。

- **受众**：老板/CEO、直属上级/VP、产品 PM、销售/客户成功、媒体记者、客户、
  投资人/董事会、合作伙伴——营销人会遇到的任何角色（矩阵见
  `.claude/skills/talk-markets/references/audiences.md`）。
- **单受众**走倒金字塔；**多受众**用纯 CSS 标签在同一批证据上分轨。
- 数据必须真实有出处（来自上游 `marketing` skill 产出 / 真实导出 / 录屏运行），
  不得编造 KPI；页面机制（自包含 HTML、审计 pill、`publish.sh` 发布 gist、
  `recall.sh` 检索）复用 `talk-html` 的钦定脚本，不复制不重写。

触发：`/talk-markets`、`营销汇报`、`给老板/客户/投资人汇报`、`campaign recap`、
`marketing report`、`stakeholder update` 等。

## 测试

```sh
cargo test      # 无头 TestBackend 渲染 + 行为测试
cargo clippy    # lint
```
