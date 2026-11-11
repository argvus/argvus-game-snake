---
title: Snake
description: Chơi trò chơi rắn trên terminal ARGVUS cổ điển.
---

Mở trò chơi bằng:

```sh
argvus game-snake
```

Khi trò chơi mở, nhấn `Enter` để bắt đầu di chuyển sang phải, hoặc nhấn `h`, `j`,
`k` hoặc `l` để bắt đầu ngay theo hướng đó. Menu ứng dụng cũng
chứa một trình khởi chạy `ARGVUS Snake`.

Trình khởi chạy mở trò chơi thông qua hồ sơ ARGVUS Kitty tiêu chuẩn trong một cửa sổ nổi kích thước `1380x840`, căn giữa
, phù hợp với Trung tâm Điều khiển.

`argvus-game-snake` là một trò chơi terminal phong cách pixel. Chuyển động của nó cố ý
sử dụng các phím Vi tiêu chuẩn:

- `h`: left;
- `j`: down;
- `k`: up;
- `l`: right.

Con rắn sẽ lớn lên khi nó ăn thức ăn. Chạm vào tường hoặc vào cơ thể của chính con rắn
sẽ kết thúc vòng chơi. `p` hoặc phím cách tạm dừng, `r` khởi động lại sau khi Game Over, và `q` hoặc
Esc exits.

Bảng xếp hạng mười người chơi hàng đầu cục bộ được lưu trong
`$XDG_CONFIG_HOME/argvus/games/snake/ranking.json`, hoặc
`~/.config/argvus/games/snake/ranking.json` theo mặc định. Cài đặt gói
không tạo tệp trong thư mục chính.
