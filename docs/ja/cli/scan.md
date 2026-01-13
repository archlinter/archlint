# archlint scan

`scan`コマンドは、プロジェクトの完全なアーキテクチャ分析を実行します。

## 使用法

```bash
archlint scan [path] [options]
```

## オプション

| オプション                      | デフォルト | 説明                                                        |
| ------------------------------- | ---------- | ----------------------------------------------------------- |
| `-f, --format <format>`         | `table`    | 出力形式: `table`, `json`, `markdown`                       |
| `-j, --json`                    | `false`    | `--format json` のショートカット                            |
| `-r, --report <file>`           | `stdout`   | レポートをファイルに保存します                              |
| `-s, --min-severity <sev>`      | `low`      | 重要度でフィルタリング: `low`, `medium`, `high`, `critical` |
| `-S, --min-score <score>`       | `none`     | 最小ヘルススコアでフィルタリング                            |
| `-d, --detectors <ids>`         | `all`      | 実行する検出器（detectors）のカンマ区切りリスト             |
| `-e, --exclude-detectors <ids>` | `none`     | スキップする検出器                                          |
| `-A, --all`                     | `false`    | すべての検出器を実行（デフォルトで無効なものを含む）        |
| `--no-cache`                    | `false`    | 分析キャッシュを無効にします                                |
| `--no-git`                      | `false`    | git 統合を無効にする (churn 分析をスキップ)                 |

## 使用例

### Markdownレポートを使用したスキャン

```bash
archlint scan --format markdown --report report.md
```

### サイクル検出のみを実行

```bash
archlint scan --detectors cycles,circular_type_deps
```

### 高重要度（high severity）のみ

```bash
archlint scan --min-severity high
```
