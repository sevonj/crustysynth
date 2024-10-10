# CrustySynth
![ci badge](https://github.com/sevonj/crustysynth/actions/workflows/rust.yml/badge.svg)

CrustySynth is a Rust crate for midi files. 

The project is being developed alongside and for [SfontPlayer.](https://github.com/sevonj/sfontplayer) The purpose of CrustySynth is to provide replacements to some [RustySynth](https://github.com/sinshu/rustysynth/) components, as it has some limitations, such as no support for seeking.

This may be useful to you, if you intend to parse or play midi files.

CrustySynth aims to cover MIDI file format as described here:  
https://www.cs.cmu.edu/~music/cmsip/readings/Standard-MIDI-file-format-updated.pdf

## Roadmap:

- [ ] Parse midi files
  - Format:
    - [x] Midi keys
    - [x] Midi channels
    - [ ] CC message types
    - [ ] GM instruments
  - [x] Tracks
  - [x] Midi events
  - [x] System events
  - [ ] Meta-events (raw bytes available)
- [ ] Play midi files (using RustySynth)

## Crates

- **crustysynth**  
  The main event.
- **crusty-midi-info** (bin)  
  Example tool that dumps midi info.  
  Use:
  - `crusty-midi-tool samples/salsa.mid > dump.txt`
  - `cargo run -p crusty-midi-info -- -f samples/salsa.mid > dump.txt`

## Development

Check out the [linked project](https://github.com/users/sevonj/projects/13) for an overview of issues.

### Continuous Integration

Pull requests are gatekept by [this workflow.](https://github.com/sevonj/crustysynth/blob/master/.github/workflows/rust.yml) It will check if the code

- builds all targets
- passes unit tests (run `cargo test`)
- has linter warnings (run `cargo clippy`)
- is formatted (run `cargo fmt`)
