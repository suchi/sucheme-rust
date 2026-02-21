# ADR-003: Standard Library Implementation in Scheme

**Date:** 2026-02-21  
**Status:** Accepted

## Context

R5RS 付録のマクロやユーティリティ手続きをどこで実装するかの決定。

## Decision

コアで表現できるものはScheme自身で実装し、`scheme/stdlib.scm` に配置する。

## Rationale

- **自己ホスティング精神**: SchemeでできることはSchemeで書く
- **テスタビリティ**: Scheme REPLで直接テスト可能
- **保守性**: Rustのコードを増やさずに機能追加できる
- **R5RS準拠**: `define-syntax`/`syntax-rules` によるマクロが自然に実装できる

## Consequences

- 起動時に `stdlib.scm` を読み込むオーバーヘッドが生じる（軽微）
- バイナリ配布時にSchemeファイルも同梱が必要
