---
title: init
description: "適切なデフォルト値を持つ設定ファイルを生成して、新しいプロジェクトにarchlintを素早くセットアップします。"
---

# archlint init

`init` コマンドは、設定ファイルを生成することで、新しいプロジェクトに archlint を素早くセットアップするのに役立ちます。

## 使用法

```bash
archlint init [options]
```

## オプション

| オプション         | デフォルト | 説明                                                                                 |
| ------------------ | ---------- | ------------------------------------------------------------------------------------ |
| `-f, --force`      | `false`    | 既存の `.archlint.yaml` がある場合に上書きします                                     |
| `--no-interactive` | `false`    | インタラクティブなプリセット選択をスキップします（ステップ2）                        |
| `--presets <list>` | `none`     | フレームワークプリセットを明示的に指定します（カンマ区切り、またはフラグの繰り返し） |

## 仕組み

1. **フレームワークの検出**: archlint は `package.json` の依存関係とプロジェクト構造を分析して、使用されているフレームワークを検出します。
2. **対話的な選択**: `--no-interactive` が使用されない限り、検出されたプリセットの確認や追加のプリセットの選択を求められます。
3. **設定の生成**: 選択されたプリセットと、IDE サポート用の JSON スキーマへの参照を含む `.archlint.yaml` ファイルを作成します。

## 例

### 対話的な初期化

```bash
archlint init
```

### 特定のプリセットを使用した非対話的な初期化

```bash
# カンマ区切り
archlint init --no-interactive --presets nestjs,prisma

# またはフラグの繰り返し
archlint init --no-interactive --presets nestjs --presets prisma
```

### 既存の設定を上書きする

```bash
archlint init --force
```
