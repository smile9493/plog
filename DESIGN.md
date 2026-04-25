# Plog Design System

## Colors

Strategy: Restrained. Tinted neutrals + one warm accent. The accent carries the brand, neutrals carry the content.

### Light theme
- Background: oklch(0.97 0.005 75) — warm stone
- Surface: oklch(0.995 0.003 75) — near-white with warmth
- Text: oklch(0.15 0.01 60) — warm black
- Text muted: oklch(0.50 0.01 60) — warm gray
- Text light: oklch(0.65 0.008 60) — lighter gray
- Border: oklch(0.88 0.008 75) — warm border
- Accent: oklch(0.55 0.15 55) — warm amber/brown

### Dark theme
- Background: oklch(0.17 0.01 60) — warm dark
- Surface: oklch(0.22 0.01 60) — slightly lighter dark
- Text: oklch(0.95 0.005 75) — warm white
- Text muted: oklch(0.65 0.01 60)
- Text light: oklch(0.50 0.008 60)
- Border: oklch(0.32 0.01 60)
- Accent: oklch(0.78 0.14 85) — warm gold

### Sepia theme
- Background: oklch(0.93 0.02 80) — parchment
- Surface: oklch(0.96 0.015 80) — lighter parchment
- Text: oklch(0.30 0.03 55) — dark brown
- Text muted: oklch(0.50 0.02 55)
- Text light: oklch(0.62 0.015 55)
- Border: oklch(0.80 0.02 75)
- Accent: oklch(0.45 0.12 50) — deep amber

## Typography

- Body: 'Noto Serif SC', serif — for Chinese content readability
- UI: 'Inter', system-ui, sans-serif — for navigation, labels, metadata
- Mono: 'JetBrains Mono', monospace — for code blocks

### Scale
- Page title: clamp(1.75rem, 5vw, 2.25rem), weight 700
- Section heading (h2): 1.25rem, weight 600
- Card title: 1rem, weight 600
- Body: 1rem (16px), line-height 1.7
- Small/meta: 0.8125rem
- Tag: 0.6875rem

### Line length
- Content: 680px max (var(--container-max))
- Wide layouts: 1200px max (var(--container-wide))

## Spacing

Base unit: 0.25rem. Scale: 0.25, 0.5, 1, 1.5, 2, 3, 4rem.
Vary spacing for rhythm. Section gaps at 3rem. Card padding at 1.5rem.

## Elevation

Minimal. 1px borders only. Subtle shadows on hover only (0 2px 8px rgba). No floating cards at rest.

## Components

### Feature card
Icon (40px) + title + description + meta/tags. Border only, no shadow at rest. Hover: border accent + slight lift.

### Book item
Horizontal layout: cover (80x110) + info. Border only.

### Category item
Full-width card with name, description, count. Border only.

### Status badge
Small pill with background tint. Success/warn/info variants.

## Motion

- Transitions: 0.2s ease
- Hover lift: translateY(-2px) on cards
- No bounce, no elastic, no decorative animation

## Borders

- Radius: 4px (small), 6px (medium), 8px (large)
- Width: 1px solid
- Color: var(--color-border)
