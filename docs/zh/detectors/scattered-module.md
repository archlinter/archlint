# 分散模块

**ID:** `module_cohesion` | **严重程度:** 中 (默认)

识别一个"模块"（通常是一个文件或逻辑分组），其中的内部元素（函数、类）之间没有很好的连接。这表明该模块缺乏内聚的目的，很可能只是不相关代码的集合。

## 为什么这是一种坏味道

一个模块应该是高内聚的，遵循"一起变化的东西应该放在一起"的原则。如果一个模块的内部部分互不影响，它就不是一个真正的模块——它只是一个用作随机存储的文件夹或文件。这使得代码更难找到，并增加了认知负担。

## 示例

### 坏习惯

一个包含不相关辅助函数的文件，这些函数没有共同的逻辑或数据。

```typescript
// misc-utils.ts
export const formatCurrency = (val: number) => { ... };
export const validateEmail = (email: string) => { ... };
export const parseJwt = (token: string) => { ... };
// 这三个函数没有共同的状态或逻辑。
```

### 好习惯

将不相关的函数分组到特定的、内聚的模块中。

```typescript
// currency-utils.ts
export const formatCurrency = (val: number) => { ... };

// validation-utils.ts
export const validateEmail = (email: string) => { ... };
```

## 配置

```yaml
rules:
  module_cohesion:
    severity: warn
    min_exports: 5
    max_components: 2
```

## 如何修复

重新评估模块的用途。将代码分组到更具内聚性的模块中，或者将不相关的部分移动到它们实际被使用的地方。
