# ADR-004: Web UI with actix-web

**Date:** 2026-02-21  
**Status:** Accepted

## Context

ブラウザからSchemeを実行できるシンプルなWebUIが必要。

## Decision

actix-webを使ったHTTPサーバーを実装し、静的HTMLファイルでシングルページREPLを提供する。

## Rationale

- actix-webはRustの成熟した非同期Webフレームワーク
- シングルバイナリで完結（外部サーバー不要）
- `/api/eval` エンドポイントでScheme評価をJSONで提供
- フロントエンドは純粋なHTML/CSS/JSで依存関係なし

## Consequences

- セッション管理が未実装のため、各リクエストが独立した環境で評価される
- 本番利用には認証・レート制限が必要（現時点では省略）
