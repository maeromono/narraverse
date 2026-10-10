<strong>Another save file breaking change possible in the next update. Make regular backups!</strong>
# narraverse

narraverse is an experimental, plain text no-BS worldbuilding TUI.

## Features.
- Single binary, no extra runtime dependencies
- Create, edit, reorder* and delete unlimited narratives, entities and artifacts
- Edit multiple narratives/entities/artifacts at once with multibuffers
- "Convenient" shortcuts and vim-like bindings (there's a bind help (F1) in case you feel stuck)
- Emacs-like shortcuts in the editor, which definitely won't clash with your vim muscle memory

\* Only applies to entities & artifacts.

## Warning.
Typing the uppercase A (or maybe some other characters that I didn't test) in editor search box results in an "attempt to subtract with overflow" panic due to the bug in the library used by narraverse (ratatui-textarea). [I already contributed the fix upstream which got merged](https://github.com/ratatui/ratatui-textarea/pull/33) and it should be available in ratatui-textarea v0.9.4.

## Installation.
Prerequisites:
- Rust/Cargo installed on your system (preferrably latest possible version)

### Latest release (recommended)
```bash
cargo install --locked narraverse
```

### From source
```bash
git clone https://codeberg.org/enmkdev/narraverse && cd narraverse && cargo install --path .
or
cargo install --git https://codeberg.org/enmkdev/narraverse --locked
```

## Configuration.
Press 'c' at the verse pick screen to enter config edit modal, or edit the config file located at "~/.config/narraverse/config.toml" (Linux), "%LOCALAPPDATA%\\enmkdev\\narraverse\\config\\config.toml" (Windows).

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
- [ ] Referencing entities & artifacts from narrative
- [ ] Entity relationships
- [ ] Extra fields for entities
- [ ] Verse export (MD or EPUB)

## License.
GPL-3.0 only. Read more in [the license file](https://codeberg.org/enmkdev/narraverse/src/branch/main/LICENSE.md).
