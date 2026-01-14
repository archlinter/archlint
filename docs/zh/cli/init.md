# archlint init

`init` 命令通过生成配置文件，帮助您在现有项目中快速设置 archlint。

## 用法

```bash
archlint init [options]
```

## 选项

| 选项               | 默认值  | 描述                                  |
| ------------------ | ------- | ------------------------------------- |
| `-f, --force`      | `false` | 如果已存在 `.archlint.yaml`，则覆盖它 |
| `--no-interactive` | `false` | 跳过交互式框架选择                    |
| `--presets <list>` | `none`  | 显式指定框架预设（逗号分隔）          |

## 工作原理

1. **框架检测**：archlint 分析您的 `package.json` 依赖项和项目结构以检测使用的框架。
2. **交互式选择**：除非使用 `--no-interactive`，否则它会提示您确认或选择其他框架预设。
3. **配置生成**：创建一个包含所选预设的 `.archlint.yaml` 文件，并包含用于 IDE 支持的 JSON 架构引用。

## 示例

### 交互式初始化

```bash
archlint init
```

### 使用特定预设的非交互式初始化

```bash
archlint init --no-interactive --presets nestjs,prisma
```

### 覆盖现有配置

```bash
archlint init --force
```
