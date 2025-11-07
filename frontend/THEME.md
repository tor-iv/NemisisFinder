# Nemesis Theme Guide

## Overview

The **Nemesis Theme** is a dark, edgy design system that reflects the concept of finding your opposite. It features dramatic contrasts, bold accent colors, and atmospheric effects.

## Color Palette

### Background Colors
```css
--nemesis-bg-primary: #0a0a0f     /* Deep dark blue-black */
--nemesis-bg-secondary: #1a1a2e   /* Dark blue-gray */
--nemesis-bg-tertiary: #16213e    /* Medium dark blue */
```

### Accent Colors
```css
--nemesis-accent-primary: #e94560      /* Vibrant pink-red */
--nemesis-accent-secondary: #a855f7    /* Purple */
--nemesis-accent-tertiary: #3b82f6     /* Blue */
```

### Text Colors
```css
--nemesis-text-primary: #e5e7eb        /* Light gray (main text) */
--nemesis-text-secondary: #9ca3af      /* Medium gray (secondary text) */
--nemesis-text-muted: #6b7280          /* Dark gray (muted text) */
```

### Gradients
```css
--nemesis-gradient: linear-gradient(135deg, #667eea 0%, #764ba2 50%, #e94560 100%)
--nemesis-gradient-dark: linear-gradient(135deg, #1a1a2e 0%, #16213e 50%, #0f3460 100%)
```

## Typography

### Logo/Branding
- **NEMESIS**: Bold, uppercase, accent-primary color, 2px letter-spacing
- **FINDER**: Light weight, uppercase, text-primary color, 1px letter-spacing

### Headings
- Font: Roboto
- Weights: 300 (light), 500 (medium), 700 (bold)
- Colors: Primary text with accent highlights

### Body Text
- Font: Roboto, 'Helvetica Neue', sans-serif
- Size: 14px base
- Color: --nemesis-text-primary

## Components

### Cards
```scss
background: var(--nemesis-bg-secondary)
border: 1px solid rgba(233, 69, 96, 0.2)
box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5)
backdrop-filter: blur(10px)
```

### Buttons

#### Primary Button
```scss
background: linear-gradient(135deg, #e94560, #a855f7)
color: white
font-weight: 600
letter-spacing: 1px
box-shadow: 0 4px 15px rgba(233, 69, 96, 0.3)

&:hover {
  transform: translateY(-2px)
  box-shadow: 0 6px 20px rgba(233, 69, 96, 0.5)
}
```

#### Secondary Button
```scss
background: transparent
border: 1px solid var(--nemesis-accent-primary)
color: var(--nemesis-accent-primary)

&:hover {
  background: rgba(233, 69, 96, 0.1)
}
```

### Form Fields
```scss
background: var(--nemesis-bg-tertiary)
border: 1px solid var(--nemesis-border)
color: var(--nemesis-text-primary)

&:focus {
  border-color: var(--nemesis-accent-primary)
  box-shadow: 0 0 0 3px rgba(233, 69, 96, 0.1)
}
```

## Effects

### Glowing Background
```scss
.background-glow {
  position: absolute
  background: radial-gradient(circle, rgba(233, 69, 96, 0.15), transparent 70%)
  animation: pulse 8s ease-in-out infinite
}

@keyframes pulse {
  0%, 100% { opacity: 0.3; transform: scale(1); }
  50% { opacity: 0.6; transform: scale(1.1); }
}
```

### Hover Effects
- **Scale**: `transform: scale(1.05)`
- **Lift**: `transform: translateY(-2px)`
- **Glow**: Increased box-shadow with accent color

### Animations
```scss
@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
```

## Scrollbar

```scss
::-webkit-scrollbar {
  width: 10px;
}

::-webkit-scrollbar-track {
  background: var(--nemesis-bg-secondary);
}

::-webkit-scrollbar-thumb {
  background: var(--nemesis-accent-primary);
  border-radius: 5px;
}

::-webkit-scrollbar-thumb:hover {
  background: var(--nemesis-accent-secondary);
}
```

## Usage Examples

### Page Container
```html
<div class="page-container">
  <div class="background-glow"></div>
  <mat-card class="content-card fade-in">
    <!-- Content here -->
  </mat-card>
</div>
```

### Buttons
```html
<!-- Primary action -->
<button mat-raised-button color="accent" class="submit-button">
  Submit
</button>

<!-- Secondary action -->
<button mat-button class="nav-btn">
  Cancel
</button>
```

### Typography
```html
<h1>
  <span class="title-nemesis">NEMESIS</span>
  <span class="title-finder">FINDER</span>
</h1>
```

## Responsive Breakpoints

```scss
// Mobile
@media (max-width: 576px) {
  .tagline { display: none; }
  .logo { font-size: 18px; }
}

// Tablet
@media (max-width: 768px) {
  .welcome { display: none; }
}

// Desktop
@media (min-width: 1200px) {
  .container { max-width: 1140px; }
}
```

## Accessibility

### Contrast Ratios
- Primary text on dark bg: 12.63:1 (AAA)
- Secondary text on dark bg: 7.5:1 (AA)
- Accent on dark bg: 4.5:1 (AA)

### Focus States
All interactive elements have visible focus states:
```scss
&:focus-visible {
  outline: 2px solid var(--nemesis-accent-primary);
  outline-offset: 2px;
}
```

## Design Philosophy

The Nemesis theme embodies the concept of **opposites**:

1. **Light vs Dark**: Bright accent colors against deep dark backgrounds
2. **Bold vs Subtle**: Strong typography with delicate glows
3. **Sharp vs Soft**: Clean edges with blur effects
4. **Static vs Dynamic**: Solid UI with pulsing animations

This creates a visual metaphor for the app's purpose: bringing together opposing viewpoints in a harmonious, engaging experience.

## File Structure

```
src/
├── styles.scss           # Global styles + theme import
├── theme.scss            # Angular Material custom theme
└── app/
    ├── features/
    │   └── auth/
    │       └── register.component.scss  # Reusable auth styles
    └── shared/
        └── components/
            └── *.component.ts  # Component-specific styles
```

## Customization

To modify the theme, edit `src/theme.scss`:

```scss
// Change primary accent
$nemesis-accent: mat.define-palette(
  $your-custom-palette,
  500  // main shade
);

// Update CSS variables in :root
:root {
  --nemesis-accent-primary: #your-color;
}
```

## Browser Support

- Chrome 90+
- Firefox 88+
- Safari 14+
- Edge 90+

Features like `backdrop-filter` may not work in older browsers and will gracefully degrade.
