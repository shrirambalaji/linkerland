<div align="center">
<picture align="center" width="400px;">
  <source media="(prefers-color-scheme: dark)" srcset="./.github/images/logo-dark.png" width="400px;">
  <img alt="linkerland logo in light" src="./.github/images/logo-light.png" width="400px;">
</picture>
</div>

<strong>linkerland</strong> is a tiny workbenchfor linker outputs with a collection of tools for inspecting linker artifacts.

- `linkerland viz` — interactive **map** viewer (TUI) to find hot sections/symbols fast
- `linkerland export` — non-interactive export (JSON/CSV) for scripting and CI

## Getting Started

```bash
# Build
cargo install --path .

# Open a .map (TUI). Shorthand: passing a .map defaults to `viz`.
linkerland path/to/app.map
# or
linkerland viz path/to/app.map

# Export current view as JSON/CSV (good for CI)
linkerland export path/to/app.map --format json --out app.text.json
```
