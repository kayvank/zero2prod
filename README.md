zero2prod
--
A toy application based on [Zero To Production In Rust](https://www.zero2prod.com/index.html?country_code=US)

## How to use it

``` sh
nix develop
docker-compose up -d
cargo build
cargo test
cargo run
```
to run the build without database, use the sqlx offline mode:

``` sh
SQLX_OFFLINE=true cargo build
```

