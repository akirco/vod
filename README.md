## 预览

![](./screenshot.png)

```sh
vod -h

Usage: vod [OPTIONS] --url <URL>

Options:
  -u, --url <URL>        Apple CMS API URL [env: VOD_API_URL="xxx"]
  -a, --action <ACTION>  action [class,detail,videolist] [default: ]
  -t, --t <T>            type ID
  -p, --pg <PG>          page number [default: 1]
  -w, --wd <WD>          search keyword
  -i, --ids <IDS>        IDs for detail action [1,2]
  -j, --json             Output in JSON format , default is false
  -h, --help             Print help
  -V, --version          Print version
```

## 安装（vodx）

```sh
cargo install --git https://github.com/akirco/vod.git
curl -o ~/.local/bin/vodx -fsSL https://raw.githubusercontent.com/akirco/vod/refs/heads/master/vodx

chmod +x ~/.local/bin/vodx

#设置环境变量 VOD_API_URL=https://360zyzz.com/api.php/provide/vod
```

## vodx 使用 (vodx_old)

```
vodx
Usage: vodx [options] <search_keyword>
Example: vodx 绝命毒师

Options:
  -f, --force      Force search, ignore cache
  -c, --clear      Clear cache before search
  -h, --help       Show this help message
```

### fzf 快捷键（vodx）

| 按键        | 功能             |
| ----------- | ---------------- |
| `Tab`       | 下一个分类       |
| `Shift-Tab` | 上一个分类       |
| `Ctrl-n`    | 下一页           |
| `Ctrl-p`    | 上一页           |
| `Ctrl-r`    | 刷新缓存         |
| `Ctrl-d`    | 设为默认分类     |
| `Ctrl-\`    | 清除默认分类     |
| `Ctrl-h`    | 切换历史记录     |
| `/`         | 搜索             |
| `Esc`       | 返回             |
| `?`         | 切换预览窗口位置 |
| `Ctrl-/`    | 隐藏/显示预览    |
| `Enter`     | 播放             |

### 播放选集快捷键 (mpv)

| 按键               | 功能         |
| ------------------ | ------------ |
| `Enter`            | 确认/播放    |
| `↑/↓` 或 `j/k`     | 上/下移动    |
| `←/→` 或 `<` / `>` | 切换上/下集  |
| `q` 或 `Esc`       | 退出         |
| `F8`               | 查看播放列表 |
| `f`                | 全屏         |
| `Space`            | 暂停/播放    |
| `Ctrl-a`           | 全选剧集     |

## 注意

- 可多选播放单选播放
