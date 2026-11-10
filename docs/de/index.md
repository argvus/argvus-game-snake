---
title: Schlange
description: Spiele das Retro-ARGVUS-Terminal-Snake-Spiel.
---
Öffne das Spiel mit:

```sh
argvus game-snake
```
Wenn das Spiel geöffnet wird, drücke `Enter`, um nach rechts zu bewegen, oder drücke `h`, `j`, `k` oder `l`, um sofort in diese Richtung zu starten. Das Anwendungsmenü enthält außerdem einen `ARGVUS Snake`-Launcher.

Der Launcher öffnet das Spiel über das Standard-ARGVUS Kitty-Profil in einem zentrierten schwebenden Fenster der Größe `1380x840`, passend zum Control Center.

`argvus-game-snake` ist ein Terminalspiel im Pixelstil. Die Bewegung verwendet bewusst die Standard-Vi-Tasten:

- `h`: links;
- `j`: unten;
- `k`: oben;
- `l`: rechts.

Die Schlange wächst, wenn sie Nahrung isst. Berührt sie die Wand oder ihren eigenen Körper, endet die Runde. `p` oder Leertaste pausiert, `r` startet nach Game Over neu, und `q` oder Esc beendet das Spiel.

Die lokale Top-10-Rangliste wird in `$XDG_CONFIG_HOME/argvus/games/snake/ranking.json` oder standardmäßig in `~/.config/argvus/games/snake/ranking.json` gespeichert. Die Paketinstallation erstellt keine Dateien im Home-Verzeichnis.
