# REST-API
[Enpoint Documentation](docs/api.md)

## Build & run

### Build

```shell
cargo build
```

### Run

```shell
cargo run
```

## Installing diesel_cli & running migrations

### Install

```shell
cargo install diesel_cli --no-default-features --features "sqlite-bundled"
```

### Set database URL

```shell
set DATABASE_URL=rest-api.db
```

### Running migrations

```shell
diesel migration run
```
