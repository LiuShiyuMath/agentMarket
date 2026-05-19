//! agentMarket — 终端营销仪表盘。
//!
//! 把 knowledge-work `marketing` 插件的 7 个能力域做成可键盘导航的频道，并用
//! 这 7 个频道，对 **pitchkit** 与 **AI Love-Lab** 各跑一遍 —— 两次 *各自独立*
//! 的单产品体检（非 A/B 对决）。
//!
//! 界面里渲染的每一个数字与结论都是 **真实数据**：事实来自 2026-05-19 对
//! GitHub API / 目标站点的实抓，频道结论由 minimax 生成（详见 `data.rs`）。
//! 这里没有写死的演示值——左侧走势图是 pitchkit 真实 commit/天序列，不是占位。

mod data;

use std::time::Duration;

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph, Sparkline, Wrap},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use data::{Product, CAPTURED, PRODUCTS, PROVENANCE};

const PLUGIN: &str = "marketing@knowledge-work-plugins v1.2.0";
const ACCENT: Color = Color::Rgb(255, 122, 0);
const ROSE: Color = Color::Rgb(255, 107, 157);

struct App {
    /// 当前产品（0 = pitchkit, 1 = AI Love-Lab）。两次独立体检，不互比。
    product: usize,
    /// 当前频道（0..7）。
    list: ListState,
    /// 是否在面板里展开「数据来源」（这份分析钉在哪些真实来源上）。
    show_sources: bool,
    running: bool,
}

impl App {
    fn new() -> Self {
        let mut list = ListState::default();
        list.select(Some(0));
        Self {
            product: 0,
            list,
            show_sources: false,
            running: true,
        }
    }

    fn product(&self) -> &'static Product {
        &PRODUCTS[self.product]
    }

    fn channel(&self) -> usize {
        self.list.selected().unwrap_or(0)
    }

    fn step_channel(&mut self, delta: isize) {
        let n = self.product().channels.len() as isize;
        let next = (self.channel() as isize + delta).rem_euclid(n);
        self.list.select(Some(next as usize));
    }

    fn select_product(&mut self, idx: usize) {
        if idx < PRODUCTS.len() {
            self.product = idx;
        }
    }

    fn toggle_product(&mut self) {
        self.product = (self.product + 1) % PRODUCTS.len();
    }

    fn on_key(&mut self, code: KeyCode, mods: KeyModifiers) {
        match code {
            KeyCode::Char('q') | KeyCode::Esc => self.running = false,
            KeyCode::Char('c') if mods.contains(KeyModifiers::CONTROL) => self.running = false,
            KeyCode::Down | KeyCode::Char('j') => self.step_channel(1),
            KeyCode::Up | KeyCode::Char('k') => self.step_channel(-1),
            KeyCode::Tab => self.step_channel(1),
            KeyCode::BackTab => self.step_channel(-1),
            KeyCode::Char('1') => self.select_product(0),
            KeyCode::Char('2') => self.select_product(1),
            KeyCode::Char('p') => self.toggle_product(),
            KeyCode::Char('s') => self.show_sources = !self.show_sources,
            _ => {}
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    while app.running {
        terminal.draw(|f| ui(f, &mut app))?;
        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(k) = event::read()? {
                if k.kind == KeyEventKind::Press {
                    app.on_key(k.code, k.modifiers);
                }
            }
        }
    }

    ratatui::restore();
    Ok(())
}

/// 按终端显示列宽在右侧补空格（CJK 字符占 2 列），保证导航列对齐。
fn pad_display(s: &str, width: usize) -> String {
    let w = UnicodeWidthStr::width(s);
    if w >= width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(width - w))
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let [header, body, footer] = Layout::vertical([
        Constraint::Length(4),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .areas(f.area());

    render_header(f, header);

    let [nav, panel] =
        Layout::horizontal([Constraint::Length(26), Constraint::Min(0)]).areas(body);
    render_nav(f, nav, app);
    render_panel(f, panel, app);

    render_footer(f, footer);
}

fn render_header(f: &mut Frame, area: Rect) {
    let lines = vec![
        Line::from(vec![
            Span::styled(" agentMarket ", Style::new().fg(Color::Black).bg(ACCENT).bold()),
            Span::raw("  营销仪表盘 · 两次独立单产品体检"),
        ]),
        Line::from(vec![
            Span::styled(format!("{CAPTURED}  ·  "), Style::new().fg(Color::Green)),
            Span::styled(PROVENANCE, Style::new().fg(Color::DarkGray)),
        ]),
    ];
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().fg(ACCENT));
    f.render_widget(Paragraph::new(lines).block(block), area);
}

fn render_nav(f: &mut Frame, area: Rect, app: &mut App) {
    let items: Vec<ListItem> = app
        .product()
        .channels
        .iter()
        .map(|c| ListItem::new(Line::from(Span::raw(pad_display(c.name, 12)))))
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" 频道 ")
                .borders(Borders::ALL)
                .padding(Padding::horizontal(1)),
        )
        .highlight_style(Style::new().fg(ACCENT).add_modifier(Modifier::BOLD))
        .highlight_symbol("▸ ");

    let mut state = app.list.clone();
    f.render_stateful_widget(list, area, &mut state);
}

fn render_panel(f: &mut Frame, area: Rect, app: &App) {
    let p = app.product();
    let facts_h = (p.facts.len() as u16) + 2;

    let [tabs, ident, facts, channel] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(4),
        Constraint::Length(facts_h),
        Constraint::Min(0),
    ])
    .areas(area);

    render_product_tabs(f, tabs, app);

    // 产品身份：tagline + url。
    let ident_lines = vec![
        Line::from(Span::styled(
            p.tagline,
            Style::new().fg(Color::Gray).add_modifier(Modifier::ITALIC),
        )),
        Line::from(vec![
            Span::styled("源  ", Style::new().fg(Color::DarkGray)),
            Span::styled(p.url, Style::new().fg(Color::Cyan)),
        ]),
    ];
    f.render_widget(
        Paragraph::new(ident_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .padding(Padding::horizontal(1)),
        ),
        ident,
    );

    // 实抓事实。
    let fact_lines: Vec<Line> = p
        .facts
        .iter()
        .map(|(k, v)| {
            Line::from(vec![
                Span::styled(pad_display(k, 14), Style::new().fg(Color::DarkGray)),
                Span::raw(*v),
            ])
        })
        .collect();
    f.render_widget(
        Paragraph::new(fact_lines)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title(format!(" 实抓事实 · {CAPTURED} "))
                    .borders(Borders::ALL)
                    .padding(Padding::horizontal(1)),
            ),
        facts,
    );

    render_channel(f, channel, app);
}

fn render_product_tabs(f: &mut Frame, area: Rect, app: &App) {
    let mut spans = vec![Span::raw(" ")];
    for (i, prod) in PRODUCTS.iter().enumerate() {
        let label = format!(" {} {} ", if i == 0 { "①" } else { "②" }, prod.name);
        let style = if i == app.product {
            Style::new().fg(Color::Black).bg(ROSE).bold()
        } else {
            Style::new().fg(Color::DarkGray)
        };
        spans.push(Span::styled(label, style));
        spans.push(Span::raw("  "));
    }
    spans.push(Span::styled(
        "（各自独立体检 · 非 A/B）",
        Style::new().fg(Color::DarkGray),
    ));
    f.render_widget(
        Paragraph::new(Line::from(spans)).block(
            Block::default()
                .title(" 产品 ")
                .borders(Borders::ALL),
        ),
        area,
    );
}

fn render_channel(f: &mut Frame, area: Rect, app: &App) {
    let p = app.product();

    if app.show_sources {
        let mut lines = vec![Line::from(Span::styled(
            format!("{} · 这份分析钉在哪些真实来源上（{CAPTURED}）", p.name),
            Style::new().fg(ROSE).add_modifier(Modifier::BOLD),
        ))];
        for src in p.sources {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled("• ", Style::new().fg(ACCENT)),
                Span::raw(*src),
            ]));
        }
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            PROVENANCE,
            Style::new().fg(Color::DarkGray),
        )));
        f.render_widget(
            Paragraph::new(lines).wrap(Wrap { trim: true }).block(
                Block::default()
                    .title(" 数据来源 · 按 s 收起 ")
                    .borders(Borders::ALL)
                    .padding(Padding::new(1, 1, 1, 1)),
            ),
            area,
        );
        return;
    }

    let c = &p.channels[app.channel()];
    // pitchkit 的「效果分析」频道有真实 commit/天序列；其它频道或无数据产品不画图。
    let show_chart = c.skill == "performance-report" && p.series.is_some();

    let [head, spark, verdict] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(if show_chart { 8 } else { 0 }),
        Constraint::Min(0),
    ])
    .areas(area);

    let head_lines = vec![Line::from(vec![
        Span::styled(
            c.name,
            Style::new().fg(ACCENT).add_modifier(Modifier::BOLD),
        ),
        Span::raw("   "),
        Span::styled(c.skill, Style::new().fg(Color::DarkGray)),
    ])];
    f.render_widget(
        Paragraph::new(head_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .padding(Padding::horizontal(1)),
        ),
        head,
    );

    if show_chart {
        if let Some(s) = &p.series {
            let [bars, cap] =
                Layout::vertical([Constraint::Length(6), Constraint::Length(2)]).areas(spark);
            let sp = Sparkline::default()
                .block(
                    Block::default()
                        .title(s.title)
                        .borders(Borders::ALL),
                )
                .data(s.data.iter().copied())
                .style(Style::new().fg(ACCENT));
            f.render_widget(sp, bars);
            f.render_widget(
                Paragraph::new(Line::from(Span::styled(
                    s.caption,
                    Style::new().fg(Color::DarkGray),
                )))
                .wrap(Wrap { trim: true }),
                cap,
            );
        }
    }

    let verdict_p = Paragraph::new(c.verdict).wrap(Wrap { trim: true }).block(
        Block::default()
            .title(" 结论 · minimax 生成 ")
            .borders(Borders::ALL)
            .padding(Padding::new(1, 1, 1, 1)),
    );
    f.render_widget(verdict_p, verdict);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let keys = "↑↓/jk 频道  ·  1/2·p 切产品  ·  s 数据来源  ·  q 退出";
    let line = Line::from(vec![
        Span::styled(format!(" {keys} "), Style::new().fg(Color::DarkGray)),
        Span::raw("   "),
        Span::styled(PLUGIN, Style::new().fg(ACCENT)),
    ]);
    f.render_widget(Paragraph::new(line).alignment(Alignment::Left), area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{backend::TestBackend, Terminal};

    /// Scrape the rendered buffer back to text. ratatui stores a wide (CJK)
    /// grapheme in one cell and fills the next cell with a literal space, so a
    /// naive symbol join splits multi-byte text. Skip the filler cell after any
    /// width-2 grapheme to reconstruct what the user actually sees.
    fn screen(app: &mut App, w: u16, h: u16) -> String {
        let mut t = Terminal::new(TestBackend::new(w, h)).unwrap();
        t.draw(|f| ui(f, app)).unwrap();
        let cells: Vec<String> = t
            .backend()
            .buffer()
            .content
            .iter()
            .map(|c| c.symbol().to_string())
            .collect();
        let mut out = String::new();
        let mut skip = false;
        for sym in &cells {
            if skip {
                skip = false;
                continue;
            }
            out.push_str(sym);
            if UnicodeWidthStr::width(sym.as_str()) == 2 {
                skip = true;
            }
        }
        out
    }

    #[test]
    fn header_carries_real_provenance() {
        let mut app = App::new();
        let s = screen(&mut app, 120, 44);
        assert!(s.contains("agentMarket"), "缺少头部品牌");
        assert!(s.contains("营销仪表盘"), "缺少标题");
        assert!(s.contains("2026-05-19 实抓"), "缺少实抓出处");
        assert!(s.contains("minimax"), "缺少 minimax provenance");
        assert!(s.contains(PLUGIN), "缺少插件状态行");
    }

    #[test]
    fn channel_nav_wraps_both_ways() {
        let mut app = App::new();
        assert_eq!(app.channel(), 0);
        app.step_channel(-1);
        assert_eq!(app.channel(), 6, "从 0 向上应绕回最后一个频道");
        app.step_channel(1);
        assert_eq!(app.channel(), 0, "从最后一个向下应绕回 0");
    }

    #[test]
    fn renders_real_pitchkit_facts_not_demo() {
        let mut app = App::new();
        let s = screen(&mut app, 120, 44);
        assert!(s.contains("pitchkit"), "缺少 pitchkit 产品");
        assert!(s.contains("MIT"), "缺少真实许可证事实");
        assert!(s.contains("72 commit"), "缺少真实 commit 数（实抓）");
        // 反例哨兵：旧演示值绝不能出现。
        assert!(!s.contains("周环比 +12%"), "残留写死演示值");
        assert!(!s.contains("语调 8.4/10"), "残留写死演示值");
    }

    #[test]
    fn product_switch_changes_evidence() {
        let mut app = App::new();
        let pk = screen(&mut app, 120, 44);
        assert!(pk.contains("github.com/EricSun0218/pitchkit"));
        app.on_key(KeyCode::Char('2'), KeyModifiers::NONE);
        let ll = screen(&mut app, 120, 44);
        assert!(ll.contains("AI Love-Lab"), "切到产品②应显示 AI Love-Lab");
        assert!(ll.contains("lovelab.renlab.ai"), "缺少 AI Love-Lab 真实源");
        assert!(!ll.contains("72 commit"), "产品②不应混入产品①的事实");
    }

    #[test]
    fn perf_channel_shows_real_commit_sparkline() {
        let mut app = App::new();
        let idx = PRODUCTS[0]
            .channels
            .iter()
            .position(|c| c.skill == "performance-report")
            .unwrap();
        app.list.select(Some(idx));
        let s = screen(&mut app, 120, 44);
        assert!(s.contains("commit/天"), "效果分析频道应显示真实 commit 走势图");
        assert!(s.contains("gh api"), "走势图说明应标注真实数据来源");
    }

    #[test]
    fn lovelab_perf_has_no_fabricated_chart() {
        // AI Love-Lab 无公开数据 → 效果分析频道不得画走势图（不编造）。
        let mut app = App::new();
        app.select_product(1);
        let idx = PRODUCTS[1]
            .channels
            .iter()
            .position(|c| c.skill == "performance-report")
            .unwrap();
        app.list.select(Some(idx));
        let s = screen(&mut app, 120, 44);
        assert!(!s.contains("commit/天"), "无数据产品不得出现走势图");
        assert!(s.contains("N/A"), "应坦白无可获取数据");
    }

    #[test]
    fn nav_names_pad_to_display_width() {
        assert_eq!(pad_display("营销活动", 12), "营销活动    ");
        assert_eq!(pad_display("SEO 优化", 12), "SEO 优化    ");
    }

    #[test]
    fn sources_toggle_shows_real_provenance() {
        let mut app = App::new();
        // 收起态：面板里没有展开标记（页脚的按键提示不算）。
        assert!(!screen(&mut app, 120, 44).contains("按 s 收起"));
        app.on_key(KeyCode::Char('s'), KeyModifiers::NONE);
        let s = screen(&mut app, 120, 44);
        assert!(s.contains("按 s 收起"), "按 s 应展开数据来源面板");
        assert!(s.contains("gh api"), "pitchkit 来源应含 GitHub API 实抓");
        assert!(
            s.contains("api.minimaxi.com") || s.contains("MiniMax"),
            "应标注 minimax provenance（非 Claude OAuth）"
        );
    }

    #[test]
    fn quit_key_stops_running() {
        let mut app = App::new();
        app.on_key(KeyCode::Char('q'), KeyModifiers::NONE);
        assert!(!app.running);
    }
}
