---
title: Yılan
description: Retro ARGVUS terminal Yılan oyununu oynayın.
---

Oyunu açmak için:

```sh
argvus game-snake
```

Oyun açıldığında, sağa hareket etmeye başlamak için `Enter` tuşuna basın veya o yönde hemen başlamak için `h`, `j`,
`k` veya `l` tuşlarına basın. Uygulama menüsü ayrıca
bir `ARGVUS Snake` başlatıcı içerir.

Başlatıcı, oyunu standart ARGVUS Kitty profili üzerinden, Kontrol Merkezi ile eşleşen
merkezli, `1380x840` boyutunda kayan bir pencerede açar.

`argvus-game-snake` pikselli tarzda bir terminal oyunudur. Hareketleri kasıtlı olarak
standart Vi tuşlarını kullanır:

- `h`: sol;
- `j`: aşağı;
- `k`: yukarı;
- `l`: doğru.

Yılan yemek yediğinde büyür. Duvara veya yılanın kendi vücuduna dokunmak
turunu bitirir. `p` veya Space duraklatır, `r` Game Over'dan sonra yeniden başlatır ve `q` veya
Esc çıkar.

Yerel ilk-on sıralaması şurada saklanır
`$XDG_CONFIG_HOME/argvus/games/snake/ranking.json`, veya
`~/.config/argvus/games/snake/ranking.json` varsayılan olarak. Paket kurulumu
ev dizininde dosya oluşturmaz.
