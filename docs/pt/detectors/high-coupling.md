# Alto Acoplamento

**ID:** `high_coupling` | **Severity:** Medium (default)

O alto acoplamento ocorre quando um módulo depende de muitos outros módulos (alto Fan-out).

## Por que isso é um "smell"

- **Rigidez**: Uma mudança em qualquer uma das dependências pode exigir uma mudança neste módulo.
- **Fragilidade**: O módulo tem maior probabilidade de quebrar quando qualquer uma de suas dependências muda.
- **Difícil de Testar**: Requer muitos mocks para isolar para testes de unidade.

## Como corrigir

1. **Extrair Responsabilidades**: Se um módulo tem muitas dependências, provavelmente está fazendo demais.
2. **Usar Abstrações**: Dependa de uma interface ou de uma fachada em vez de muitas implementações concretas.

## Configuração

```yaml
thresholds:
  high_coupling:
    max_dependencies: 15
```
