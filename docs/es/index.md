---
title: Serpiente
description: Juega al juego de la serpiente en el terminal retro ARGVUS.
---
Abre el juego con:

```sh
argvus game-snake
```
Cuando se abre el juego, presiona `Enter` para comenzar a moverte a la derecha, o presiona `h`, `j`, `k` o `l` para comenzar inmediatamente en esa dirección. El menú de la aplicación también contiene un lanzador `ARGVUS Snake`.

El lanzador abre el juego a través del perfil estándar ARGVUS Kitty en una ventana flotante centrada de tamaño `1380x840`, coincidiendo con el Centro de Control.

`argvus-game-snake` es un juego de terminal de estilo pixelado. Su movimiento utiliza deliberadamente las teclas estándar de Vi:

- `h`: izquierda;
- `j`: abajo;
- `k`: arriba;
- `l`: derecha.

La serpiente crece cuando come comida. Tocar la pared o el propio cuerpo de la serpiente termina la ronda. `p` o Espacio pausa, `r` reinicia después de Game Over, y `q` o Esc sale.

La clasificación local de los diez mejores se almacena en `$XDG_CONFIG_HOME/argvus/games/snake/ranking.json`, o en `~/.config/argvus/games/snake/ranking.json` por defecto. La instalación del paquete no crea archivos en el directorio de inicio.
