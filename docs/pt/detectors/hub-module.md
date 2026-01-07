# Módulo Hub

**ID:** `hub_module` | **Severity:** Medium (default)

Um "Módulo Hub" (Hub Module) é um módulo que atua como um ponto central no gráfico de dependências, tendo alto Fan-in e alto Fan-out.

## Por que isso é um "smell"

Os módulos Hub são "pontos únicos de falha" perigosos em sua arquitetura. Como tantas coisas dependem deles, e eles dependem de tantas outras coisas, eles são extremamente frágeis e difíceis de mudar sem causar um efeito cascata em toda a base de código.

## Como corrigir

Quebre o hub! Identifique os diferentes caminhos de dados ou controle que passam pelo hub e extraia-os em módulos separados e mais focados.
