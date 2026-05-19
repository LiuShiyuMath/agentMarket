# agentMarket

终端**营销仪表盘**（Rust + [Ratatui](https://ratatui.rs)），把 knowledge-work
`marketing` 插件（`marketing@knowledge-work-plugins` v1.2.0，通过
`.claude/settings.json` 在项目级启用）的 7 个能力域做成可键盘导航的频道，并用
这 7 个频道，对 **pitchkit** 与 **AI Love-Lab** 各跑一遍——两次*各自独立*的
单产品体检（非 A/B 对决）。界面为中文。

## 真实数据，不是演示值

界面里渲染的每一个数字与结论都是**真实数据**，钉在 **2026-05-19 实抓**上：

- pitchkit 的事实来自 GitHub API（`gh api repos/EricSun0218/pitchkit`：
  metadata / languages / 72 commits / contributors / git trees / contents）。
- AI Love-Lab 的事实来自对 `https://lovelab.renlab.ai/` 与母公司
  `renlab.ai` 的 HTTP 实抓（HTML / headers / manifest / robots /
  Googlebot-UA / `/api*` 404 探测）。
- 每个频道的结论文本由 minimax（`MiniMax-M2.7-highspeed` via
  `api.minimaxi.com/anthropic`，`ANTHROPIC_AUTH_TOKEN` 未设置——全程未用
  Claude OAuth）于 2026-05-19 生成。
- 「效果分析」频道左侧是 pitchkit **真实 commit/天序列**（`gh api … commits
  --paginate`，合计 72），不是写死的占位；AI Love-Lab 无公开数据 → 不画图、
  坦白 N/A。按 `s` 可展开「数据来源」核对每一条出处。

全部源数据固化在 `src/data.rs`（每字段带来源注释）；TUI 只负责渲染，不编造。

## 运行

```sh
cargo run --release
```

## 按键

| 按键              | 操作                          |
| ----------------- | ----------------------------- |
| `↑`/`k`           | 上一个频道                    |
| `↓`/`j`           | 下一个频道                    |
| `Tab`/`Shift+Tab` | 循环切换频道                  |
| `1` / `2`         | 切到产品①pitchkit / ②AI Love-Lab |
| `p`               | 在两个产品间切换              |
| `s`               | 展开/收起「数据来源」         |
| `q`/`Esc`/`Ctrl-C`| 退出                          |

## 频道 → 插件 skill

每个频道把一个营销能力领域映射到支撑真实工作流的插件 skill：

| 频道       | Skill                                |
| ---------- | ------------------------------------ |
| 营销活动   | `campaign-plan`                      |
| 内容创作   | `draft-content` / `content-creation` |
| SEO 优化   | `seo-audit`                          |
| 效果分析   | `performance-report`                 |
| 品牌审查   | `brand-review`                       |
| 竞品分析   | `competitive-brief`                  |
| 邮件序列   | `email-sequence`                     |

每个产品都按这 7 个频道各跑一遍，得到一份独立体检；两份之间不互相比较。

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
