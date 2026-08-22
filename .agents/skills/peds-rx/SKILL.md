---
name: peds-rx
description: >-
  Development, drug data curation, calculation verification, and deployment guide for the PedsRx
  Pediatric Drug Calculator (兒科常用藥快速查詢與精算系統) built with Rust, Yew, WebAssembly, and Trunk.
---

# PedsRx Pediatric Drug Calculator Skill

This skill provides comprehensive instructions for maintaining, extending, calculating, and deploying the **PedsRx (兒科常用藥快速查詢與精算系統)** codebase.

## Project Overview

- **Stack**: Rust 2024, Yew 0.21 (Client-Side Rendering), WebAssembly (`wasm32-unknown-unknown`), Trunk bundler.
- **Data Engine**: `data.js` containing `window.PEDS_DATA` (drug database with `calc(BW)` dose calculation formulas) and `window.PEDS_TOC` (category grouping index).
- **Deployment**: Automatic GitHub Actions CI/CD to GitHub Pages (`/Peds/`) with Subresource Integrity (SRI sha384).
- **Features**:
  - Real-time pediatric dose recalculation upon body weight (BW) input.
  - Route color coding (oral, rectal suppository, injection, topical).
  - Safety caps and warning callouts (dosage upper limits, age restrictions).
  - Categorized drawer navigation with drug counters.
  - Full-text fuzzy search (brand name, generic name, indications, symptoms).
  - 3 eye-care color palettes (🌿 柔和護眼灰, 📖 暖米色調, 🌙 深色夜間模式).
  - Fully responsive mobile & tablet design (touch-friendly controls, drawer navigation).

---

## Key Workflows

### 1. Adding or Modifying Drug Data (`data.js`)

All drug definitions are stored in `data.js` under `const DATA = [...]`.

#### Drug Object Structure:
```javascript
{
  n: '藥品商品名 (學名) 規格 劑型', // e.g. 'Voren (diclofenac) 12.5 mg 栓劑' (Prefix ★ for starred drugs)
  rt: '栓劑',                      // Route badge: '針劑' | '栓劑' | '外用' | '' (口服)
  s: '成分劑量｜給藥途徑｜適應症條件', // Subtitle
  w: '嚴重安全性警語 (紅框 ⚠)',       // Optional warning
  wm: '一般注意事項 (橘框 ※)',        // Optional mild warning
  f: '常用簡化公式字串',             // e.g. '(BW/12.5) 顆'
  m: [                            // Array of clinical explanation bullet points
    'Max 1 顆/dose；一日最多 3 次',
    '一般開 3–5 顆回家'
  ],
  r: [                            // Optional array of [Reference Name, URL]
    ['非炎栓劑仿單', 'https://...']
  ],
  calc: BW => [                   // Calculation function taking body weight in kg, or null if fixed
    {
      lbl: '每次劑量',             // Row label
      big: r1(BW/12.5) + ' #',    // Calculated prominent number/text
      freq: 'Q8H PRN',            // Frequency badge
      flag: BW/12.5 > 1 ? '已達單次上限 1 顆' : null // Optional safety flag
    },
    {
      lbl: '每日上限',
      big: '37.5 mg',
      freq: '≤3 mg/kg/day',
      sub: true                   // Set true for secondary/sub-dose annotation rows
    }
  ]
}
```

#### Math Helper Functions in `data.js`:
- `r1(n)`: Round to 1 decimal place (`Math.round(n * 10) / 10`)
- `r2(n)`: Round to 2 decimal places (`Math.round(n * 100) / 100`)

---

### 2. Rust Application Architecture (`src/main.rs`)

- **DOM Mounting**: Mounted via `yew::Renderer::<App>::new().render()`.
- **State Management**:
  - `weight`: `Option<f64>` - Body weight in kg.
  - `search_query`: `String` - Search term.
  - `selected_category`: `i32` - Currently selected category index (`-1` for ALL).
  - `drawer_open`: `bool` - Mobile drawer toggle state.
  - `theme_idx`: `usize` - Active eye-care theme (0: Soft Slate, 1: Warm Paper, 2: Dark Night).
- **Safe HTML Rendering**:
  - Uses `safe_html(&str) -> Html` (`Html::from_html_unchecked`) to safely render `<b>`, `<i>`, and formatting tags without React-specific attributes.

---

### 3. Local Development & Testing

#### Prerequisites:
- Rust with `wasm32-unknown-unknown` target:
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
- Trunk bundler:
  ```bash
  cargo install trunk
  ```

#### Commands:
- **Check Rust build**:
  ```bash
  cargo build --target wasm32-unknown-unknown
  ```
- **Run local development server with hot-reload**:
  ```bash
  trunk serve
  ```
- **Build production release**:
  ```bash
  trunk build --release --public-url /Peds/
  ```

---

### 4. CI/CD & Deployment

- Automated via `.github/workflows/deploy.yml` on push to `master` branch.
- Generates static bundle into `dist/` with **Subresource Integrity (SRI sha384)**.
- Deploys directly to GitHub Pages: `https://b8401121.github.io/Peds/`.
