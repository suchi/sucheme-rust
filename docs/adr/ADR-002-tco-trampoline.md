# ADR-002: Tail Call Optimization via Trampoline Loop

**Date:** 2026-02-21  
**Status:** Accepted

## Context

R5RS はプロパーテール呼び出し (proper tail calls) を必須要件としている。  
Rustはネイティブな末尾呼び出し最適化をサポートしていない。

## Decision

評価器 (`eval.rs`) にトランポリンループパターンを使用する。  
`loop { expr = ...; env = ...; continue; }` の形式で末尾呼び出しをイテレーションに変換。

## Rationale

- Rustのスタックを消費せずに深い再帰をサポートできる
- 実装が単純：`expr` と `env` を更新して `continue` するだけ
- 評価器の主ループが1つになり見通しが良い

## Consequences

- `apply()` 関数は内部用途（`map`等）のため完全TCOではない
- 末尾再帰の10万回呼び出しテストで動作確認済み
