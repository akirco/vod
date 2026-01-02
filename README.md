苹果 cms 接口地址：

分类: ?ac=class

列表: ?ac=videolist&t=${t}&pg=${pg}

搜索: ?ac=detail&wd=${wd}

详情: ?ac=detail&ids=${id}

```shell
❯ vod -h

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

```sh
cargo install --git https://github.com/akirco/vod.git
curl -o ~/.local/bin/vodx -fsSL https://raw.githubusercontent.com/akirco/vod/refs/heads/master/vodx
chmod +x ~/.local/bin/vodx
```
