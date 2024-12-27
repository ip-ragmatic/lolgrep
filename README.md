# Hello, `lolgrep`

`lolgrep` is a command-line utility whose sole purpose is search for a query inside specified files.
It is 100% based on `grep` (and subsequently `ripgrep`). If `lolgrep` finds a matchm it will return
any line numbers at which this query occurs. I am building this to become more familiar with Rust
and building things from scratch.

`lolgrep` is really bare bones at the moment, but I do plan on expanding its capabilities.

## TODO

- [ ] Intelligently detect flags to alter how `lolgrep`.
- [ ] Add flags!
- [ ] Add default case where providing now file path recursively searches **ONLY** the current
  directory (not recursively through its children).
- [ ] Add query highlighting in the line where the match is found.

> By the way, I'm not sure if I'm using the right license here. I just wanted to start getting in
> the habbit of adding a license to any projects I start.
