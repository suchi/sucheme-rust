# ADR-001: Use Rust for Scheme Interpreter Implementation

**Date:** 2026-02-21  
**Status:** Accepted

## Context

R5RS準拠のSchemeインタープリターをどの言語で実装するかの決定。

## Decision

Rustを使用する。

## Rationale

- **メモリ安全性**: GCなしでのメモリ管理、Rc<RefCell<>>でSchemeの共有可変状態を安全に扱える
- **パフォーマンス**: コンパイル済みバイナリで高速実行
- **エコシステム**: actix-web (Web), rustyline (REPL), serde (JSON) の充実したクレート
- **型システム**: enum によるScheme値型の自然なモデリング

## Consequences

- Rustのライフタイムシステムにより、Schemeの環境チェーン実装に `Rc<RefCell<>>` パターンが必要
- 完全な継続 (call/cc) の実装が複雑になる（初期フェーズでは制限付きのエスケープ継続のみ）
