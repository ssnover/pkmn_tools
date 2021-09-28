# pkmn_tools

This repo hopes to eventually contain a set of a well tested tools and libraries for working with binary blobs from Pokemon games.

## Organization

The Cargo workspace contains subdirectories defining other crates for manipulating a format of Pokemon binary (e.g. `pk3` for GameBoy Advance generation games like Ruby and FireRed), manipulating save file binaries (e.g. `frlgrse_sav` for GameBoy Advance generation game save files), and currently a single crate which tests the public-facing APIs of all of the other crates (`integration_tests`). The test crate also holds all of the binary files in the repo.

## Help Wanted

Parsing these formats for all of the games is an immense and relatively tedious task. I'm happy to accept pull requests parsing even a single field or adding documentation to just a single function. At this time I'm not looking for submissions of tools which make use of these libraries, those should go in their own separate repos. However if you find these libraries useful and develop a handy tool, let me know and I'll compile a list in this document!

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.