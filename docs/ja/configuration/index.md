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
aliases:
  '@/*': 'src/*'

# 解析のエントリポイント (デッドコード検出に使用)
entry_points:
  - 'src/main.ts'

# 各検出器のルール設定
rules:
  # 短縮形式: 重要度レベルまたは "off"
  cycles: error
  dead_code: warn

  # 完全形式: 追加オプション付き
  god_module:
    severity: error
    enabled: true
    exclude: ['**/generated/**']
    # 検出器固有のオプション
    fan_in: 15
    fan_out: 15
    churn: 20

# 特定のパスに対するルールのオーバーライド
overrides:
  - files: ['**/legacy/**']
    rules:
      complexity: warn
      god_module: off

# スコアリングとグレーディングの設定
scoring:
  # 報告する最小重要度レベル (info, warn, error, critical)
  minimum: warn
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

# フレームワークの使用
framework: nestjs

# フレームワークの自動検出 (デフォルトは true)
auto_detect_framework: true

# Git 履歴分析の有効化 (デフォルトは true)
enable_git: true

# Git 設定
git:
  history_period: '1y'
```

## 重要度レベル (Severity Levels)

`rules` セクションでは、以下のレベルを使用できます。

- `critical`: 即時の対応が必要な重大な問題。
- `error`: アーキテクチャ上のエラー。
- `warn`: 潜在的な問題に関する警告。
- `info`: 情報メッセージ。
- `off`: 検出器を完全に無効にします。

## CLI による設定

CLI から設定ファイルのパスを明示的に指定することもできます。

```bash
archlint scan --config custom-config.yaml
```
