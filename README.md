This program exposes the current temperature and atmospheric pressure read from BMP085 sensor to Prometheus.

### To Compile
I am compiling it on MacOS to Linux with armv7. I choose to statically link musl libc to avoid setting up a full cross compilation toolchain with glibc. The binary size from a release build is 8.5 MB which is acceptable to me.

I use the toolchain from https://github.com/MarioSchwalbe/homebrew-gcc-musl-cross

My `.cargo/config` file:
```toml
[target.armv7-unknown-linux-musleabihf]
linker = "arm-linux-musleabihf-gcc-9"
```

I need extra environment variables to make it compile
```shell script
CC=arm-linux-musleabihf-gcc-9
CFLAGS=-march=armv7+fp
```

Other references that I found useful when trying to fix compiling problems:
- https://sigmaris.info/blog/2019/02/cross-compiling-rust-on-mac-os-for-an-arm-linux-router/
- https://gcc.gnu.org/onlinedocs/gcc/ARM-Options.html