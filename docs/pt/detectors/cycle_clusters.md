# Clusters de Dependência Cíclica

**ID:** `cycle_clusters` | **Severity:** Critical (default)

Um cluster de dependência cíclica é o que acontece quando dependências circulares começam a se reproduzir. Não é apenas um loop simples "A depende de B, B depende de A"—é uma rede complexa onde uma dúzia de módulos estão todos emaranhados.

## Por que isso é um problema

- **Deterioração Arquitetural**: É sinal de que seus limites de módulos entraram em colapso completo.
- **O efeito "Monolito"**: Você não pode simplesmente pegar um módulo do cluster; você tem que arrastar toda a bagunça emaranhada. É um pacote completo que você não pediu.
- **Isolamento Impossível**: Quer testar uma única função? Sem sorte, você está agora mockando metade do seu código porque tudo está interconectado.
- **Pesadelo de Manutenção**: Mudar um módulo no cluster pode disparar um efeito borboleta imprevisível que quebra algo do outro lado da rede.

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
