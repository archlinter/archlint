# Módulo Deus

**ID:** `god_module` | **Severity:** High (default)

Um "Módulo Deus" (God Module) é um arquivo que cresceu demais e assumiu muitas responsabilidades.

## Por que isso é um "smell"

- **Viola o Princípio da Responsabilidade Única**: O módulo faz coisas demais.
- **Conflitos de Merge**: Mudanças frequentes por diferentes desenvolvedores levam a conflitos constantes.
- **Fragilidade**: Mudanças em uma parte do módulo podem quebrar inesperadamente outra parte.
- **Difícil de Testar**: Requer uma configuração complexa para testar várias funcionalidades não relacionadas.

## Critérios de Detecção

O archlint identifica Módulos Deus com base em:

- **Fan-in**: Número de outros módulos que dependem dele.
- **Fan-out**: Número de módulos dos quais ele depende.
- **Churn**: Frequência de mudanças no git.
- **Lines of Code**: Tamanho total do arquivo.

## Como corrigir

1. **Identificar Responsabilidades**: Liste todas as diferentes tarefas que o módulo executa.
2. **Extrair Módulos**: Divida o arquivo em módulos menores e mais focados.
3. **Padrão Facade**: Se o módulo atua como um coordenador, mantenha apenas a lógica de coordenação e delegue o trabalho para submódulos.

## Configuração

```yaml
thresholds:
  god_module:
    fan_in: 15
    fan_out: 15
    churn: 20
    max_lines: 500
```
