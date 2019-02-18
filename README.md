# Codename: Molva

__Description__: substrate example


### Tools

Install Rust v 1.31.0

```
sudo apt-get update && apt-get install -y curl gcc clang`
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable`
```

### Build

```$xslt
./build.sh
```

#### Frontend

```
cd frontend
yarn start
```

### Usage

```
cd dev/back
cargo build
cargo run 
```