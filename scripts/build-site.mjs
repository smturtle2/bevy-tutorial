import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { marked } from "marked";
import { markedHighlight } from "marked-highlight";
import hljs from "highlight.js";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");
const dist = path.join(root, "dist");
const docs = path.join(root, "docs");
const assets = path.join(root, "assets");
const repoUrl = "https://github.com/smturtle2/bevy-tutorial";
const base = normalizeBase(process.env.PAGES_BASE ?? "/bevy-tutorial/");

const chapters = [
  chapter(0, "00-project-setup", "Project Setup", "프로젝트 설정", "Foundations", "준비", "cargo check", "Install Rust, create the Bevy project, and verify the toolchain.", "Rust와 Bevy 프로젝트를 준비하고 빌드 가능한 상태를 확인합니다."),
  chapter(1, "01-rust-for-bevy", "Rust For Bevy", "Bevy를 위한 Rust", "Foundations", "기초", "cargo run --example 01_empty_app", "Learn the Rust syntax that appears immediately in Bevy systems.", "Bevy 시스템을 읽기 위해 바로 필요한 Rust 문법을 배웁니다."),
  chapter(2, "02-bevy-app-model", "The Bevy App Model", "Bevy 앱 모델", "Foundations", "기초", "cargo run --example 01_empty_app", "Read App, plugins, startup systems, update systems, and insertion points.", "App, 플러그인, startup/update 시스템, insert 흐름을 읽습니다."),
  chapter(3, "03-ecs-fundamentals", "ECS Fundamentals", "ECS 기본", "ECS", "ECS", "cargo run --example 02_spawn_sprite", "Separate entity identity, component data, resources, systems, and queries.", "엔티티, 컴포넌트, 리소스, 시스템, 쿼리의 책임을 분리합니다."),
  chapter(4, "04-input-and-movement", "Input And Movement", "입력과 이동", "ECS", "ECS", "cargo run --example 03_player_input", "Turn keyboard input into velocity and transform changes.", "키보드 입력을 속도와 위치 변경으로 연결합니다."),
  chapter(5, "05-bundles-plugins-sets", "Bundles, Plugins, And Sets", "번들, 플러그인, 세트", "Architecture", "구조", "cargo run --example 05_plugins_sets", "Group spawn data, feature registration, and execution order.", "생성 데이터, 기능 등록, 실행 순서를 책임별로 묶습니다."),
  chapter(6, "06-assets-camera-ui", "Assets, Camera, And UI", "에셋, 카메라, UI", "Presentation", "표현", "cargo run --example 06_assets_camera_ui", "Load sprites, add cameras, and place world/UI text.", "스프라이트를 로드하고 카메라와 월드/UI 텍스트를 배치합니다."),
  chapter(7, "07-rpg-slice", "RPG Foundation Slice", "RPG 기초 조각", "Playable Slice", "플레이 가능한 조각", "cargo run --example 07_rpg_slice", "Combine movement, collision, AI, collection, score, and HUD.", "이동, 충돌, AI, 수집, 점수, HUD를 하나의 루프로 묶습니다."),
  chapter(8, "08-smooth-camera-follow", "Smooth Camera Follow", "부드러운 카메라 추적", "RPG Features", "RPG 기능", "cargo run --example 08_smooth_camera_follow", "Follow the player smoothly without snapping the camera every frame.", "프레임마다 튀지 않는 부드러운 카메라 추적을 만듭니다."),
  chapter(9, "09-enemy-waves", "Enemy Waves", "적 웨이브", "RPG Features", "RPG 기능", "cargo run --example 09_enemy_waves", "Spawn enemies over time with explicit wave state.", "명시적인 웨이브 상태로 적을 시간에 따라 생성합니다."),
  chapter(10, "10-attack-hitbox", "Attack Hitboxes", "공격 히트박스", "RPG Features", "RPG 기능", "cargo run --example 10_attack_hitbox", "Represent attacks as short-lived hitbox entities.", "공격을 짧게 살아 있는 hitbox 엔티티로 표현합니다."),
  chapter(11, "11-sprite-assets", "Sprite Assets", "스프라이트 에셋", "RPG Features", "RPG 기능", "cargo run --example 11_sprite_assets", "Replace colored shapes with real tutorial sprites.", "색상 도형을 실제 튜토리얼 스프라이트로 바꿉니다."),
  chapter(12, "12-screen-space-ui", "Screen-Space UI", "화면 고정 UI", "RPG Features", "RPG 기능", "cargo run --example 12_screen_space_ui", "Keep HUD elements fixed to the screen instead of the world.", "HUD 요소를 월드가 아니라 화면에 고정합니다."),
  chapter(13, "13-animation-state", "Animation State", "애니메이션 상태", "RPG Features", "RPG 기능", "cargo run --example 13_animation_state", "Drive sprite frames from movement and gameplay state.", "이동과 게임플레이 상태로 스프라이트 프레임을 제어합니다."),
  chapter(14, "14-handmade-map-geometry", "Handmade Map Geometry", "직접 만든 맵 지오메트리", "RPG Features", "RPG 기능", "cargo run --example 14_handmade_map_geometry", "Build a small map from explicit geometry and collision bounds.", "명시적인 지오메트리와 충돌 범위로 작은 맵을 만듭니다."),
  chapter(15, "15-game-states", "Game States", "게임 상태", "RPG Features", "RPG 기능", "cargo run --example 15_game_states", "Model menu, playing, paused, and game over as Bevy states.", "메뉴, 플레이, 일시정지, 게임 오버를 Bevy 상태로 모델링합니다."),
  chapter(16, "16-save-load-progress", "Save And Load Progress", "진행도 저장과 불러오기", "RPG Features", "RPG 기능", "cargo run --example 16_save_load_progress", "Persist explicit progress data instead of serializing the whole world.", "전체 월드가 아니라 명시적인 진행 데이터를 저장합니다."),
  chapter(17, "17-complete-rpg-slice", "Complete RPG Slice", "완성된 RPG 조각", "RPG Features", "RPG 기능", "cargo run --example 17_complete_rpg_slice", "Put the tutorial systems together into a complete small RPG slice.", "튜토리얼 시스템을 합쳐 완성된 작은 RPG 조각을 만듭니다."),
];

const chapterThumbnails = {
  "00-project-setup": "diagrams/rpg-feature-roadmap.png",
  "01-rust-for-bevy": "diagrams/ecs-overview.png",
  "02-bevy-app-model": "screenshots/ch02-spawn-sprite.png",
  "03-ecs-fundamentals": "diagrams/ecs-overview.png",
  "04-input-and-movement": "screenshots/ch04-velocity-body.png",
  "05-bundles-plugins-sets": "screenshots/ch05-plugins-sets.png",
  "06-assets-camera-ui": "screenshots/ch06-assets-camera-ui.png",
  "07-rpg-slice": "screenshots/ch07-rpg-slice.png",
  "08-smooth-camera-follow": "screenshots/ch08-smooth-camera-follow.png",
  "09-enemy-waves": "screenshots/ch09-enemy-waves.png",
  "10-attack-hitbox": "screenshots/ch10-attack-hitbox.png",
  "11-sprite-assets": "screenshots/ch11-sprite-assets.png",
  "12-screen-space-ui": "screenshots/ch12-screen-space-ui.png",
  "13-animation-state": "screenshots/ch13-animation-attack.png",
  "14-handmade-map-geometry": "screenshots/ch14-handmade-map.png",
  "15-game-states": "screenshots/ch15-game-state-menu.png",
  "16-save-load-progress": "screenshots/ch16-save-load-progress.png",
  "17-complete-rpg-slice": "screenshots/ch17-complete-rpg-gameplay.png",
};

const copy = {
  en: {
    htmlLang: "en",
    siteTitle: "Rust + Bevy Tutorial",
    navChapters: "Chapters",
    navExamples: "Examples",
    navGithub: "GitHub",
    start: "Start learning",
    viewExamples: "View examples",
    heroTitle: "<strong>Rust</strong> + Bevy Tutorial",
    heroLead: "Learn Rust through a playable Bevy RPG slice. This is a community-made path for learners who want practical code, clear contracts, and runnable examples.",
    heroPoints: [
      "Rust basics are introduced exactly when Bevy code needs them.",
      "Every major RPG system has a chapter and a runnable example.",
      "Architecture grows from systems to bundles, plugins, states, and saving.",
    ],
    learningPath: "Your learning path",
    learningPathNote: "Read in order or jump to the chapter that matches the system you are building.",
    examplesTitle: "Runnable examples",
    examplesNote: "Each command compiles independently and mirrors one tutorial step.",
    communityTitle: "This tutorial is community-made.",
    communityText: "It is written for learners who want to inspect real Bevy code and change it. Issues, corrections, and improvements are welcome.",
    contribute: "Contribute on GitHub",
    runThis: "Run this chapter",
    onThisPage: "On this page",
    allChapters: "All chapters",
    previous: "Previous",
    next: "Next",
    home: "Home",
    chapter: "Chapter",
    versions: "Versions",
    footerText: "A community tutorial for learning Rust and Bevy by building a small RPG.",
    source: "Source code",
    issues: "Issues",
    license: "MIT License",
  },
  ko: {
    htmlLang: "ko",
    siteTitle: "Rust + Bevy 튜토리얼",
    navChapters: "목차",
    navExamples: "예제",
    navGithub: "GitHub",
    start: "학습 시작",
    viewExamples: "예제 보기",
    heroTitle: "<strong>Rust</strong> + Bevy 튜토리얼",
    heroLead: "플레이 가능한 Bevy RPG 조각을 만들면서 Rust를 배웁니다. 실용 코드, 명확한 계약, 실행 가능한 예제를 원하는 학습자를 위한 커뮤니티 제작 튜토리얼입니다.",
    heroPoints: [
      "Bevy 코드에 필요한 Rust 기초를 등장 순서에 맞춰 설명합니다.",
      "핵심 RPG 시스템마다 장과 실행 가능한 예제가 있습니다.",
      "시스템에서 번들, 플러그인, 상태, 저장까지 구조를 단계적으로 키웁니다.",
    ],
    learningPath: "학습 경로",
    learningPathNote: "순서대로 읽어도 되고, 지금 만들고 싶은 시스템에 맞는 장으로 이동해도 됩니다.",
    examplesTitle: "실행 가능한 예제",
    examplesNote: "각 명령은 독립적으로 컴파일되며 튜토리얼 단계를 그대로 반영합니다.",
    communityTitle: "이 튜토리얼은 커뮤니티 제작 문서입니다.",
    communityText: "실제 Bevy 코드를 읽고 직접 바꿔보고 싶은 학습자를 위해 작성했습니다. 이슈, 수정, 개선 제안을 환영합니다.",
    contribute: "GitHub에서 기여하기",
    runThis: "이 장 실행",
    onThisPage: "이 페이지",
    allChapters: "전체 목차",
    previous: "이전",
    next: "다음",
    home: "홈",
    chapter: "장",
    versions: "버전",
    footerText: "작은 RPG를 만들면서 Rust와 Bevy를 배우는 커뮤니티 튜토리얼입니다.",
    source: "소스 코드",
    issues: "이슈",
    license: "MIT License",
  },
};

marked.use(markedHighlight({
  emptyLangClass: "hljs",
  langPrefix: "hljs language-",
  highlight(code, lang) {
    const language = hljs.getLanguage(lang) ? lang : "plaintext";
    return hljs.highlight(code, { language }).value;
  },
}));

main();

function main() {
  assertDist();
  copyTutorialAssets();
  const assetTags = readViteAssetTags();

  writeFile("index.html", renderRootRedirect(assetTags));
  writeFile("404.html", renderNotFound(assetTags));

  for (const lang of ["en", "ko"]) {
    writeFile(`${lang}/index.html`, renderHome(lang, assetTags));

    for (const current of chapters) {
      writeFile(`${lang}/${current.slug}/index.html`, renderChapter(lang, current, assetTags));
    }
  }

  writeFile(".nojekyll", "");
}

function chapter(number, slug, enTitle, koTitle, phaseEn, phaseKo, command, enSummary, koSummary) {
  return {
    number,
    slug,
    title: { en: enTitle, ko: koTitle },
    phase: { en: phaseEn, ko: phaseKo },
    command,
    summary: { en: enSummary, ko: koSummary },
  };
}

function normalizeBase(value) {
  if (!value.startsWith("/")) value = `/${value}`;
  return value.endsWith("/") ? value : `${value}/`;
}

function assertDist() {
  const index = path.join(dist, "index.html");
  if (!fs.existsSync(index)) {
    throw new Error("dist/index.html is missing. Run vite build before scripts/build-site.mjs.");
  }
}

function copyTutorialAssets() {
  fs.cpSync(assets, path.join(dist, "assets"), { recursive: true, force: true });
}

function readViteAssetTags() {
  const shell = fs.readFileSync(path.join(dist, "index.html"), "utf8");
  const matches = shell.match(/<(?:script|link)\b[^>]*(?:src|href)="[^"]*\/assets\/[^"]+"[^>]*>(?:<\/script>)?/g);
  return matches ? matches.join("\n") : "";
}

function renderRootRedirect(assetTags) {
  return htmlDocument({
    lang: "en",
    title: "Rust + Bevy Tutorial",
    description: "Redirecting to the English Rust + Bevy tutorial.",
    assetTags,
    body: `
      <main class="home">
        <section class="hero">
          <div>
            <h1><strong>Rust</strong> + Bevy Tutorial</h1>
            <p class="hero-lede">Redirecting to the English tutorial.</p>
            <p><a class="button primary" href="${base}en/">Open /en/</a></p>
          </div>
        </section>
      </main>
      <script>window.location.replace("${base}en/");</script>
    `,
    redirect: `${base}en/`,
  });
}

function renderNotFound(assetTags) {
  return htmlDocument({
    lang: "en",
    title: "Page not found - Rust + Bevy Tutorial",
    description: "Page not found.",
    assetTags,
    body: `
      ${renderHeader("en", null)}
      <main class="home">
        <section class="hero">
          <div>
            <h1>Page not found</h1>
            <p class="hero-lede">The tutorial page you requested does not exist.</p>
            <p><a class="button primary" href="${base}en/">Go to /en/</a></p>
          </div>
        </section>
      </main>
      ${renderFooter("en")}
    `,
  });
}

function renderHome(lang, assetTags) {
  const t = copy[lang];
  const heroImage = `${base}assets/screenshots/ch17-complete-rpg-gameplay.png`;
  const features = [
    [asset("player.png"), lang === "en" ? "Hands-on learning" : "직접 만드는 학습", lang === "en" ? "Build an RPG step by step with complete examples." : "완성 예제로 RPG를 단계별로 만듭니다."],
    [asset("enemy.png"), lang === "en" ? "Bevy patterns" : "Bevy 패턴", lang === "en" ? "Learn ECS, plugins, states, assets, UI, and saving." : "ECS, 플러그인, 상태, 에셋, UI, 저장을 배웁니다."],
    [asset("slash.png"), lang === "en" ? "Rust in context" : "맥락 속 Rust", lang === "en" ? "Rust concepts are tied directly to Bevy code." : "Rust 개념을 Bevy 코드와 바로 연결합니다."],
    [asset("gem.png"), lang === "en" ? "Community-shaped" : "커뮤니티 문서", lang === "en" ? "Practical, inspectable, and open to corrections." : "실용적이고 검토 가능하며 개선을 받습니다."],
  ];

  const body = `
    ${renderHeader(lang, null)}
    <main id="content" class="main home">
      <section class="hero">
        <div>
          <h1>${t.heroTitle}</h1>
          <p class="hero-lede">${escapeHtml(t.heroLead)}</p>
          <ul class="hero-points">
            ${t.heroPoints.map((point) => `<li>${escapeHtml(point)}</li>`).join("")}
          </ul>
          <div class="actions">
            <a class="button primary" href="${pageUrl(lang, chapters[0].slug)}">${escapeHtml(t.start)}</a>
            <a class="button" href="#examples">${escapeHtml(t.viewExamples)}</a>
          </div>
          <div class="version-row">
            <span class="chip rust">Rust 2024</span>
            <span class="chip bevy">Bevy 0.18.1</span>
            <span class="chip">18 chapters</span>
          </div>
        </div>
        <div class="preview-panel" aria-label="RPG tutorial preview">
          <img src="${heroImage}" alt="Complete RPG tutorial preview" />
          <div class="preview-hud">
            <span>HP 28/28 <i class="hud-bar" style="--value: 92%"></i></span>
            <span>LV 3 <i class="hud-bar" style="--value: 62%"></i></span>
            <span>EXP 40/120 <i class="hud-bar" style="--value: 36%"></i></span>
          </div>
        </div>
      </section>

      <section class="section" aria-labelledby="features-title">
        <div class="feature-strip">
          ${features.map(([image, title, text]) => `
            <article class="feature">
              <img src="${image}" alt="" />
              <h3>${escapeHtml(title)}</h3>
              <p>${escapeHtml(text)}</p>
            </article>
          `).join("")}
        </div>
      </section>

      <section id="chapters" class="section" aria-labelledby="chapters-title">
        <div class="section-head">
          <div>
            <h2 id="chapters-title">${escapeHtml(t.learningPath)}</h2>
            <p class="section-note">${escapeHtml(t.learningPathNote)}</p>
          </div>
        </div>
        <div class="path-grid">
          ${chapters.map((item) => renderChapterCard(lang, item)).join("")}
        </div>
      </section>

      <section id="examples" class="section" aria-labelledby="examples-title">
        <div class="section-head">
          <div>
            <h2 id="examples-title">${escapeHtml(t.examplesTitle)}</h2>
            <p class="section-note">${escapeHtml(t.examplesNote)}</p>
          </div>
        </div>
        <div class="examples-grid">
          ${chapters.slice(1).map((item) => renderExampleRow(lang, item)).join("")}
        </div>
      </section>

      <section class="section community-note">
        <img src="${asset("player.png")}" alt="" />
        <div>
          <h2>${escapeHtml(t.communityTitle)}</h2>
          <p>${escapeHtml(t.communityText)}</p>
        </div>
        <a class="button" href="${repoUrl}">${escapeHtml(t.contribute)}</a>
      </section>
    </main>
    ${renderFooter(lang)}
  `;

  return htmlDocument({
    lang: t.htmlLang,
    title: t.siteTitle,
    description: stripTags(t.heroLead),
    assetTags,
    body,
  });
}

function renderChapter(lang, current, assetTags) {
  const t = copy[lang];
  const markdownPath = path.join(docs, lang, `${current.slug}.md`);
  const markdown = cleanMarkdown(fs.readFileSync(markdownPath, "utf8"), lang);
  const { html, toc } = renderMarkdown(markdown, lang);
  const index = chapters.findIndex((item) => item.slug === current.slug);
  const previous = chapters[index - 1] ?? null;
  const next = chapters[index + 1] ?? null;

  const body = `
    ${renderHeader(lang, current.slug)}
    <main id="content" class="main doc-layout">
      <aside class="chapter-sidebar" aria-label="${escapeHtml(t.allChapters)}">
        ${renderChapterSidebar(lang, current.slug)}
      </aside>
      <section class="doc-main">
        <nav class="breadcrumbs" aria-label="Breadcrumb">
          <a href="${pageUrl(lang)}">${escapeHtml(t.home)}</a>
          <span>/</span>
          <a href="${pageUrl(lang)}#chapters">${escapeHtml(t.navChapters)}</a>
          <span>/</span>
          <span>${pad(current.number)}. ${escapeHtml(current.title[lang])}</span>
        </nav>
        <header class="article-header">
          <div class="chapter-kicker">${escapeHtml(t.chapter)} ${pad(current.number)}</div>
          <h1>${escapeHtml(current.title[lang])}</h1>
          <p class="article-summary">${escapeHtml(current.summary[lang])}</p>
          <div class="chapter-meta">
            <span class="chip rust">Rust 2024</span>
            <span class="chip bevy">Bevy 0.18.1</span>
            <span class="chip">${escapeHtml(current.phase[lang])}</span>
          </div>
          <div class="run-command" style="margin-top: 14px">
            <span>${escapeHtml(t.runThis)}</span>
            <code>${escapeHtml(current.command)}</code>
          </div>
        </header>
        <details class="mobile-chapters">
          <summary>${escapeHtml(t.allChapters)}</summary>
          ${renderChapterList(lang, current.slug)}
        </details>
        <article class="article">
          ${html}
        </article>
        ${renderPrevNext(lang, previous, next)}
      </section>
      <aside class="page-toc">
        <div class="side-panel">
          <div class="side-panel__title">${escapeHtml(t.onThisPage)}</div>
          <nav class="toc-list">
            ${toc.length > 0 ? toc.map((item) => `<a data-toc-link data-depth="${item.depth}" href="#${item.id}">${escapeHtml(item.text)}</a>`).join("") : `<a href="#content">${escapeHtml(current.title[lang])}</a>`}
          </nav>
        </div>
        <div class="side-panel">
          <div class="side-panel__title">${escapeHtml(t.versions)}</div>
          <div class="toc-list">
            <span class="chip rust">Rust 2024</span>
            <span class="chip bevy">Bevy 0.18.1</span>
          </div>
        </div>
      </aside>
    </main>
    ${renderFooter(lang)}
  `;

  return htmlDocument({
    lang: t.htmlLang,
    title: `${current.title[lang]} - ${t.siteTitle}`,
    description: current.summary[lang],
    assetTags,
    body,
  });
}

function renderHeader(lang, currentSlug) {
  const t = copy[lang];
  const enPath = currentSlug ? pageUrl("en", currentSlug) : pageUrl("en");
  const koPath = currentSlug ? pageUrl("ko", currentSlug) : pageUrl("ko");

  return `
    <a class="skip-link" href="#content">Skip to content</a>
    <header class="site-header">
      <div class="header-inner">
        <a class="brand" href="${pageUrl(lang)}" aria-label="${escapeHtml(t.siteTitle)}">
          <span class="brand-mark">B</span>
          <span>${escapeHtml(t.siteTitle)}</span>
        </a>
        <nav class="header-nav" aria-label="Primary">
          <a href="${pageUrl(lang)}#chapters">${escapeHtml(t.navChapters)}</a>
          <a href="${pageUrl(lang)}#examples">${escapeHtml(t.navExamples)}</a>
          <a href="${repoUrl}">${escapeHtml(t.navGithub)}</a>
        </nav>
        <div class="language-switch" aria-label="Language">
          <a href="${enPath}" aria-current="${lang === "en"}">EN</a>
          <span>/</span>
          <a href="${koPath}" aria-current="${lang === "ko"}">KO</a>
        </div>
        <a class="github-link" href="${repoUrl}" aria-label="GitHub">GH</a>
      </div>
      <div class="read-progress"><span class="read-progress__bar" data-reading-progress></span></div>
    </header>
  `;
}

function renderFooter(lang) {
  const t = copy[lang];
  return `
    <footer class="site-footer">
      <div class="footer-inner">
        <div>
          <h2>${escapeHtml(t.siteTitle)}</h2>
          <p>${escapeHtml(t.footerText)}</p>
        </div>
        <div>
          <h3>${escapeHtml(t.navChapters)}</h3>
          <div class="footer-links">
            <a href="${pageUrl(lang)}#chapters">${escapeHtml(t.learningPath)}</a>
            <a href="${pageUrl(lang, chapters[0].slug)}">${escapeHtml(chapters[0].title[lang])}</a>
          </div>
        </div>
        <div>
          <h3>${escapeHtml(t.navExamples)}</h3>
          <div class="footer-links">
            <a href="${pageUrl(lang)}#examples">${escapeHtml(t.examplesTitle)}</a>
            <a href="${pageUrl(lang, chapters[17].slug)}">${escapeHtml(chapters[17].title[lang])}</a>
          </div>
        </div>
        <div>
          <h3>${escapeHtml(t.navGithub)}</h3>
          <div class="footer-links">
            <a href="${repoUrl}">${escapeHtml(t.source)}</a>
            <a href="${repoUrl}/issues">${escapeHtml(t.issues)}</a>
            <a href="${repoUrl}/blob/main/LICENSE">${escapeHtml(t.license)}</a>
          </div>
        </div>
      </div>
    </footer>
  `;
}

function renderChapterSidebar(lang, currentSlug) {
  const t = copy[lang];
  return `
    <div class="side-panel">
      <div class="side-panel__title">${escapeHtml(t.allChapters)}</div>
      ${renderChapterList(lang, currentSlug)}
    </div>
  `;
}

function renderChapterList(lang, currentSlug) {
  return `<nav class="chapter-list">${chapters.map((item) => `
    <a class="chapter-link" href="${pageUrl(lang, item.slug)}" ${item.slug === currentSlug ? `aria-current="page"` : ""}>
      <span class="chapter-link__number">${pad(item.number)}</span>
      <span>${escapeHtml(item.title[lang])}</span>
    </a>
  `).join("")}</nav>`;
}

function renderChapterCard(lang, item) {
  return `
    <a class="chapter-card" href="${pageUrl(lang, item.slug)}">
      <img class="chapter-card__image" src="${chapterThumbnail(item)}" alt="" />
      <span class="chapter-card__body">
        <span class="chapter-card__number">${pad(item.number)}</span>
        <h3>${escapeHtml(item.title[lang])}</h3>
        <p>${escapeHtml(item.summary[lang])}</p>
        <small>${escapeHtml(item.phase[lang])}</small>
      </span>
    </a>
  `;
}

function renderExampleRow(lang, item) {
  return `
    <a class="example-row" href="${pageUrl(lang, item.slug)}">
      <img src="${chapterThumbnail(item)}" alt="" />
      <span>
        <strong>${pad(item.number)}. ${escapeHtml(item.title[lang])}</strong><br />
        <code>${escapeHtml(item.command)}</code>
      </span>
    </a>
  `;
}

function renderPrevNext(lang, previous, next) {
  const t = copy[lang];
  return `
    <nav class="prev-next" aria-label="Chapter navigation">
      ${previous ? `<a href="${pageUrl(lang, previous.slug)}"><span>${escapeHtml(t.previous)}</span><strong>${pad(previous.number)}. ${escapeHtml(previous.title[lang])}</strong></a>` : `<a href="${pageUrl(lang)}"><span>${escapeHtml(t.previous)}</span><strong>${escapeHtml(t.home)}</strong></a>`}
      ${next ? `<a href="${pageUrl(lang, next.slug)}"><span>${escapeHtml(t.next)}</span><strong>${pad(next.number)}. ${escapeHtml(next.title[lang])}</strong></a>` : `<a href="${repoUrl}"><span>${escapeHtml(t.next)}</span><strong>${escapeHtml(t.contribute)}</strong></a>`}
    </nav>
  `;
}

function renderMarkdown(markdown, lang) {
  let html = marked.parse(markdown, { async: false });
  html = normalizeLinks(html, lang);
  html = wrapFigures(html);
  const toc = [];
  html = addHeadingAnchors(html, toc);
  html = wrapCodeBlocks(html);
  return { html, toc };
}

function cleanMarkdown(markdown, lang) {
  return preprocessAlerts(markdown, lang)
    .replace(/\r\n/g, "\n")
    .replace(/^# .+\n+/, "")
    .replace(/\n?<div align="center">[\s\S]*?<\/div>\n?/g, "\n")
    .replace(/^\s*---\s*/, "")
    .replace(/\n---\s*$/g, "")
    .trim();
}

function preprocessAlerts(markdown, lang) {
  const labels = {
    en: { IMPORTANT: "Important", TIP: "Tip", NOTE: "Note", WARNING: "Warning" },
    ko: { IMPORTANT: "중요", TIP: "팁", NOTE: "노트", WARNING: "주의" },
  };

  return markdown.replace(/^> \[!(IMPORTANT|TIP|NOTE|WARNING)\]\n((?:>.*(?:\n|$))+)/gm, (_, type, body) => {
    const text = body
      .split("\n")
      .map((line) => line.replace(/^> ?/, ""))
      .join("\n")
      .trim();
    return `<div class="callout callout--${type.toLowerCase()}">\n\n**${labels[lang][type]}**\n\n${text}\n\n</div>\n`;
  });
}

function normalizeLinks(html, lang) {
  return html
    .replace(/src="(?:\.\.\/)+assets\//g, `src="${base}assets/`)
    .replace(/href="index\.md(#[^"]*)?"/g, (_, hash = "") => `href="${pageUrl(lang)}${hash}"`)
    .replace(/href="([^":#]+)\.md(#[^"]*)?"/g, (_, href, hash = "") => {
      const slug = path.basename(href);
      const chapter = chapters.find((item) => item.slug === slug);
      return chapter ? `href="${pageUrl(lang, chapter.slug)}${hash}"` : `href="${href}.md${hash}"`;
    });
}

function wrapFigures(html) {
  return html.replace(/<p><img src="([^"]+)" alt="([^"]*)"><\/p>/g, (_, src, alt) => `
    <figure>
      <img src="${src}" alt="${alt}" />
      ${alt ? `<figcaption>${alt}</figcaption>` : ""}
    </figure>
  `);
}

function addHeadingAnchors(html, toc) {
  const used = new Map();
  return html.replace(/<h([2-3])>([\s\S]*?)<\/h\1>/g, (_, level, inner) => {
    const text = decodeHtml(stripTags(inner)).replace(/#/g, "").trim();
    const baseId = slugify(text);
    const count = used.get(baseId) ?? 0;
    used.set(baseId, count + 1);
    const id = count === 0 ? baseId : `${baseId}-${count + 1}`;
    toc.push({ id, text, depth: Number(level) });
    return `<h${level} id="${id}">${inner}<a class="heading-anchor" href="#${id}" aria-label="Link to this section">#</a></h${level}>`;
  });
}

function wrapCodeBlocks(html) {
  return html.replace(/<pre><code class="([^"]*)">([\s\S]*?)<\/code><\/pre>/g, (_, className, code) => {
    const language = className.match(/language-([a-z0-9_-]+)/i)?.[1] ?? "text";
    return `
      <div class="code-shell">
        <div class="code-toolbar"><span>${escapeHtml(language)}</span><button type="button" data-copy-code>Copy</button></div>
        <pre><code class="${className}">${code}</code></pre>
      </div>
    `;
  });
}

function htmlDocument({ lang, title, description, assetTags, body, redirect = null }) {
  return `<!doctype html>
<html lang="${lang}">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    ${redirect ? `<meta http-equiv="refresh" content="0; url=${redirect}" />` : ""}
    <meta name="description" content="${escapeHtml(description)}" />
    <meta property="og:title" content="${escapeHtml(title)}" />
    <meta property="og:description" content="${escapeHtml(description)}" />
    <meta property="og:type" content="website" />
    <link rel="icon" href="${base}assets/favicon.svg" type="image/svg+xml" />
    <title>${escapeHtml(title)}</title>
    ${assetTags}
  </head>
  <body>
    ${body}
  </body>
</html>`;
}

function pageUrl(lang, slug = "") {
  return `${base}${lang}/${slug ? `${slug}/` : ""}`;
}

function asset(name) {
  return `${base}assets/${name}`;
}

function chapterThumbnail(item) {
  return asset(chapterThumbnails[item.slug] ?? "screenshots/ch17-complete-rpg-gameplay.png");
}

function writeFile(relativePath, content) {
  const target = path.join(dist, relativePath);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, content);
}

function pad(value) {
  return String(value).padStart(2, "0");
}

function slugify(value) {
  const normalized = value
    .toLowerCase()
    .replace(/`/g, "")
    .replace(/&amp;/g, "and")
    .replace(/[^a-z0-9가-힣]+/g, "-")
    .replace(/^-+|-+$/g, "");
  return normalized || "section";
}

function stripTags(value) {
  return value.replace(/<[^>]+>/g, "");
}

function escapeHtml(value) {
  return String(value)
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

function decodeHtml(value) {
  return String(value)
    .replace(/&amp;/g, "&")
    .replace(/&lt;/g, "<")
    .replace(/&gt;/g, ">")
    .replace(/&quot;/g, "\"")
    .replace(/&#39;/g, "'");
}
