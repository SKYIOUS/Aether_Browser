# Aether Browser UI Implementation Plan

## Objective
Replicate the Aether Browser UI designs from the screens/ HTML prototypes into a production-ready Iced interface.

## Modules
- src/ui/mod.rs: Main UI controller and routing.
- src/ui/screens/browser.rs: Main browser view (sidebar + dual-panel content).
- src/ui/screens/palette.rs: Command palette implementation.
- src/ui/screens/settings.rs: Settings interface.
- src/ui/style.rs: Global Aether color palette and glass-morphism style definitions.

## Steps
1. Create styling module with custom colors/themes based on oklch values from the HTML files.
2. Build Browser main screen with the specified sidebar and dual-panel layout.
3. Build Command Palette screen (modal/overlay).
4. Build Settings screen.
5. Integrate navigation/routing into src/main.rs.