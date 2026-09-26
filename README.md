<strong>WORK IN PROGRESS.</strong> Pretty barebones. Selecting "Timeline" and "Artifacts" in the area menu will result in a "not yet implemented" panic.

# narraverse

narraverse is an experimental, plain text no-BS worldbuilding TUI featuring narrative and entity creation/modification with more features to come.

## Installation.
Prerequisites:
- Rust/Cargo installed on your system (preferrably latest possible version)

### From source
```bash
git clone https://codeberg.org/enmkdev/narraverse && cd narraverse && cargo install --path .
```

## Configuration.
The config file is located at "~/.config/narraverse/config.toml" (idk for Windows).

### Config options:
- default_written_by (string): the default name to use for prefilling the "Written by" field on narrative creation.

### Default configuration:
```toml
default_written_by = "Me"
```

## Roadmap.
- [X] Stabilize the thing a little bit
- [X] Entity creation & modification
- [ ] Timeline creation & modification
- [ ] Artifacts creation & modification

## License.
GPL-3.0 only. Read more in [the license file](https://codeberg.org/enmkdev/narraverse/src/branch/main/LICENSE.md).
