# Rust

Rust Compiler

```
rustc main.rs
```

# Cargo

## Build

```
cargo build
cargo build --release
```

## run

```
cargo run
```

## new

```
cargo new app --bin
```


# let

```
let foo = bar // イミュータブル
let mut foo = bar // ミュータブル
```


# スタティックメソッド

```
String::new()
```

# クレート

クレートはRustのコードのパッケージ

## Cargo.toml

```
[dependencies]

rand="0.3.0"
```

## バージョン

```
#0.3.0と互換性のあるバージョン
^0.3.0

#0.3.0のみ
=0.3.0
```

## extern crate

```
extern crate rand;

use rand::Rng;
```

# シャドーイング

既に定義した変数を同じ名前で定義し直せる

```
let mut guess = String::new();
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```
