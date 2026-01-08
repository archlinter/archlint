# 稳定依赖原则 (SDP)

**ID:** `sdp_violation` | **严重程度:** 中 (默认)

稳定依赖原则指出："包之间的依赖关系应该朝向稳定的方向。"换句话说，稳定的（难以改变的）模块不应该依赖于不稳定的（易于改变的）模块。

在这个上下文中，稳定性是通过有多少其他模块依赖于一个模块（Fan-in）与它依赖多少模块（Fan-out）来衡量的。

## 为什么这是一种坏味道

当一个稳定模块——许多其他组件依赖的模块——依赖于一个不稳定模块时，它变得难以更改。不稳定依赖中的任何修改都可能破坏稳定模块，然后影响到它的所有依赖者。这实际上"冻结"了不稳定模块，或使整个系统变得脆弱。

## 示例

### 坏习惯

一个核心领域实体（稳定的）依赖于特定的数据库实现或频繁更改的 UI 组件（不稳定的）。

```typescript
// domain/user.ts (稳定的，很多东西依赖 User)
import { UserPreferencesUI } from '../ui/user-prefs'; // 不稳定的依赖

export class User {
  updateSettings(prefs: UserPreferencesUI) {
    // ...
  }
}
```

### 好习惯

稳定模块依赖于一个很少变化的抽象（如接口）。

```typescript
// domain/user.ts
export interface UserSettings {
  theme: string;
  notifications: boolean;
}

export class User {
  updateSettings(settings: UserSettings) {
    // ...
  }
}
```

## 配置

```yaml
rules:
  sdp_violation:
    severity: warn
    min_fan_total: 5
    instability_diff: 0.3
```

## ESLint 规则

此检测器可作为 ESLint 规则使用，以便在编辑器中获得实时反馈。

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-sdp-violations': 'warn',
    },
  },
];
```

详见 [ESLint 集成](/zh/integrations/eslint) 了解设置说明。

## 如何修复

确保您的核心、稳定模块不依赖于易变模块。使用接口或抽象类来解耦它们。
