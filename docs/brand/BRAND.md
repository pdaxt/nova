# Nova Brand Guidelines

> Open source brand assets and design rationale for the Nova programming language.

## Brand Philosophy

Nova is a programming language where **code proves itself**. Our brand reflects:

1. **Clarity** — No ambiguity in code, no ambiguity in design
2. **Verification** — Every program is provably correct
3. **Innovation** — Built for the AI age
4. **Trust** — Reliability you can see

## Logo Concept

### The Nova Mark

The Nova mark combines two core concepts:

```
Nova (astronomy) = A star that suddenly increases in brightness
Nova (language) = Code that proves itself correct
```

**Design elements:**

| Element | Meaning |
|---------|---------|
| **8-point burst** | A nova explosion — sudden brilliance, new star |
| **Checkmark center** | Verification — code that proves itself |
| **Gradient purple** | Innovation, creativity, future-forward |
| **Radiating rays** | Clarity emanating outward |

The checkmark inside the nova burst represents **verified correctness at the core** — the fundamental promise of the language.

### Why This Design Works

1. **Memorable** — Distinct shape, not confused with other lang logos
2. **Scalable** — Works at 16px favicon to 1000px hero
3. **Meaningful** — Every element has purpose
4. **Versatile** — Works on light and dark backgrounds

## Logo Versions

### Primary Mark
`nova-logo.svg` — Full logo with rays and verification checkmark

### Simplified Mark
`nova-mark.svg` — Compact version for small sizes (128px and below)

### Wordmark
`nova-wordmark.svg` — Logo + "NOVA" text (white text for dark backgrounds)

### Wordmark Dark
`nova-wordmark-dark.svg` — Logo + "NOVA" text (dark text for light backgrounds)

## Color Palette

### Primary Colors

| Color | Hex | Usage |
|-------|-----|-------|
| **Nova Purple** | `#6366f1` | Primary brand, logo, accents |
| **Nova Purple Light** | `#818cf8` | Gradients, highlights |
| **Nova Purple Dark** | `#4f46e5` | Shadows, depth |

### Semantic Colors

| Color | Hex | Usage |
|-------|-----|-------|
| **Verified Green** | `#22c55e` | Success, verification passed |
| **Warning Amber** | `#f59e0b` | Warnings, caution |
| **Error Red** | `#ef4444` | Errors, verification failed |
| **Info Cyan** | `#06b6d4` | Information, tips |

### Background Colors

| Color | Hex | Usage |
|-------|-----|-------|
| **Background** | `#0a0a0f` | Primary dark background |
| **Surface** | `#12121a` | Cards, elevated surfaces |
| **Surface Alt** | `#1a1a24` | Secondary surfaces |
| **Border** | `rgba(255,255,255,0.1)` | Subtle borders |

### Text Colors

| Color | Hex | Usage |
|-------|-----|-------|
| **Text Primary** | `#ffffff` | Headings, emphasis |
| **Text Secondary** | `#a0a0b0` | Body text, descriptions |
| **Text Muted** | `#64748b` | Captions, metadata |

## Typography

### Font Stack

```css
/* Primary (headings, UI) */
font-family: 'Inter', system-ui, -apple-system, sans-serif;

/* Code */
font-family: 'JetBrains Mono', 'Fira Code', monospace;
```

### Font Weights

| Weight | Usage |
|--------|-------|
| 400 (Regular) | Body text |
| 500 (Medium) | Subheadings, emphasis |
| 600 (Semibold) | Section headings |
| 700 (Bold) | Page titles, hero text |

### Type Scale

| Size | Usage |
|------|-------|
| 2.5rem (40px) | Page titles |
| 1.75rem (28px) | Section headings |
| 1.25rem (20px) | Subheadings |
| 1rem (16px) | Body text |
| 0.875rem (14px) | Small text, captions |

## Usage Guidelines

### Do

- Use the full-color logo on dark backgrounds
- Maintain clear space around the logo (minimum: logo height / 4)
- Use the wordmark for contexts where "Nova" isn't clear
- Use the mark alone when space is limited

### Don't

- Stretch or distort the logo
- Change the logo colors outside brand palette
- Add effects (shadows, glows, outlines)
- Place on busy backgrounds without contrast
- Rotate the logo

## Logo Clear Space

```
     ┌─────────────────────────┐
     │                         │
     │    ┌───────────┐        │
     │    │           │        │
     │    │   LOGO    │  ← Minimum 1/4 height
     │    │           │        │  around all sides
     │    └───────────┘        │
     │                         │
     └─────────────────────────┘
```

## Minimum Sizes

| Version | Minimum Size |
|---------|--------------|
| Full logo | 64px |
| Mark only | 24px |
| Wordmark | 120px width |
| Favicon | 16px (use simplified mark) |

## File Formats

| Format | Usage |
|--------|-------|
| `.svg` | Web, documentation, scalable contexts |
| `.png` | Social media, presentations |
| `.ico` | Browser favicon |

## Co-branding

When Nova appears alongside other logos:
1. Maintain equal visual weight
2. Use consistent spacing
3. Don't modify either logo
4. Prefer horizontal arrangements

## License

All Nova brand assets are licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/).

You may:
- Use the logo to reference Nova
- Include in articles, tutorials, talks about Nova
- Create Nova-related projects and tools

Attribution: "Nova logo by the Nova project"

---

## Downloads

All brand assets are available in the [`/docs/brand/`](./brand/) directory:

- `nova-logo.svg` — Primary logo
- `nova-mark.svg` — Simplified mark
- `nova-wordmark.svg` — Logo with text (light text)
- `nova-wordmark-dark.svg` — Logo with text (dark text)

For questions about brand usage, open a [discussion](https://github.com/pdaxt/nova/discussions).
