# archlint scan

`scan`コマンドは、プロジェクトの完全なアーキテクチャ分析を実行します。

## 使用法

```bash
archlint scan [path] [options]
```

## オプション

| オプション                  | デフォルト | 説明                                                        |
| --------------------------- | ---------- | ----------------------------------------------------------- |
| `--format <format>`         | `table`    | 出力形式: `table`, `json`, `markdown`                       |
| `--report <file>`           | `stdout`   | レポートをファイルに保存します                              |
| `--min-severity <sev>`      | `low`      | 重要度でフィルタリング: `low`, `medium`, `high`, `critical` |
| `--detectors <ids>`         | `all`      | 実行する検出器（detectors）のカンマ区切りリスト             |
| `--exclude-detectors <ids>` | `none`     | スキップする検出器                                          |
| `--no-cache`                | `false`    | 分析キャッシュを無効にします                                |

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
