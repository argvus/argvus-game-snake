# ARGVUS Game Snake

Retro terminal Snake game for ARGVUS. The game uses the classic Vi movement
keys `h`, `j`, `k` and `l` to encourage keyboard-first navigation.

[![CI](https://github.com/argvus/argvus-game-snake/actions/workflows/ci.yml/badge.svg)](https://github.com/argvus/argvus-game-snake/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/License-GPL--3.0-blue.svg)](LICENSE)

The repository contains the deterministic game engine, Ratatui terminal UI,
local ranking storage, and Arch package tooling.

## Install

```sh
sudo pacman -S argvus-game-snake
```

The package installs `/usr/lib/argvus/commands/argvus-game-snake`. The ARGVUS dispatcher also
opens it with:

```sh
argvus game-snake
```

## Controls

- When the game opens, press `Enter` to start moving right, or press one of
  `h`, `j`, `k` or `l` to start immediately in that direction.
- `h`, `j`, `k`, `l`: move left, down, up and right.
- `p` or Space: pause and resume.
- `r`: restart after Game Over.
- `q` or Esc: quit.

Scores are kept locally in
`$XDG_CONFIG_HOME/argvus/games/snake/ranking.json`, or
`~/.config/argvus/games/snake/ranking.json` by default.

## Build from source

```sh
make check
make build
```

The local package builder expects the sibling `argvus` checkout under the same
`de/` directory; its crates are path dependencies.

## Workspace

```text
crates/
  core/       Snake rules and ranking persistence
  main/       Ratatui application and terminal event loop
packaging/
  arch/
    local/    package built from the current working tree
    ci/       package built from a version tag
tools/sh/     local source archive and makepkg driver
```

## License

SPDX: `GPL-3.0-only`. See [LICENSE](LICENSE).
