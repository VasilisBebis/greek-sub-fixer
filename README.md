# Why?
Because Greek subs downloaded from the internet have encoding inconsistencies and most of the times don't work out of the box (with VLC for example). This program converts the `.srt` file to UTF-8 encoding.

It also replaces `’` (`U+2019`) to `Ά` (`U+0386`), which is a widely known problem in a lot of Greek subtitles (due to the ISO 8859-7 and Windows-1253 encodings colliding). There are a lot of other problems that I will try to locate and fix soon

If you are someone who likes to write subtitles please switch your Windows to English and only use UTF-8.

# How

To compile
```shell
cargo build --release
```

To fix `.srt` files in a directory pass it as an argument to the executable (doesn't support recursive searching for files in the directory for now):
```shell
greek-sub-fixer path/to/subs
```
