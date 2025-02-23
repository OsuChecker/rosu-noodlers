# rosu-noodlers

A Rust library for manipulating long notes (LN) in osu!mania beatmaps.

## Features

- Convert all long notes to normal notes (`noln`)
- Convert all notes to long notes with custom gap and duration (`full_ln`)
- Convert notes to long notes based on beat divisions (`full_ln_beat_divisor`)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rosu-noodlers = "0.1.0"
rosu-map = "0.2.0"
```

## Usage

```rust
use rosu_noodlers::{noln, all_ln, full_ln_beat_divisor};
use rosu_map::Beatmap;

// Load a beatmap
let mut map = Beatmap::from_path("path/to/map.osu").unwrap();

// Remove all long notes
noln(&mut map);

// Convert all notes to LNs with 40ms gap and 30ms minimum duration
full_ln(&mut map, 40.0, 30.0);

// Convert notes to LNs based on 1/8 beat division
full_ln_beat_divisor(&mut map, 8.0, None);

// Save the modified beatmap
map.encode_to_path("path/to/output.osu").unwrap();
```


## Requirements

- Rust 1.84.1 or higher
- rosu-map 0.2.0

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

- [rosu-map](https://github.com/MaxOhn/rosu-map) for the osu! beatmap parsing functionality