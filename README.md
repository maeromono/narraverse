# narraverse

narraverse is an experimental, plain text no-BS worldbuilding TUI.

<strong>Another save file breaking change possible in the next update. Make regular backups!</strong>

## Features.
- Single binary, no extra runtime dependencies
- Create, edit and delete unlimited narratives, entities and artifacts
- "Convenient" shortcuts and vim-like bindings
- Emacs-like shortcuts in the editor, which definitely won't clash with your vim muscle memory

## Installation.
Prerequisites:
- Rust/Cargo installed on your system (preferrably latest possible version)

### Through Cargo (recommended)
```bash
cargo install --locked narraverse
```

### From source
```bash
git clone https://codeberg.org/enmkdev/narraverse && cd narraverse && cargo install --path .
```

## Configuration.
Press 'c' at the verse pick screen to enter config edit modal, or edit the config file located at "~/.config/narraverse/config.toml" (idk for Windows).

### Config options:
- 'Written by' name / default_written_by (string): the default name to use for prefilling the "Written by" field on narrative creation.

### Default configuration:
```toml
default_written_by = "Me"
```

## Roadmap.
- [X] Stabilize the thing a little bit
- [X] Entity creation & modification
- [X] Artifacts creation & modification
- [ ] Extra fields for entities
- [ ] Verse export
- [ ] Editor improvements (vim binds for editor, search, etc)

## License.
GPL-3.0 only. Read more in [the license file](https://codeberg.org/enmkdev/narraverse/src/branch/main/LICENSE.md).
