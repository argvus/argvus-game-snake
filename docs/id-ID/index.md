---
title: Ular
description: Mainkan permainan Snake terminal ARGVUS retro.
---

Buka permainan dengan:

```sh
argvus game-snake
```

Saat permainan terbuka, tekan `Enter` untuk mulai bergerak ke kanan, atau tekan `h`, `j`,
`k` atau `l` untuk langsung mulai ke arah itu. Menu aplikasi juga
berisi `ARGVUS Snake` peluncur.

Peluncur membuka permainan melalui profil ARGVUS Kitty standar di jendela mengambang yang dipusatkan
berukuran `1380x840`, sesuai dengan Pusat Kontrol.

`argvus-game-snake` adalah permainan terminal gaya pixel. Gerakannya secara sengaja
menggunakan tombol Vi standar:

- `h`: kiri;
- `j`: turun;
- `k`: naik;
- `l`: benar.

Ular akan tumbuh saat makan makanan. Menyentuh dinding atau tubuh ular itu sendiri
akan mengakhiri putaran. `p` atau Spasi untuk jeda, `r` memulai ulang setelah Game Over, dan `q` atau
Esc keluar.

Peringkat sepuluh besar lokal disimpan dalam
`$XDG_CONFIG_HOME/argvus/games/snake/ranking.json`, atau
`~/.config/argvus/games/snake/ranking.json` secara default. Instalasi paket
tidak membuat file di direktori home.
