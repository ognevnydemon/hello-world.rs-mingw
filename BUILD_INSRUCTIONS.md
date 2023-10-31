to build this project using MinGW-w64 toolchain, you need:
- run `cargo vendor --locked --versioned-dirs` to obtain sources of dependencies (.cargo/config.toml is already set to use vendored deps)
- run `patch -Np0 -i fix-deps.patch` to apply patches
- run `make`
- run `make PREFIX=${MINGW_PREFIX} install` to install executable
