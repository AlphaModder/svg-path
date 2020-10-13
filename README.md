# `svg-path`
`svg-path` is a simple, dependencyless library for composing [SVG path data strings](https://www.w3.org/TR/SVG/paths.html), intended for use in Rust-based frontend frameworks and tools that output SVG. It consists of a single type, `Path`, which supports a simple and type-safe builder-like syntax for appending path commands, and implements `Display` for conversion into the SVG syntax.

`Path` also includes helper methods for producing certain common path shapes that can be nontrivial to express directly (currently just `Path::partial_circle`).

## Future plans
Considerations for the future of this library, depending on demand, include:
- Support for concatenation of `Path` objects
- Support for passing data types other than `f32` for command values
- First-class treatment of commands with a `PathCommand` enum of some sort.
- Support for parsing `Path` objects from strings (likely behind a feature, since this would probably mean introducing dependencies.)

## Alternatives and related projects
- [`svgtypes`](https://github.com/bodoni/svg), a library with parsers and composers for many SVG data types (but not the XML syntax itself).
- [`svg_minimal`](https://github.com/36den/svg_minimal-rs), a similarly minimal path-focused SVG library 
- [`svg`](https://github.com/bodoni/svg), a full SVG composer and parser
- [`nsvg`](https://github.com/nickbrowne/nsvg), a Rust wrapper for the NanoSVG C library.
- [`rsvg`](https://crates.io/crates/rsvg), Rust bindings to the `rsvg` library
- [`resvg`](https://github.com/RazrFalcon/resvg), a Rust SVG renderer
- [`lyon_path`](https://github.com/nical/lyon/tree/master/path), a library with an SVG-inspired path data structure