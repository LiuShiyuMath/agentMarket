//! 真实分析数据（非演示值）。
//!
//! 本模块的每个字段都钉在 **2026-05-19 实抓** 的真实来源上：
//! pitchkit 的事实来自 GitHub API（`gh api repos/EricSun0218/pitchkit`：
//! metadata / languages / 72 commits / contributors / git trees recursive
//! / contents），AI Love-Lab 的事实来自对 `https://lovelab.renlab.ai/` 与母
//! 公司 `renlab.ai` 的 HTTP 实抓（HTML / headers / manifest / robots /
//! Googlebot-UA / `/api*` 404 探测）。每个频道的结论文本由 minimax
//! （`MiniMax-M2.7-highspeed` via `api.minimaxi.com/anthropic`，
//! `ANTHROPIC_AUTH_TOKEN` 未设置——全程未用 Claude OAuth）于 2026-05-19 生成。
//!
//! 这是两次 **各自独立** 的单产品体检，用同一条 7 频道流水线，不是 A/B 对决。

/// 一个产品的真实走势序列（仅在有真实数据时存在；无数据写 `None`，不编造）。
pub struct Series {
    pub title: &'static str,
    /// 真实数据点。
    pub data: &'static [u64],
    pub caption: &'static str,
}

/// 一个营销频道在某个产品上的真实结论（minimax 生成，2026-05-19）。
pub struct Channel {
    pub name: &'static str,
    pub skill: &'static str,
    pub verdict: &'static str,
}

/// 一次独立的单产品体检。
pub struct Product {
    pub name: &'static str,
    pub tagline: &'static str,
    pub url: &'static str,
    /// 实抓事实（标签, 值），逐行渲染在面板抬头。
    pub facts: &'static [(&'static str, &'static str)],
    /// 该产品「效果分析」频道的真实走势；无公开数据则为 `None`。
    pub series: Option<Series>,
    /// 7 个频道的真实结论，顺序与 `CHANNEL_ORDER` 对齐。
    pub channels: &'static [Channel],
    /// 这份分析钉在哪些真实来源上。
    pub sources: &'static [&'static str],
}

/// 抓取与分析的统一出处（页脚 / 抬头展示，保证可核实）。
pub const CAPTURED: &str = "2026-05-19 实抓";
pub const PROVENANCE: &str =
    "结论经 minimax · MiniMax-M2.7-highspeed @ api.minimaxi.com/anthropic · ANTHROPIC_AUTH_TOKEN=<unset> · 0 Claude OAuth";

pub const PRODUCTS: &[Product] = &[
    Product {
        name: "pitchkit",
        tagline: "\"Your repo, launch-ready\" · Local AI launch kit",
        url: "github.com/EricSun0218/pitchkit",
        facts: &[
            ("许可 / 版本", "MIT · package.json v0.3.0 · 0 release · 0 tag"),
            ("规模", "306 文件 · 1.06 MB（GitHub size 1089 KB）"),
            ("语言", "TS 796,249 B · HTML 82,331 B · Shell 10,032 B"),
            ("生命周期", "2026-05-11 建 → 2026-05-19 推（8 天）"),
            ("社区", "★2 · fork 1 · issue 0 · watch 2 · sub 0"),
            ("节奏", "72 commit / 8 天 · 主贡献者 EricSun0218 + sunrf-renlab-ai"),
            (
                "发现面",
                "GitHub topics 12 · package.json 13 keyword · homepage = pitchkit-site.vercel.app（非品牌域名）",
            ),
            (
                "自我发射",
                "2026-05-19 commit 已 dogfood：06:50 嵌入自渲染 promo → 07:22 redesign+redeploy；pitchkit-site.vercel.app HTTP 200 引用 pitchkit-promo.mp4",
            ),
            (
                "名称撞车",
                "pitchkit.net HTTP 200 = \"Pitchkit | Pitch Deck Builder with VC Scoring\"（无关 SaaS，品牌搜索被稀释）",
            ),
        ],
        series: Some(Series {
            title: " commit/天 · 2026-05-11→19（真实，gh api）",
            // 真实：gh api commits 按 author.date 分日计数（05-16/17 为 0）。
            data: &[23, 3, 10, 3, 1, 0, 0, 17, 15],
            caption: "源：gh api repos/EricSun0218/pitchkit/commits --paginate · 合计 72 · 非写死演示值",
        }),
        channels: &[
            Channel {
                name: "营销活动",
                skill: "campaign-plan",
                verdict: "面向 Claude Code/Cursor 用户群，建站 8 天即开启自我发射——commit 显示 07:22 自跑 promo 视频 redesign，06:50 将自渲染成品嵌回官网，完成首次 dogfood 闭环。目标受众明确为本地 AI 开发者，渠道走 GitHub + Vercel 官网，暂无邮件或社区基建，推广依赖 GitHub topics 铺量。",
            },
            Channel {
                name: "内容创作",
                skill: "draft-content / content-creation",
                verdict: "tagline \"Your repo, launch-ready\" 已落地，site-context.json 叙事骨架完整（problem/solution/why_now 三段式），三件套（promo 视频 + pitch deck + 落地页）已跑通。颜色 token 支持 cursorDark/anthropicWarm/vercelNeon 三套调色板，适配不同 AI 工具品牌调性。中文 CONTRIBUTING.md 与英文 README 并存，国际化与本地化均有痕迹。",
            },
            Channel {
                name: "SEO 优化",
                skill: "seo-audit",
                verdict: "GitHub topics 填满 12 个、package.json 含 13 个 keyword，关键词密度充分。硬伤：homepage 跑在 vercel.app 非品牌域名，搜索心智被 pitchkit.net（同名竞品 SaaS）稀释。commit 06:33 记录了自我 SEO 审计并落地优化，说明已具备 SEO 意识但执行路径较新。",
            },
            Channel {
                name: "效果分析",
                skill: "performance-report",
                verdict: "★2 fork1 issue0 watch2 ——冷启动数据，无参考价值。commit 频率（8 天 72 次，左侧迷你图为真实 commit/天）反证团队活跃度远高于 star 数字所现。真实 promo 视频已上线且嵌回官网，流量数据未公开，转化路径无法评估。建议以 commit 频率和自我 dogfood 完成度替代 star 类指标。",
            },
            Channel {
                name: "品牌审查",
                skill: "brand-review",
                verdict: "MIT 协议完全开源，品牌完全靠 GitHub 展示而非官网构建。配色体系三套（cursorDark/anthropicWarm/vercelNeon）说明尚未固化自有品牌色。tagline \"Your repo, launch-ready\" 定位清晰，但与同名竞品 pitchkit.net 存在心智冲突，需尽快启用品牌域名隔离。",
            },
            Channel {
                name: "竞品分析",
                skill: "competitive-brief",
                verdict: "定位\"零云 AI、零 API key\"的本地 launch kit，在 Claude Code/Cursor 生态中暂无直接对标。差异化落在 script-driven 方法论（19 套 promo 脚本原型 + 98 个真实视频拆解）和 HTML-native 场景原子（38 scene atoms）。GitHub topics 覆盖 ai/claude-code/developer-tools 等 12 个标签，入口布局完整。",
            },
            Channel {
                name: "邮件序列",
                skill: "email-sequence",
                verdict: "N/A — 产品无邮件订阅、CTA 或 onboarding 序列，暂无邮件基建规划。",
            },
        ],
        sources: &[
            "GitHub API：gh api repos/EricSun0218/pitchkit（metadata · languages · 72 commits --paginate · contributors · git/trees recursive=306 blobs · contents: package.json / docs/pitchkit-site-context.json / shared/tokens.ts / CONTRIBUTING.md / README）",
            "自我发射实证：git log 2026-05-19 — 06:50 embed self-rendered promo · 07:22 redesign+redeploy · 08:07 self-review-and-iterate；pitchkit-site.vercel.app HTTP 200（Vercel, 引用 pitchkit-promo.mp4）",
            "名称撞车：pitchkit.net HTTP 200 — \"Pitchkit | Pitch Deck Builder with VC Scoring\"（独立无关 SaaS）",
        ],
    },
    Product {
        name: "AI Love-Lab（心动模拟器）",
        tagline: "\"在沙盘中推演那段你不敢真实开口的关系。\" · zh-CN PWA",
        url: "lovelab.renlab.ai",
        facts: &[
            (
                "技术栈",
                "Next.js App Router (RSC) on Netlify Edge · cache Netlify Edge hit",
            ),
            (
                "形态",
                "zh-CN PWA · 竖屏移动优先（display standalone / orientation portrait）",
            ),
            ("页面", "HTML 25,523 B · theme #1a1a2e · manifest bg #0b1230"),
            (
                "robots 实情",
                "无 robots meta（Googlebot UA 亦无）· robots.txt 404 · 无 X-Robots-Tag → 可抓取但零 SEO 投入（非 noindex）",
            ),
            (
                "API 面",
                "/api · /api/health · /api/chat · /api/session 全 404（无公开端点）",
            ),
            (
                "母公司",
                "renlab.ai HTTP 200 · \"Social Intelligence · Vol.01\" · /directions 200",
            ),
            (
                "定位词",
                "不打分 · 不预测 yes/no · 你看见的不是答案，是一种可能 · 推演一下 520",
            ),
        ],
        // 无公开分析脚本 / 无可获取流量数据——不编造走势图。
        series: None,
        channels: &[
            Channel {
                name: "营销活动",
                skill: "campaign-plan",
                verdict: "以\"520\"为营销支点，主打\"推演不敢真实开口的关系\"情感定位，切移动端竖屏 PWA 场景。soft launch 姿态明显：robots.txt 404、无 SEO 投入、无公开 API，推广依赖口碑与社群而非搜索流量。母公司 renlab.ai 站台背书，提供技术可信度。",
            },
            Channel {
                name: "内容创作",
                skill: "draft-content / content-creation",
                verdict: "中文沉浸式叙事——\"每答一题 agent 变丰富\"、\"你看见的不是答案，是一种可能\"，语气克制且富有悬念感，适合情感类内容传播。落地页无英文版本，完全服务华语用户。\"AI 生成内容·仅供娱乐\"免责声明已嵌入多处文案，符合监管预期。",
            },
            Channel {
                name: "SEO 优化",
                skill: "seo-audit",
                verdict: "robots meta 和 X-Robots-Tag 均无，robots.txt 404，技术上可被抓取但无任何 SEO 优化动作。页面 title/meta description 存在，关键词未做布局。判断：产品处于软发布期，不追求搜索流量，当前 SEO 状态属于\"未投入\"而非\"主动屏蔽\"。",
            },
            Channel {
                name: "效果分析",
                skill: "performance-report",
                verdict: "N/A — 页面无公开分析脚本，无可获取的流量或转化数据。Netlify Edge hit + Next.js 缓存头仅证明 CDN 部署正常，不代表真实访问量。需接入独立分析工具方可评估。（故本频道左侧不画走势图——无真实数据不编造。）",
            },
            Channel {
                name: "品牌审查",
                skill: "brand-review",
                verdict: "\"心动模拟器·AI Love-Lab\"双命名，lang zh-CN，theme-color #1a1a2e / background #0b1230 暗色调，符合浪漫/神秘情感调性。display standalone + orientation portrait 说明竖屏移动优先。品牌词\"仅供娱乐\"在文案重复出现三次，风控意识充分。",
            },
            Channel {
                name: "竞品分析",
                skill: "competitive-brief",
                verdict: "关系模拟/沙盘推演品类在中文互联网无明显直接竞品。\"AI 生成内容\"定位排除了与真实情感咨询的边界，差异化落在\"不打分·不预测 yes/no\"的不确定性叙事。母公司 renlab.ai 的 og 描述（\"A flight simulator for organizational decisions\"）暗示底层技术可跨场景复用。",
            },
            Channel {
                name: "邮件序列",
                skill: "email-sequence",
                verdict: "N/A — /api/health 等所有端点返回 404，无邮件订阅、触发序列或 onboarding 邮件。当前产品阶段尚未到邮件基建铺设期。",
            },
        ],
        sources: &[
            "HTTP 实抓 https://lovelab.renlab.ai/ — HTML / headers / manifest.json / robots.txt(404) / X-Robots-Tag(无) / Googlebot-UA 抓取 / /api* 404 探测",
            "母公司 renlab.ai 首页 HTTP 200 + /directions 200 · og \"A flight simulator for organizational decisions\"",
        ],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_independent_products() {
        assert_eq!(PRODUCTS.len(), 2, "本页是两次独立单产品体检");
        assert_eq!(PRODUCTS[0].name, "pitchkit");
        assert!(PRODUCTS[1].name.contains("AI Love-Lab"));
    }

    #[test]
    fn every_product_has_seven_channels() {
        for p in PRODUCTS {
            assert_eq!(p.channels.len(), 7, "{} 应有 7 个频道", p.name);
        }
    }

    #[test]
    fn real_series_only_when_data_exists() {
        // pitchkit 有真实 commit/天 序列；AI Love-Lab 无公开数据 → None，不编造。
        let pk = &PRODUCTS[0];
        let s = pk.series.as_ref().expect("pitchkit 应有真实 commit 序列");
        assert_eq!(s.data.iter().sum::<u64>(), 72, "真实 commit 合计应为 72");
        assert!(PRODUCTS[1].series.is_none(), "无数据产品不得有走势图");
    }

    #[test]
    fn facts_are_real_not_demo_placeholders() {
        // 反例哨兵：旧演示值（"周环比 +12%"、"语调 8.4/10"）不得出现在真实数据里。
        for p in PRODUCTS {
            for (_, v) in p.facts {
                assert!(!v.contains("周环比 +12%"), "{} 残留演示值", p.name);
                assert!(!v.contains("语调 8.4/10"), "{} 残留演示值", p.name);
            }
        }
        assert!(PRODUCTS[0].facts.iter().any(|(_, v)| v.contains("MIT")));
        assert!(PRODUCTS[0]
            .facts
            .iter()
            .any(|(_, v)| v.contains("72 commit")));
    }
}
