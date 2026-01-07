# archlint snapshot

`snapshot` 命令捕获项目架构的当前状态并将其保存到 JSON 文件中。此文件随后可与 `diff` 命令配合使用。

## 用法

```bash
archlint snapshot [options]
```

## 选项

| 选项                  | 默认值                   | 描述             |
| --------------------- | ------------------------ | ---------------- |
| `--output, -o <file>` | `archlint-snapshot.json` | 保存快照的文件名 |

## 示例

### 为项目创建基线

```bash
archlint snapshot -o .archlint-baseline.json
```
