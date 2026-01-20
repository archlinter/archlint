# 中心模块

**ID:** `hub_module` | **严重程度:** 中 (默认)

"中心模块"就像一个没有红绿灯的繁忙交叉路口。它是一个所有人都依赖的模块，它也依赖于所有其他模块，但它本身实际上并不*做*多少逻辑。

## 为什么这是一种坏味道

中心模块是危险的单点故障。因为它们位于一切的中心，所以它们极其脆弱。对一个中心的微小改动可能会破坏你应用中不相关的部分，使其成为整个代码库中最可怕的重构文件。它是架构的终极"瓶颈"。

## 示例

### 坏习惯

一个仅仅重新导出或协调许多不相关服务的模块，而且它本身被整个应用程序使用。

```typescript
// app-hub.ts
import { AuthService } from './auth';
import { ApiService } from './api';
import { LoggerService } from './logger';
import { ConfigService } from './config';
// ... 还有 10 多个导入

export class AppHub {
  constructor(
    public auth: AuthService,
    public api: ApiService,
    public logger: LoggerService
    // ... 还有 10 多个依赖
  ) {}
}
```

### 好习惯

将中心分解为特定的、专注的协调器，或在消费者层面使用依赖注入来避免中心化。

```typescript
// auth-coordinator.ts (专注于认证相关的协调)
import { AuthService } from './auth';
import { SessionStore } from './session';

export class AuthCoordinator {
  constructor(
    private auth: AuthService,
    private session: SessionStore
  ) {}
}
```

## 配置

```yaml
rules:
  hub_module:
    severity: medium
    min_fan_in: 5
    min_fan_out: 5
    max_complexity: 5
```

## ESLint 规则

此检测器可作为 ESLint 规则使用，以便在编辑器中获得实时反馈。

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-hub-modules': 'warn',
    },
  },
];
```

详见 [ESLint 集成](/zh/integrations/eslint) 了解设置说明。

## 如何修复

打破中心！识别通过该中心的不同数据或控制路径，并将它们提取到独立的、更集中的模块中。
