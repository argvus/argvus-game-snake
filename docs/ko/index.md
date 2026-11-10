---
title: 뱀
description: 레트로 ARGVUS 터미널 스네이크 게임을 플레이하세요.
---
다음으로 게임을 여세요:

```sh
argvus game-snake
```
게임을 열면 오른쪽으로 움직이려면 `Enter`를 누르거나 `h`, `j`, `k` 또는 `l`를 눌러 즉시 해당 방향으로 시작할 수 있습니다. 애플리케이션 메뉴에는 `ARGVUS Snake` 실행기 또한 포함되어 있습니다.

실행기는 표준 ARGVUS Kitty 프로필을 통해 `1380x840` 크기의 중앙에 떠 있는 창으로 게임을 엽니다. 이는 제어 센터와 일치합니다.

`argvus-game-snake`는 픽셀 스타일의 터미널 게임입니다. 이동은 의도적으로 표준 Vi 키를 사용합니다:

- `h`: 왼쪽;
- `j`: 아래;
- `k`: 위;
- `l`: 오른쪽.

뱀은 음식을 먹으면 성장합니다. 벽이나 뱀 자신의 몸에 닿으면 라운드가 종료됩니다. `p` 또는 스페이스바는 일시정지, `r`는 게임 오버 후 재시작, `q` 또는 Esc는 종료합니다.

지역 상위 10위 순위는 `$XDG_CONFIG_HOME/argvus/games/snake/ranking.json` 또는 기본적으로 `~/.config/argvus/games/snake/ranking.json`에 저장됩니다. 패키지 설치는 홈 디렉터리에 파일을 생성하지 않습니다.
