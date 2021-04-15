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

### Set enviromental variables
Use either the file `.env.example` and modify to your needs
```shell
cp .env.example .env
```
Or set them manually
```shell
set DATABASE_URL=rest-api.db
```

### Running migrations

```shell
diesel migration run
```

### Loading data
Restaurant data can be loaded with the argument `load` followed by the path to the json file to be loaded

```shell
cargo run load path/to/json_file.json
```


## Production
1. Make sure that you have `x86_64-unknown-linux-musl` in your rust target chain:
```
rustup target add x86_64-unknown-linux-musl
```
2. Run the deploy script
```
./deploy.sh
```