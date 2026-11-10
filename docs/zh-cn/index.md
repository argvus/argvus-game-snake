---
title: 蛇
description: 玩复古 ARGVUS 终端贪吃蛇游戏。
---
使用以下方式打开游戏：

```sh
argvus game-snake
```
当游戏打开时，按 `Enter` 开始向右移动，或者按 `h`、`j`、`k` 或 `l` 立即朝该方向移动。应用程序菜单还包含一个 `ARGVUS Snake` 启动器。

启动器通过标准 ARGVUS Kitty 配置文件在一个居中的浮动窗口中打开游戏，窗口大小为 `1380x840`，与控制中心匹配。

`argvus-game-snake` 是一个像素风格的终端游戏。其移动故意使用标准 Vi 按键：

- `h`：左；
- `j`：下；
- `k`：上；
- `l`：右。

蛇吃到食物时会变长。碰到墙壁或蛇自己的身体会结束本轮游戏。`p` 或空格键暂停，`r` 在游戏结束后重新开始，`q` 或 Esc 退出。

本地前十名排名存储在 `$XDG_CONFIG_HOME/argvus/games/snake/ranking.json`，或者默认存储在 `~/.config/argvus/games/snake/ranking.json`。安装软件包不会在主目录中创建文件。
