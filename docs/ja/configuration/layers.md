---
title: レイヤー
description: "プロジェクトのアーキテクチャレベルを定義し、厳密な依存関係ルールを強制して、クリーンなアーキテクチャを維持し、結合を防ぎます。"
---

# レイヤー

レイヤー設定を使用すると、プロジェクトのアーキテクチャレベルを定義し、それらの間の依存関係ルールを強制できます。

## レイヤーの定義

レイヤーは `layer_violation` ルール内で設定されます。各レイヤー定義は以下で構成されます。

- `name`: レイヤーの一意の名前。
- `path` (または `paths`): このレイヤーのファイルを特定する glob パターン。
- `allowed_imports` (または `can_import`): このレイヤーがインポートを許可されているレイヤー名のリスト。

## 例: クリーンアーキテクチャ (Clean Architecture)

```yaml
rules:
  layer_violation:
    severity: high
    layers:
      - name: domain
        path: '**/domain/**'
        allowed_imports: [] # Domain レイヤーは何にも依存してはいけません

      - name: application
        path: '**/application/**'
        allowed_imports:
          - domain

      - name: infrastructure
        path: '**/infrastructure/**'
        allowed_imports:
          - domain
          - application

      - name: presentation
        path: '**/presentation/**'
        allowed_imports:
          - domain
          - application
```

## 仕組み

`layer_violation` 検出器が有効な場合:

1. `path` パターンに基づいて、プロジェクト内のすべてのファイルを特定のレイヤーにマッピングします。
2. ファイルが複数のパターンに一致する場合、最も具体的なもの (最も長いパターン) が選択されます。
3. ツールはすべてのインポートをチェックします。レイヤー `A` のファイルがレイヤー `B` のファイルをインポートし、`B` がレイヤー `A` の `allowed_imports` リストにない場合、違反が報告されます。
