# CrustySynth
![ci badge](https://github.com/sevonj/crustysynth/actions/workflows/rust.yml/badge.svg)

This project is being developed alongside and for [SfontPlayer.](https://github.com/sevonj/sfontplayer)  
The purpose of CrustySynth is to provide better fitting replacements to some [RustySynth](https://github.com/sinshu/rustysynth/) components, as it has some limitations such as no support for seeking.

## Roadmap:
- [ ] Parse midi files 
- [ ] Play midi files (using RustySynth)

## Crates
- **crustysynth**  
  The main event.
- **crusty-midi-info** (bin)  
  Example tool that dumps midi info.

## Development
Check out the [linked project](https://github.com/users/sevonj/projects/13) for an overview of issues.

### Continuous Integration
Pull requests are gatekept by [this workflow.](https://github.com/sevonj/crustysynth/blob/master/.github/workflows/rust.yml) It will check if the code
- builds all targets
- passes unit tests (run `cargo test`)
- has linter warnings (run `cargo clippy`)
- is formatted (run `cargo fmt`)
