# Clusters de Dependência Cíclica

**ID:** `cycle_clusters` | **Severity:** Critical (default)

Um cluster de dependência cíclica é um conjunto de dependências circulares interconectadas, formando uma teia complexa de dependências. Ao contrário de ciclos simples (A -> B -> A), os clusters envolvem múltiplos ciclos que se sobrepõem (por exemplo, A -> B -> C -> A e B -> D -> C -> B).

## Por que isso é um problema

- **Degradação Arquitetural**: Clusters geralmente indicam a falta de limites claros entre múltiplos componentes.
- **Acoplamento Extremo**: Todo o cluster deve ser tratado como uma única unidade monolítica.
- **Isolamento Impossível**: É quase impossível alterar ou testar um módulo no cluster sem afetar todos os outros.
- **Pesadelo de Manutenção**: Alterações em qualquer parte do cluster podem ter efeitos imprevisíveis em todos os módulos envolvidos.

## Exemplos

### Ruim

Um grupo de módulos em um diretório "core" onde quase todos os módulos importam vários outros do mesmo diretório, criando múltiplos ciclos sobrepostos.

### Bom

Os módulos devem ser organizados em uma hierarquia ou com um desacoplamento claro baseado em interfaces para garantir que os ciclos não formem clusters.

## Configuração

```yaml
rules:
  cycle_clusters:
    severity: high
    max_cluster_size: 5
```

## Como corrigir

1. **Quebrar o hub**: Identifique os módulos "hub" que participam de múltiplos ciclos e desacople-os primeiro.
2. **Camadas**: Imponha regras rígidas de camadas para evitar dependências horizontais ou ascendentes.
3. **Refatorar Monólitos**: Muitas vezes, os clusters são um sinal de que um único módulo grande foi dividido incorretamente. Considere fundir ou dividir novamente ao longo de limites diferentes.
