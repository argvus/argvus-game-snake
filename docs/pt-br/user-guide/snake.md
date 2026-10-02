---
title: Snake
description: Jogue o Snake retro do ARGVUS no terminal.
slug: pt/0.4.0/docs/user-guide/applications/snake
---

Abra o jogo com:

```sh
argvus game-snake
```

Quando o jogo abrir, pressione `Enter` para começar movendo para a direita, ou
pressione `h`, `j`, `k` ou `l` para começar imediatamente nessa direção. O menu
de aplicativos também contém o lançador `Snake ARGVUS`.

O lançador abre o jogo pelo perfil padrão do Kitty do ARGVUS em uma janela
flutuante centralizada de `1380x840`, igual à Central de Controle.

O `argvus-game-snake` é um jogo pixelado no terminal. A movimentação usa de
propósito as teclas padrão do editor Vi:

* `h`: esquerda;
* `j`: baixo;
* `k`: cima;
* `l`: direita.

A minhoca cresce ao comer a comida. Encostar na parede ou no próprio corpo
encerra a rodada. `p` ou Espaço pausa, `r` reinicia após o fim de jogo, e `q`
ou Esc sai.

O ranking local com as dez melhores pontuações fica em
`$XDG_CONFIG_HOME/argvus/games/snake/ranking.json`, ou em
`~/.config/argvus/games/snake/ranking.json` por padrão. A instalação do pacote
não cria arquivos na pasta pessoal.
