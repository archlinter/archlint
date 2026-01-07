# しきい値（Thresholds）

しきい値を使用すると、検出器が不吉なにおい（smell）を報告するタイミングを微調整できます。

## 一般的なしきい値

| 検出器         | オプション         | デフォルト | 説明                                             |
| -------------- | ------------------ | ---------- | ------------------------------------------------ |
| `cycles`       | `exclude_patterns` | `[]`       | サイクル検出で無視するグロブパターン             |
| `god_module`   | `fan_in`           | `10`       | 最大流入依存関係数                               |
| `god_module`   | `fan_out`          | `10`       | 最大流出依存関係数                               |
| `god_module`   | `churn`            | `20`       | 履歴における最大のgitコミット数                  |
| `god_module`   | `max_lines`        | `500`      | ファイルの最大行数                               |
| `complexity`   | `max_complexity`   | `15`       | 関数ごとの最大循環複雑度                         |
| `deep_nesting` | `max_depth`        | `4`        | ブロックの最大ネスト深度                         |
| `long_params`  | `max_params`       | `5`        | 関数ごとの最大パラメータ数                       |
| `large_file`   | `max_lines`        | `1000`     | ファイルごとの最大行数                           |
| `lcom`         | `threshold`        | `1`        | クラス内で許容される非接続コンポーネントの最大数 |

## 設定例

```yaml
thresholds:
  god_module:
    fan_in: 20
    max_lines: 800

  complexity:
    max_complexity: 10

  large_file:
    max_lines: 2000
```
