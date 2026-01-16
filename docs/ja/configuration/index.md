---
title: 設定
description: .archlint.yaml を使用して archlint を設定し、アーキテクチャレイヤーを定義し、検出器のルールを設定する方法について説明します。
---

# 設定

archlint は、プロジェクトルートにある `.archlint.yaml` ファイルを使用して設定できます。設定ファイルが見つからない場合、ツールはすべての検出器に対して適切なデフォルト値を使用します。

## 設定ファイルの構造

```yaml
# 無視するファイルとディレクトリ (グローバル)
ignore:
  - '**/dist/**'
  - '**/node_modules/**'

# パスエイリアス (tsconfig.json や webpack と同様)
# デフォルトでは、archlint は tsconfig.json からエイリアスを自動的に読み込みます。
# ここで明示的に定義されたエイリアスは、tsconfig.json から派生した値よりも優先されます。
aliases:
  '@/*': 'src/*'

# TypeScript 統合設定 (true、false、またはファイルパス)
tsconfig: true

# 組み込みまたはカスタムプリセットから拡張
extends:
  - nestjs
  - ./my-company-preset.yaml

# 解析のエントリポイント (デッドコード検出に使用)
entry_points:
  - 'src/main.ts'

# 各検出器のルール設定
rules:
  # 短縮形式: 重要度レベルまたは "off"
  cycles: high
  dead_code: medium

  # 完全形式: 追加オプション付き
  god_module:
    severity: high
    enabled: true
    exclude: ['**/generated/**']
    # 検出器固有のオプション
    fan_in: 15
    fan_out: 15
    churn: 20

  dead_symbols:
    severity: high
    # インターフェースメソッドの設定（未使用の実装による誤検知を回避）
    contract_methods:
      MyInterface: ['method1', 'method2']
      ValidatorConstraintInterface: ['validate', 'defaultMessage']

  vendor_coupling:
    severity: medium
    ignore_packages: ['lodash', 'rxjs']

# 特定のパスに対するルールのオーバーライド
overrides:
  - files: ['**/legacy/**']
    rules:
      cyclomatic_complexity: medium
      god_module: off

# スコアリングとグレーディングの設定
scoring:
  # 報告する最小重要度レベル (low, medium, high, critical)
  minimum: low
  # 総スコア計算の重み
  weights:
    critical: 100
    high: 50
    medium: 20
    low: 5
  # グレーディングの閾値 (密度 = 総スコア / ファイル数)
  grade_rules:
    excellent: 1.0
    good: 3.0
    fair: 7.0
    moderate: 15.0
    poor: 30.0

# フレームワークの自動検出 (デフォルトは true)
auto_detect_framework: true

# アーキテクチャ差分の設定
diff:
  # メトリクスが悪化したとみなされるパーセンテージのしきい値
  metric_threshold_percent: 20
  # ファジー差分で同じ臭いとみなす最大行移動数
  line_tolerance: 50

# Git 設定
git:
  enabled: true # 分析を有効にする (デフォルトは true)
  history_period: '1y'
```

## 拡張 (Extends)

`extends` フィールドを使用すると、異なるソースからプリセットを読み込むことができます：

- **組み込みプリセット**: `nestjs`、`nextjs`、`express`、`react`、`angular`、`vue`、`typeorm`、`prisma`、`oclif`、`class-validator`。
- **ローカルファイル**: YAMLファイルへの相対パス（例：`./archlint-shared.yaml`）。
- **URL**: YAMLファイルへの直接URL（例：`https://example.com/preset.yaml`）。

プリセットはリストされた順序でマージされます。ユーザー設定が常に最優先されます。

## ルールと重要度レベル (Rules and Severity Levels)

`rules` セクションでは、以下のレベルを使用できます。

- `critical`: 即時の対応が必要な重大な問題。
- `high`: 重要度の高いアーキテクチャ上の問題。
- `medium`: 中程度の重要度の問題または警告。
- `low`: 低い重要度の問題または情報メッセージ。
- `off`: 検出器を完全に無効にします。

## CLI による設定

CLI から設定ファイルのパスを明示的に指定することもできます。

```bash
archlint scan --config custom-config.yaml
```

## TypeScript との統合

archlint は `tsconfig.json` と自動的に同期できます。`tsconfig` フィールドを使用してこれを制御します：

- `tsconfig: true` (デフォルト): プロジェクトルートで `tsconfig.json` を自動的に検索します。
- `tsconfig: false` または `tsconfig: null`: TypeScript 統合を無効にします。
- `tsconfig: "./path/to/tsconfig.json"`: 特定の設定ファイルを使用します。

有効にすると、ツールは：

1. **エイリアスの読み込み**: `compilerOptions.paths` と `compilerOptions.baseUrl` を抽出し、`aliases` を自動的に設定します。
2. **自動無視**: `compilerOptions.outDir` をグローバルな `ignore` リストに追加します。
3. **除外設定**: `exclude` フィールドのパターンを `ignore` リストに組み込みます。

## Diff の設定

`diff` セクションは、2 つのスナップショットを比較する際のアーキテクチャの回帰（regression）の検出方法を制御します。

- **`metric_threshold_percent`** (デフォルト: `20`): メトリクス（循環的複雑度や結合度など）がどの程度増加したら「悪化した」と報告するかを定義します。例えば、20% のしきい値では、関数の複雑度が 10 から少なくとも 12 に増加した場合にフラグが立てられます。
- **`line_tolerance`** (デフォルト: `50`): ファイル内の他の場所での追加や削除により、コードシンボルが移動できる最大行数を定義します。この「ファジー・マッチング」により、移動したコードが新しい回帰として報告されるのを防ぎます。

ツールはプロジェクトルートで `tsconfig.json` を検索します。カスタム設定を使用している場合は、`tsconfig` フィールドを使用して正しいファイルを指定してください。
