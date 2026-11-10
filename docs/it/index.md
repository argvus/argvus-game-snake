---
title: Serpente
description: Gioca al gioco Snake del terminale retro ARGVUS.
---
Apri il gioco con:

```sh
argvus game-snake
```
Quando si apre il gioco, premi `Enter` per iniziare a muoverti a destra, oppure premi `h`, `j`, `k` o `l` per partire immediatamente in quella direzione. Il menu dell'applicazione contiene anche un launcher `ARGVUS Snake`.

Il launcher apre il gioco tramite il profilo standard ARGVUS Kitty in una finestra flottante centrata di dimensione `1380x840`, corrispondente al Centro di Controllo.

`argvus-game-snake` è un gioco in stile terminal a pixel. I movimenti utilizzano deliberatamente i tasti standard di Vi:

- `h`: sinistra;
- `j`: giù;
- `k`: su;
- `l`: destra.

Il serpente cresce quando mangia il cibo. Toccare il muro o il corpo del serpente termina il round. `p` o Spazio mette in pausa, `r` riavvia dopo Game Over, e `q` o Esc esce.

La classifica locale dei primi dieci è memorizzata in `$XDG_CONFIG_HOME/argvus/games/snake/ranking.json`, o in `~/.config/argvus/games/snake/ranking.json` per impostazione predefinita. L'installazione del pacchetto non crea file nella directory home.
