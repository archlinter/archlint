# archlint snapshot

`snapshot`コマンドは、プロジェクトのアーキテクチャの現在の状態をキャプチャし、JSONファイルに保存します。このファイルは、後で`diff`コマンドで使用できます。

## 使用法

```bash
archlint snapshot [options]
```

## オプション

| オプション            | デフォルト               | 説明                               |
| --------------------- | ------------------------ | ---------------------------------- |
| `--output, -o <file>` | `archlint-snapshot.json` | スナップショットを保存するファイル |

## 使用例

### プロジェクトのベースラインを作成する

```bash
archlint snapshot -o .archlint-baseline.json
```
