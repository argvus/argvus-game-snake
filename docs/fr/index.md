---
title: Serpent
description: Jouez au jeu de serpent rétro du terminal ARGVUS.
---
Ouvrez le jeu avec :

```sh
argvus game-snake
```
Lorsque le jeu s'ouvre, appuyez sur `Enter` pour commencer à aller à droite, ou appuyez sur `h`, `j`, `k` ou `l` pour démarrer immédiatement dans cette direction. Le menu de l'application contient également un lanceur `ARGVUS Snake`.

Le lanceur ouvre le jeu via le profil standard ARGVUS Kitty dans une fenêtre flottante centrée de taille `1380x840`, correspondant au Centre de Contrôle.

`argvus-game-snake` est un jeu de terminal de style pixel. Ses déplacements utilisent délibérément les touches standard Vi :

- `h` : gauche ;
- `j` : bas ;
- `k` : haut ;
- `l` : droite.

Le serpent grandit lorsqu'il mange de la nourriture. Toucher le mur ou le corps du serpent met fin à la partie. `p` ou la barre d'espace met en pause, `r` redémarre après Game Over, et `q` ou Échap quitte.

Le classement local des dix meilleurs est stocké dans `$XDG_CONFIG_HOME/argvus/games/snake/ranking.json`, ou `~/.config/argvus/games/snake/ranking.json` par défaut. L'installation du package ne crée pas de fichiers dans le répertoire personnel.
