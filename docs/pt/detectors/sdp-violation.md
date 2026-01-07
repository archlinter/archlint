# Princípio de Dependências Estáveis (SDP)

**ID:** `sdp_violation` | **Severity:** Medium (default)

O Princípio de Dependências Estáveis afirma que "as dependências entre pacotes devem estar na direção da estabilidade". Em outras palavras, módulos estáveis (difíceis de mudar) não devem depender de módulos instáveis (fáceis de mudar).

## Por que isso é um smell

Se um módulo estável (aquele do qual muitos outros dependem) depende de um módulo instável, o módulo estável torna-se mais difícil de mudar porque qualquer alteração no módulo instável pode afetá-lo, o que, por sua vez, afeta todos os seus dependentes.

## Como corrigir

Certifique-se de que seus módulos principais e estáveis não dependam de módulos voláteis. Use interfaces ou classes abstratas para desacoplá-los.
