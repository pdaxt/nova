# Nova Brand Guide

## Brand Essence

**Nova** — Latin for "new star." A stellar explosion that brings light where there was darkness.

Nova represents:
- **Clarity** — Code that proves itself correct
- **Precision** — Every byte matters, every decision documented
- **Trust** — Verified, not hoped
- **Innovation** — A new way to write software

---

## Logo

The Nova logo is a stylized star burst with the wordmark.

```
    ✦
   /|\
  / | \
 /  |  \
────┼────  NOVA
 \  |  /
  \ | /
   \|/
    ✦
```

The star represents:
- Explosion of light (nova event)
- Verification checkmark (correctness)
- Compass rose (direction, guidance)

---

## Color Palette

### Primary Colors

| Color | Hex | RGB | Usage |
|-------|-----|-----|-------|
| **Nova Blue** | `#0ea5e9` | 14, 165, 233 | Primary brand color, links, key elements |
| **Deep Space** | `#0c1222` | 12, 18, 34 | Backgrounds, dark mode |
| **Star White** | `#f8fafc` | 248, 250, 252 | Text on dark, highlights |

### Secondary Colors

| Color | Hex | RGB | Usage |
|-------|-----|-----|-------|
| **Verified Green** | `#10b981` | 16, 185, 129 | Success, correct, approved |
| **Alert Amber** | `#f59e0b` | 245, 158, 11 | Warnings, attention needed |
| **Error Red** | `#ef4444` | 239, 68, 68 | Errors, rejected, critical |

### Component Colors

Each foundation component has a signature color:

| Component | Color | Hex | Meaning |
|-----------|-------|-----|---------|
| **Span** | Cyan | `#06b6d4` | Location, position (like a cursor) |
| **Token** | Purple | `#8b5cf6` | Syntax, structure (lexical analysis) |
| **Source** | Blue | `#3b82f6` | Content, files (data) |
| **Error** | Amber | `#f59e0b` | Diagnostics (attention) |
| **Lexer** | Teal | `#14b8a6` | Processing (active transformation) |

### Tier Colors (for architecture diagrams)

| Tier | Color | Hex |
|------|-------|-----|
| Foundation | Nova Blue | `#0ea5e9` |
| Parsing | Purple | `#a855f7` |
| Types | Pink | `#ec4899` |
| IR | Orange | `#f97316` |
| Codegen | Green | `#22c55e` |

---

## Typography

### Primary Font
**Inter** — Clean, modern, highly legible at all sizes.
- Headings: Inter Bold (700)
- Body: Inter Regular (400)
- UI elements: Inter Medium (500)

### Code Font
**JetBrains Mono** — Designed for code, excellent symbol distinction.
- Code blocks: JetBrains Mono Regular
- Inline code: JetBrains Mono at 90% size

### Font Scale

| Element | Size | Weight | Line Height |
|---------|------|--------|-------------|
| H1 | 36px | 700 | 1.2 |
| H2 | 28px | 700 | 1.25 |
| H3 | 20px | 600 | 1.3 |
| Body | 16px | 400 | 1.5 |
| Small | 14px | 400 | 1.4 |
| Code | 14px | 400 | 1.6 |

---

## Visual Language

### Shapes

- **Rounded rectangles** (radius 12px) — Components, containers
- **Circles** — Status indicators, badges
- **Lines with arrows** — Data flow, dependencies
- **Dashed lines** — Optional or indirect relationships

### Shadows

Consistent drop shadow for depth:
```css
box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1),
            0 2px 4px -1px rgba(0, 0, 0, 0.06);
```

For dark backgrounds:
```css
filter: drop-shadow(0 4px 12px rgba(0, 0, 0, 0.4));
```

### Gradients

Use subtle gradients for component cards:
- Direction: Top-left to bottom-right (135°)
- Opacity difference: 10-15%

Example (Nova Blue):
```css
background: linear-gradient(135deg, #0ea5e9 0%, #0284c7 100%);
```

---

## Iconography

### Status Icons

| Status | Icon | Color |
|--------|------|-------|
| Correct/Pass | ✓ (checkmark) | Verified Green |
| Error/Fail | ✕ (cross) | Error Red |
| Warning | ⚠ (triangle) | Alert Amber |
| Info | ℹ (info) | Nova Blue |

### Flow Icons

| Concept | Symbol |
|---------|--------|
| Data flow | → (arrow) |
| Dependency | ─── (solid line) |
| Optional | ┈┈┈ (dashed line) |
| Bidirectional | ↔ (double arrow) |

---

## Diagram Standards

### Background
- Always use **Deep Space** (`#0c1222`) for diagram backgrounds
- Add subtle grid pattern at 5% opacity for technical feel

### Components
- Use component's signature color
- White text on colored backgrounds
- 12px border radius
- Subtle gradient (10% lighter to base)
- Drop shadow for depth

### Text Hierarchy
1. **Component Name** — 16px, Bold, White
2. **Description** — 12px, Regular, White at 80% opacity
3. **Technical Details** — 11px, Mono, White at 60% opacity

### Arrows/Lines
- Color: `#64748b` (Slate 500)
- Width: 2px for main flows, 1px for secondary
- Arrow heads: Filled triangles, 8×6 pixels

### Annotations
- Background: `#1e293b` (Slate 800)
- Border: 1px `#334155` (Slate 700)
- Text: `#94a3b8` (Slate 400)

---

## Usage Examples

### Good Example
```
┌────────────────────────────────────┐
│ Deep Space background (#0c1222)    │
│                                    │
│   ┌─────────────┐   Component with │
│   │   SPAN      │   signature color│
│   │   8 bytes   │   and gradient   │
│   └─────────────┘                  │
│         │                          │
│         ▼ Slate arrow              │
│   ┌─────────────┐                  │
│   │   TOKEN     │   Clear visual   │
│   │   12 bytes  │   hierarchy      │
│   └─────────────┘                  │
│                                    │
└────────────────────────────────────┘
```

### Bad Example (What NOT to do)
- ❌ Random gradient colors
- ❌ Inconsistent border radius
- ❌ Multiple font sizes without hierarchy
- ❌ Arrows in different colors
- ❌ Missing shadows (flat appearance)
- ❌ Text too small to read
- ❌ No clear visual flow

---

## File Naming

Diagram files should follow:
```
nova-[topic]-[variant].svg
nova-[topic]-[variant].png
```

Examples:
- `nova-foundation-overview.svg`
- `nova-pipeline-full.png`
- `nova-span-comparison.svg`

---

## Brand Voice

### Tone
- **Precise** — Say exactly what we mean
- **Confident** — We know this is right
- **Inclusive** — Anyone can contribute
- **Technical** — We respect our audience's intelligence

### Writing Style
- Use active voice
- Be concise
- Explain the "why" not just the "what"
- Include code examples
- Acknowledge trade-offs

### Example

**Good:**
> Span is 8 bytes because it's copied everywhere. At 10,000 spans, that's 80KB vs 160KB—a 50% saving that matters.

**Bad:**
> We decided to make Span 8 bytes for efficiency reasons.

---

*This brand guide ensures Nova looks professional, consistent, and trustworthy across all documentation and materials.*
