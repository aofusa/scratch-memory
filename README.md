Scratch Memory
=====


scratchコンテナで動作するメモリ確保を行うサンプル
no_std環境でmmapシステムコールを直接呼び出してみる


- ビルドと実行
```sh
docker build -t dev.local/scratch-memory:latest .
docker run dev.local/scratch-memory
```

参考
-----
- [hello-worldコンテナ](https://hub.docker.com/_/hello-world])
- [Rust でしっかりとスタティックリンク](https://qiita.com/moriai/items/b1fa7d1b43d985d408cc)
- [sc](https://github.com/japaric/syscall.rs/blob/master/src/platform/linux-x86_64/mod.rs)
- [low-level-programming](https://github.com/Apress/low-level-programming/blob/master/listings/chap4/mmap/mmap.asm)
- [sys/mman.h](https://sites.uclouvain.be/SystInfo/usr/include/bits/mman.h.html)
- [rustix](https://github.com/bytecodealliance/rustix/blob/main/src/backend/linux_raw/param/auxv.rs)

