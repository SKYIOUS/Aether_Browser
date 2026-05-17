---
name: Aether
colors:
  surface: '#131313'
  surface-dim: '#131313'
  surface-bright: '#393939'
  surface-container-lowest: '#0e0e0e'
  surface-container-low: '#1c1b1b'
  surface-container: '#201f1f'
  surface-container-high: '#2a2a2a'
  surface-container-highest: '#353534'
  on-surface: '#e5e2e1'
  on-surface-variant: '#c3c6d0'
  inverse-surface: '#e5e2e1'
  inverse-on-surface: '#313030'
  outline: '#8d919a'
  outline-variant: '#43474f'
  surface-tint: '#a5c9fe'
  primary: '#d5e4ff'
  on-primary: '#00315d'
  primary-container: '#a5c9ff'
  on-primary-container: '#2e5483'
  inverse-primary: '#3b608f'
  secondary: '#d3bcfc'
  on-secondary: '#38265b'
  secondary-container: '#523f76'
  on-secondary-container: '#c4aeed'
  tertiary: '#e5e2e2'
  on-tertiary: '#303030'
  tertiary-container: '#c9c6c6'
  on-tertiary-container: '#535252'
  error: '#ffb4ab'
  on-error: '#690005'
  error-container: '#93000a'
  on-error-container: '#ffdad6'
  primary-fixed: '#d4e3ff'
  primary-fixed-dim: '#a5c9fe'
  on-primary-fixed: '#001c39'
  on-primary-fixed-variant: '#204876'
  secondary-fixed: '#ebdcff'
  secondary-fixed-dim: '#d3bcfc'
  on-secondary-fixed: '#230f45'
  on-secondary-fixed-variant: '#503d73'
  tertiary-fixed: '#e5e2e1'
  tertiary-fixed-dim: '#c8c6c5'
  on-tertiary-fixed: '#1b1b1c'
  on-tertiary-fixed-variant: '#474746'
  background: '#131313'
  on-background: '#e5e2e1'
  surface-variant: '#353534'
typography:
  headline-lg:
    fontFamily: Inter
    fontSize: 32px
    fontWeight: '500'
    lineHeight: '1.2'
    letterSpacing: -0.02em
  headline-lg-mobile:
    fontFamily: Inter
    fontSize: 24px
    fontWeight: '500'
    lineHeight: '1.2'
    letterSpacing: -0.01em
  headline-md:
    fontFamily: Inter
    fontSize: 20px
    fontWeight: '500'
    lineHeight: '1.4'
    letterSpacing: -0.01em
  body-lg:
    fontFamily: Inter
    fontSize: 16px
    fontWeight: '400'
    lineHeight: '1.6'
    letterSpacing: 0.01em
  body-md:
    fontFamily: Inter
    fontSize: 14px
    fontWeight: '400'
    lineHeight: '1.6'
    letterSpacing: 0.01em
  label-md:
    fontFamily: Inter
    fontSize: 12px
    fontWeight: '500'
    lineHeight: '1'
    letterSpacing: 0.05em
  label-sm:
    fontFamily: Inter
    fontSize: 11px
    fontWeight: '400'
    lineHeight: '1'
    letterSpacing: 0.03em
rounded:
  sm: 0.25rem
  DEFAULT: 0.5rem
  md: 0.75rem
  lg: 1rem
  xl: 1.5rem
  full: 9999px
spacing:
  unit: 8px
  gutter: 16px
  margin-mobile: 16px
  margin-desktop: 32px
  panel-padding: 24px
---

## Brand & Style

This design system is anchored in the concept of **Spatial Minimalism**. It is designed to provide a serene, distraction-free environment for deep work and intentional browsing. The aesthetic is "Dark-first Premium"—evoking the feeling of a quiet, high-end architectural studio at night.

The visual direction blends **Minimalism** with sophisticated **Glassmorphism**. By prioritizing negative space and reducing visual noise, the UI recedes into the background, allowing the user's content to take center stage. Every element is intentional, avoiding decorative clutter in favor of functional elegance and a calm, intellectual atmosphere.

## Colors

The palette is rooted in deep, matte neutrals to reduce eye strain and establish a grounded foundation. 

- **Base Surfaces:** Matte Charcoal (#121212) serves as the primary canvas.
- **Elevated Panels:** Graphite (#1E1E1E) is used for toolbars, sidebars, and floating panels to create subtle structural definition.
- **Accents:** Muted Icy Blue and Desaturated Violet are used sparingly. These are reserved for active states, focus indicators, or subtle highlights, ensuring they never overwhelm the dark environment.
- **Overlays:** Semi-transparent layers with background blurs (acrylic) are used to maintain context while navigating layers.

## Typography

The typography system utilizes **Inter** to achieve a clinical yet elegant appearance. The hierarchy is defined by varying weights (Light to Medium) and intentional tracking (letter spacing).

For headlines, a tighter letter spacing and medium weight create a modern, authoritative look. For body text and labels, the letter spacing is increased slightly to ensure legibility against dark backgrounds and to contribute to the "breathable" feel of the design system. Avoid using Bold weights; hierarchy should be achieved through scale and color contrast rather than heavy strokes.

## Layout & Spacing

This design system employs a **Fixed Grid** philosophy for the application "chrome" (toolbars, sidebars) and a fluid, centered model for web content. 

- **The 8px Rhythm:** All padding and margins scale in increments of 8px. 
- **Sidebars:** Use a fixed width (e.g., 280px) with high internal padding (24px) to maintain the minimalist aesthetic.
- **Viewports:** On desktop, large margins (32px) are used to box the content, reinforcing the architectural feel. On mobile, margins reduce to 16px to maximize screen real estate.
- **Negative Space:** Elements should never feel crowded. If a layout feels busy, increase the padding-to-content ratio rather than adding borders.

## Elevation & Depth

Depth is communicated through **Acrylic Surfaces** and light-based hierarchy rather than heavy shadows.

- **Surface Tiers:** Level 0 is the Matte Charcoal background. Level 1 (Graphite) is for persistent UI panels. Level 2 (Glass) is for floating menus or transient modals.
- **Glassmorphism:** Overlays use a backdrop blur (20px to 40px) with a 10% white tint. 
- **Dividers:** Use ultra-low contrast lines (1px, 10% opacity white) to separate sections without breaking the visual flow.
- **Shadows:** Use only one type of shadow—an ultra-soft, diffused ambient shadow (0px 12px 32px rgba(0,0,0,0.4))—reserved for the highest-level floating elements like modals or active browser tabs.

## Shapes

The shape language is "Smoothly Geometric." Standard UI elements (buttons, inputs) utilize a **0.5rem (8px)** radius to feel approachable. Larger containers like browser panels, cards, and modals use **1rem (16px)** or **1.5rem (24px)** to create a sophisticated, softened architectural frame. This progression of roundness ensures that smaller interactive elements feel precise, while the overall structure feels calm and organic.

## Components

- **Buttons:** Primary buttons use a subtle gradient of the primary color or a solid glass effect. Secondary buttons are "Ghost" style with a low-opacity border that only becomes prominent on hover.
- **Tabs:** Browser tabs should not have hard vertical dividers. Use a soft background fill for the active tab with 12px rounded top corners.
- **Input Fields:** Search and URL bars are Graphite (#1E1E1E) with a subtle 1px border. The text should be inset significantly (16px) to maintain the airy feel.
- **Cards:** Content cards use the Graphite surface with no border, relying on the background contrast and 16px corner radius for definition.
- **Chips/Status:** Small, pill-shaped indicators using the secondary Violet color at 15% opacity with 100% opacity text for soft, non-aggressive signaling.
- **Scrollbars:** Custom-styled to be ultra-thin and grey, appearing only on hover to prevent visual clutter.