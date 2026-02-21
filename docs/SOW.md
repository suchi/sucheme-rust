# Statement of Work (SOW)

## Project: sucheme-rust

**Date:** 2026-02-21  
**Version:** 1.0

---

## 1. Project Overview

RustでR5RS準拠のSchemeインタープリターを実装する。  
目標は、R5RS仕様書（付録マクロ含む）に完全準拠したScheme処理系を作ることである。

---

## 2. Objectives

1. **最小限のREPL** – 対話的に式を入力・評価できる環境
2. **コアScheme構文** – R5RS必須構文・手続きの実装
3. **Schemeスクリプトによる拡張** – コアで表現できるものはSchemeで実装
4. **WebUI** – ブラウザから実行できるシンプルなWebアプリ
5. **CI/CD** – GitHub ActionsによるCI・GitHub Releaseの自動化

---

## 3. Scope

### In Scope

| カテゴリ | 内容 |
|---------|------|
| 字句解析 | R5RS準拠のトークナイザー |
| 構文解析 | S式パーサー |
| 評価器 | 末尾呼び出し最適化 (TCO) 付き評価器 |
| 特殊形式 | `define`, `lambda`, `if`, `cond`, `case`, `and`, `or`, `let`, `let*`, `letrec`, `begin`, `do`, `delay`, `quasiquote`, `set!` 等 |
| 組み込み手続き | 算術、リスト、文字列、文字、ベクター、I/O 等 (R5RS全手続き) |
| マクロ | `syntax-rules` ベースのマクロシステム |
| R5RS付録マクロ | `when`, `unless`, `let-values`, `define-record-type` 等 |
| REPL | rustyline使用、Emacsバインディング、履歴 |
| WebUI | actix-webによるHTTPサーバー + ブラウザREPL |
| CI/CD | GitHub Actionsでのビルド・テスト・リリース |
| ドキュメント | ADR、オンボーディングガイド |

### Out of Scope (初期)

- 完全なcall/cc (継続)
- 完全なI/Oポートシステム
- ガベージコレクター（Rustの所有権システムを活用）
- マルチスレッド

---

## 4. Deliverables

| #  | 成果物 | 説明 |
|----|-------|------|
| D1 | `src/lexer.rs` | 字句解析器 |
| D2 | `src/parser.rs` | 構文解析器 |
| D3 | `src/types.rs` | Scheme値型定義 |
| D4 | `src/eval.rs` | 評価器（TCO付き） |
| D5 | `src/builtins.rs` | 組み込み手続き群 |
| D6 | `src/repl.rs` | 対話的REPL |
| D7 | `src/web.rs` | WebサーバーAPI |
| D8 | `scheme/stdlib.scm` | Schemeで実装した標準ライブラリ |
| D9 | `web/static/index.html` | WebUIフロントエンド |
| D10 | `.github/workflows/ci.yml` | CI設定 |
| D11 | `.github/workflows/release.yml` | リリース設定 |
| D12 | `docs/` | ADR・オンボーディングドキュメント |

---

## 5. Implementation Phases

### Phase 1: プロジェクト基盤
- Cargoプロジェクト初期化
- 依存関係設定（rustyline, actix-web, serde）
- .editorconfig, .gitignore設定

### Phase 2: コアインタープリター
- 型定義 (Val, Env, Proc)
- 字句解析器
- 構文解析器
- 評価器（TCO付き）
- 組み込み手続き

### Phase 3: REPL
- rustylineを使ったライン編集
- Emacsキーバインディング
- コマンド履歴

### Phase 4: Scheme標準ライブラリ
- R5RS由来のSchemeマクロ・手続き
- `scheme/stdlib.scm` として実装

### Phase 5: WebUI
- actix-webによるHTTPサーバー
- シングルページWebアプリケーション
- `/api/eval` エンドポイント

### Phase 6: CI/CD
- GitHub Actions CI (ビルド、テスト、lint)
- GitHub Releaseの自動化

---

## 6. Acceptance Criteria

- [ ] R5RS必須手続きが全て動作する
- [ ] 再帰的なフィボナッチ・階乗が動作する
- [ ] 末尾再帰で100,000回の呼び出しがスタックオーバーフローしない
- [ ] REPL でのマルチライン入力が動作する
- [ ] WebUIでScheme式が評価できる
- [ ] CI が全テストをパスする

---

## 7. Timeline

| フェーズ | 期間 |
|---------|------|
| Phase 1 | Week 1 |
| Phase 2 | Week 1-2 |
| Phase 3 | Week 2 |
| Phase 4 | Week 2-3 |
| Phase 5-6 | Week 3 |

---

## 8. Risks

| リスク | 影響 | 対策 |
|-------|------|------|
| R5RS仕様の複雑性 | 高 | 段階的実装、既存テストスイートを参考 |
| Rustライフタイム管理 | 中 | Rc<RefCell<>> による共有可変参照 |
| マクロシステム実装 | 中 | syntax-rulesをevalで処理 |
