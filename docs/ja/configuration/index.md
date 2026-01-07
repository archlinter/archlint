---
title: 設定
description: archlint.yamlを使用してarchlintを構成し、アーキテクチャレイヤーを定義し、検出器のカスタムしきい値を設定する方法を学びます。
---

# 設定

archlintは、プロジェクトルートにある`archlint.yaml`ファイルを使用して設定できます。設定ファイルが見つからない場合、ツールはすべての検出器に対して適切なデフォルト値を使用します。

## 設定ファイルの構造

```yaml
# 無視するファイル
ignore:
  - '**/dist/**'

# パスエイリアス（例：tsconfig.jsonから）
aliases:
  '@/*': 'src/*'

# デッドコード分析のエントリポイント
entry_points:
  - 'src/index.ts'

# 検出器のカスタムしきい値
thresholds:
  cycles:
    exclude_patterns: []
  god_module:
    fan_in: 15
    fan_out: 15

# アーキテクチャレイヤー
layers:
  - name: domain
    paths: ['**/domain/**']
    can_import: []

# フレームワークのプリセット
frameworks:
  - nestjs

# 重要度の上書き（Severity overrides）
severity:
  cycles: critical
```

## CLIでの設定

CLI経由で設定ファイルのパスを指定することもできます。

```bash
archlint scan --config custom-config.yaml
```
