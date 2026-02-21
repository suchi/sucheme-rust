# ADR-003: 標準ライブラリをScheme自身で実装

**日付:** 2026-02-21
**ステータス:** 承認済み

## 背景

R5RS付録マクロや高階手続きをどこで実装するかの決定が必要だった。

## 候補アプローチの比較

```mermaid
graph TD
    Q["標準ライブラリの実装方法"] --> RUST_IMPL["Rustで全部実装"]
    Q --> SCHEME_IMPL["Schemeで実装"]
    Q --> HYBRID["ハイブリッド"]

    RUST_IMPL --> R1["✅ 高速"]
    RUST_IMPL --> R2["❌ コード量が膨大"]
    RUST_IMPL --> R3["❌ マクロの実装が複雑"]

    SCHEME_IMPL --> S1["✅ コード量が少ない"]
    SCHEME_IMPL --> S2["✅ 自己記述的 (dogfooding)"]
    SCHEME_IMPL --> S3["✅ マクロが自然に書ける"]
    SCHEME_IMPL --> S4["❌ 起動時にロードが必要"]

    HYBRID --> H1["✅ バランスが良い"]
    HYBRID --> H2["コア = Rust, 高レベル = Scheme"]
```

## 決定

**コア手続きはRustで実装し、R5RS付録マクロと高レベルユーティリティはSchemeで実装する。**

## 対応表

```mermaid
graph LR
    subgraph "Rust実装 (builtins.rs)"
        B1["算術: + - * /"]
        B2["リスト: car cdr cons"]
        B3["比較: eq? equal? ="]
        B4["文字列・文字・ベクター"]
        B5["I/O: display write read"]
    end
    subgraph "Scheme実装 (stdlib.scm)"
        S1["define-record-type"]
        S2["let-values"]
        S3["when unless"]
        S4["fold-left fold-right"]
        S5["filter partition"]
        S6["sort"]
        S7["iota"]
        S8["string-split join"]
    end
```

## 理由

- SchemeでSchemeを実装することで仕様への準拠度を高検証できる
- マクロ (`define-syntax`) はRustより Schemeで書く方が自然
- 既存の実装 (MIT Scheme, Guile) でも同様のアプローチが採用されている

## 結果

- 起動時に `scheme/stdlib.scm` のロードが必要 (数ミリ秒)
- stdlib.scm はユーザーが拡張可能
- ファイルが見つからなくてもコアは動作する
