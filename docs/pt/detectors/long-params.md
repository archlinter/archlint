# Lista de Parâmetros Longa

**ID:** `long_params` | **Severity:** Low (default)

Identifica funções ou métodos que possuem parâmetros demais.

## Por que isso é um "smell"

Funções com muitos parâmetros são difíceis de usar e de ler. Elas geralmente indicam que a função está fazendo demais ou que alguns parâmetros deveriam ser agrupados em um objeto.

## Como consertar

- **Introduce Parameter Object**: Agrupe parâmetros relacionados em um único objeto ou interface.
- **Decompose Function**: Divida a função em funções menores que exijam menos parâmetros.

## Configuração

```yaml
thresholds:
  long_params:
    max_params: 5
```
