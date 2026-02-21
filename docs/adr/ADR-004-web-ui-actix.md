# ADR-004: Web UIにactix-webを採用

**日付:** 2026-02-21
**ステータス:** 承認済み

## 背景

ブラウザベースのREPLを提供するために、Rustで動作するHTTPサーバーフレームワークが必要だった。

## 候補の比較

```mermaid
graph TD
    Q["Rust HTTPフレームワークの選択"] --> ACTIX["actix-web"]
    Q --> AXUM["axum"]
    Q --> WARP["warp"]
    Q --> TINY["tiny_http"]

    ACTIX --> A1["✅ 実績豊富"]
    ACTIX --> A2["✅ 高パフォーマンス"]
    ACTIX --> A3["✅ 豊富なドキュメント"]
    ACTIX --> A4["❌ やや複雑なAPI"]

    AXUM --> X1["✅ tokioエコシステム統合"]
    AXUM --> X2["✅ シンプルなAPI"]
    AXUM --> X3["❌ 比較的新しい"]

    WARP --> W1["✅ 型安全なルーティング"]
    WARP --> W2["❌ マクロが複雑"]

    TINY --> T1["✅ 最小依存"]
    TINY --> T2["❌ 機能が少ない"]
```

## 決定

**actix-webを使用する。**

## Web UIのアーキテクチャ

```mermaid
sequenceDiagram
    participant B as ブラウザ
    participant S as actix-web
    participant E as Scheme評価器
    participant F as ファイルシステム

    B->>S: GET /
    S->>F: web/static/index.html を読み込み
    F-->>S: HTMLコンテンツ
    S-->>B: HTMLレスポンス

    B->>S: POST /api/eval {"code": "(+ 1 2)"}
    S->>E: eval_str("(+ 1 2)")
    E-->>S: Val::Int(3)
    S-->>B: {"result": "3", "error": null}
```

## 理由

- **実績**: crates.io で最もダウンロードされているRust Webフレームワーク
- **パフォーマンス**: TechEmpower ベンチマークでトップクラス
- **機能**: 静的ファイル配信、JSON サポート、ミドルウェアが充実
- **ドキュメント**: 豊富なサンプルと活発なコミュニティ

## 結果

- `actix-web` の `actix_rt` でメインスレッドをブロックせずにサーバーを起動
- セキュリティ: 入力コードは100KB に制限
- 評価はタイムアウトなし（将来の課題）
- Web UI は `--web` フラグでのみ有効になる（通常起動には影響なし）
