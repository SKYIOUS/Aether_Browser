# UI Documentation

The Aether Browser UI is built using the Iced library, following a clean, minimalist, and space-conscious design.

## Design Palette (OKLCH)
The interface utilizes specific colors from the design system to ensure coherence:
- **Background:** `oklch(12% 0.01 250)`
- **Surface:** `oklch(18% 0.01 250)`
- **Accent:** `oklch(75% 0.12 255)`

## Structure
- `src/ui/style.rs`: Centralized color definitions.
- `src/ui/screens/`: Individual screen implementations.
- `src/ui/mod.rs`: Main routing and UI state management.

## Components
The interface employs a side-navigation layout (sidebar + dual-panel content area) for productivity, mirroring the design language of Aether's web-based prototypes.
