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

## パターン

```
let (x, y) = (1, 2);
println!("{}, {}", x, y);
```

## 型アノテーション

Rustは型推論してくれるが
:の後に型を書くことも出来る

```
let x: i32 = 5;
```

# 可変性

デフォルトで束縛はイミュータブル

```
let x = 5;
x = 10;
```
コンパイルエラー

ミュータブルにしたいならmutを使う

```
let mut x = 5;
x = 10;
println!("{}", x);
```

# スコープとシャドーイング

変数束縛にはスコープがある。
変数束縛は定義されたブロック内でしか有効でない。
ブロックは { と } に囲まれた箇所。
関数定義もブロック。
ブロック内で定義された変数は外のブロックからはアクセスできない。

```
fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("x is {}, y is {}", x, y);
    }
    println!("x is {}, y is {}", x, y); // Error
}
```

# 関数

関数の引数は let 宣言と非常によく似た動き。

引数の名前にコロンに続けて型を追加。

```
fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}
```

let と異なり、あなたは関数の引数の型を宣言しなければならない 

```
fn print_sum(x, y) {
    println!("sum is: {}", x + y);
}
```

これはコンパイルエラー

戻り値

```
fn add_one(x: i32) -> i32 {
    x + 1
}
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

# タプル

```
let tuple = (100, "hello");
let (number, word) = tuple;

println!("{:?}", tuple);
println!("{}", tuple.0);
println!("{}", tuple.1);
```

## タプルを返す関数

```
fn make_tuple() -> (u32, String) {
    (100, "hello".to_string())
}
```

## 1要素のタプル

若干奇妙に思えるが、Rustでは1要素のタプルを定義できる。

この場合タプルの要素の後ろに,を置く必要がある。


```
let sigle_tuple = ("hello",)
```
