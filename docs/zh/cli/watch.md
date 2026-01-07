# archlint watch

`watch` 命令在后台运行 archlint，并在每次文件更改时重新分析您的项目。

## 用法

```bash
archlint watch [options]
```

## 选项

| 选项              | 默认值  | 描述                             |
| ----------------- | ------- | -------------------------------- |
| `--debounce <ms>` | `300`   | 在重新运行之前等待更多更改的时间 |
| `--clear`         | `false` | 每次运行前清空终端屏幕           |

## 示例

### 开发过程中的实时反馈

```bash
archlint watch --clear --debounce 500
```
