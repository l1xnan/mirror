# Mirror Manager
管理 pypi, npm 等各种国内镜像

```bash
$ mirror help
Usage: mirror.exe [OPTIONS] <COMMAND>

Commands:
  pypi  配置 python pypi
  npm   配置 nodejs npm
  help  Print this message or the help of the given subcommand(s)

Options:
  -c, --config <FILE>  Sets a custom config file
  -h, --help           Print help
  -V, --version        Print version
```


```bash
$ mirror pypi test
start test pypi mirror speed...
=============================
Mirror: bfsu
Downloaded 10.65 MiB in 0.45 s (23.42 MiB/s)
=============================
Mirror: ustc
Downloaded 10.65 MiB in 0.60 s (17.89 MiB/s)
=============================
Mirror: douban
Downloaded 10.65 MiB in 1.16 s (9.18 MiB/s)
=============================
Mirror: huawei
Downloaded 10.65 MiB in 0.47 s (22.65 MiB/s)
=============================
Mirror: tuna
File Size: 10.65 MiB
Downloaded 10.65 MiB in 0.43 s (24.75 MiB/s)
=============================
Mirror: aliyun
Downloaded 10.65 MiB in 3.33 s (3.20 MiB/s)
```


```bash
$ mirror pypi conf tuna
Exec: pip config set global.index-url https://pypi.tuna.tsinghua.edu.cn/simple
Writing to C:\Users\Administrator\AppData\Roaming\pip\pip.ini
```