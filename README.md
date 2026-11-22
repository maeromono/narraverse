<strong>Another save file breaking change possible in v0.5.0. A migration guide will be written when it's released but it's still better to make regular backups!</strong>
# narraverse

narraverse is a worldbuilding TUI, premise of which is no-bullshit, plain text, "convenient" world creation.

## Features.
- Single binary, no extra runtime dependencies
- Create, edit, reorder* and delete unlimited narratives, entities and artifacts
- Edit multiple narratives/entities/artifacts at once with multibuffers
- "Convenient" shortcuts and vim-like bindings (there's a bind help (F1) in case you feel stuck)
- Emacs-like shortcuts in the editor, which definitely won't clash with your vim muscle memory

\* Only applies to entities & artifacts.

## Installation.
Prerequisites:
- Rust/Cargo installed on your system (preferrably latest possible version)

### Latest release (recommended)
```bash
cargo install --locked narraverse
```

### From source
```bash
cargo install --git https://codeberg.org/maeromono/narraverse --locked
```

## Configuration.
Press 'c' at the verse pick screen to enter config edit modal, or edit the config file located at:
- "~/.config/narraverse/config.toml" on Linux,
- "%LOCALAPPDATA%\\maeromono\\narraverse\\config\\config.toml" on Windows.

### Config options:
- 'Written by' name / default_written_by (string): the default name to use for prefilling the "Written by" field on narrative creation.

### Default configuration:
```toml
default_written_by = "Me"
```

## Roadmap (in no particular order).
- [X] Stabilize the thing a little bit
- [X] Entity creation & modification
- [X] Artifacts creation & modification
- [X] Editor improvements (multibuffers, search, etc)
- [ ] Chapters & acts for narratives
- [ ] Referencing entities & artifacts from narrative
- [ ] Entity relationships
- [ ] Extra fields for entities
- [ ] Verse export (MD or EPUB)

## License.
GPL-3.0 only. Read more in [the license file](https://codeberg.org/maeromono/narraverse/src/branch/main/LICENSE.md).
