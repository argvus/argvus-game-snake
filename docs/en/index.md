---
title: Snake
description: Play the retro ARGVUS terminal Snake game.
---

Open the game with:

```sh
argvus game-snake
```

When the game opens, press `Enter` to start moving right, or press `h`, `j`,
`k` or `l` to start immediately in that direction. The application menu also
contains an `ARGVUS Snake` launcher.

The launcher opens the game through the standard ARGVUS Kitty profile in a
centered floating window sized `1380x840`, matching the Control Center.

`argvus-game-snake` is a pixel-style terminal game. Its movement deliberately
uses the standard Vi keys:

- `h`: left;
- `j`: down;
- `k`: up;
- `l`: right.

The snake grows when it eats food. Touching the wall or the snake's own body
ends the round. `p` or Space pauses, `r` restarts after Game Over, and `q` or
Esc exits.

The local top-ten ranking is stored in
`$XDG_CONFIG_HOME/argvus/games/snake/ranking.json`, or
`~/.config/argvus/games/snake/ranking.json` by default. Package installation
does not create files in the home directory.
