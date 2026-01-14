---
title: Visão Geral dos Detectores
description: Explore mais de 28 detectores de code smells de arquitetura no `archlint`, incluindo dependências cíclicas, violações de camadas, módulos deus e muito mais.
---

# Visão Geral dos Detectores

O `archlint` vem com mais de 28 detectores integrados categorizados pelo tipo de problema arquitetural ou de qualidade de código que identificam.

::: tip
**Falsos Positivos**: A análise arquitetural pode, às vezes, produzir falsos positivos, especialmente em projetos com carregamento dinâmico pesado, reflexão ou contêineres complexos de Injeção de Dependência (DI).
:::

## Problemas de Dependência

| Detector                                                 | ID                   | Descrição                                       | Padrão |
| -------------------------------------------------------- | -------------------- | ----------------------------------------------- | ------ |
| [Dependências Cíclicas](/pt/detectors/cyclic_dependency) | `cyclic_dependency`  | Dependências circulares entre arquivos          | ✅     |
| [Clusters de Ciclos](/pt/detectors/cycle_clusters)       | `cycle_clusters`     | Teia complexa de dependências circulares        | ✅     |
| [Ciclos de Tipos](/pt/detectors/circular_type_deps)      | `circular_type_deps` | Dependências circulares apenas de tipos         | ❌     |
| [Ciclos de Pacotes](/pt/detectors/package_cycles)        | `package_cycles`     | Dependências cíclicas entre pacotes             | ❌     |
| [Violação de Camadas](/pt/detectors/layer_violation)     | `layer_violation`    | Violações de camadas arquiteturais definidas    | ❌     |
| [Violação de SDP](/pt/detectors/sdp_violation)           | `sdp_violation`      | Violações do Princípio de Dependências Estáveis | ❌     |

## Design de Módulo e Classe

| Detector                                         | ID                | Descrição                                           | Padrão |
| ------------------------------------------------ | ----------------- | --------------------------------------------------- | ------ |
| [Módulo Deus](/pt/detectors/god_module)          | `god_module`      | Módulos com muitas responsabilidades                | ✅     |
| [Módulo Hub](/pt/detectors/hub_module)           | `hub_module`      | Módulos "hub" altamente conectados                  | ❌     |
| [Baixa Coesão](/pt/detectors/lcom)               | `lcom`            | Classes com baixa coesão interna (LCOM4)            | ❌     |
| [Alto Acoplamento](/pt/detectors/high_coupling)  | `high_coupling`   | Módulos com muitas dependências                     | ❌     |
| [Módulo Disperso](/pt/detectors/module_cohesion) | `module_cohesion` | Funcionalidade dispersa em muitos arquivos          | ❌     |
| [Inveja de Recursos](/pt/detectors/feature_envy) | `feature_envy`    | Métodos que usam mais outra classe do que a própria | ❌     |

## Qualidade do Código e Organização

| Detector                                                     | ID                    | Descrição                                              | Padrão |
| ------------------------------------------------------------ | --------------------- | ------------------------------------------------------ | ------ |
| [Código Morto](/pt/detectors/dead_code)                      | `dead_code`           | Exports não utilizados                                 | ✅     |
| [Símbolos Mortos](/pt/detectors/dead_symbols)                | `dead_symbols`        | Funções e variáveis locais não utilizadas              | ✅     |
| [Tipos Órfãos](/pt/detectors/orphan_types)                   | `orphan_types`        | Tipos não conectados à base de código                  | ✅     |
| [Abuso de Barrel](/pt/detectors/barrel_file)                 | `barrel_file`         | Arquivos barrel grandes causando acoplamento           | ✅     |
| [Obsessão por Primitivos](/pt/detectors/primitive_obsession) | `primitive_obsession` | Uso excessivo de primitivos em vez de tipos de domínio | ❌     |

## Complexidade e Tamanho

| Detector                                           | ID             | Descrição                                 | Padrão |
| -------------------------------------------------- | -------------- | ----------------------------------------- | ------ |
| [Alta Complexidade](/pt/detectors/complexity)      | `complexity`   | Funções com alta complexidade ciclomática | ✅     |
| [Aninhamento Profundo](/pt/detectors/deep_nesting) | `deep_nesting` | Blocos de código profundamente aninhados  | ✅     |
| [Muitos Parâmetros](/pt/detectors/long_params)     | `long_params`  | Funções com muitos parâmetros             | ✅     |
| [Arquivo Grande](/pt/detectors/large_file)         | `large_file`   | Arquivos fonte que são muito grandes      | ✅     |

## Padrões de Mudança

| Detector                                                 | ID                   | Descrição                                          | Padrão |
| -------------------------------------------------------- | -------------------- | -------------------------------------------------- | ------ |
| [Cirurgia por Perdigotos](/pt/detectors/shotgun_surgery) | `shotgun_surgery`    | Mudanças que exigem modificação em muitos arquivos | ❌     |
| [Interface Instável](/pt/detectors/unstable_interface)   | `unstable_interface` | Interfaces públicas que mudam frequentemente       | ❌     |

## Execução e Segurança

| Detector                                                            | ID                     | Descrição                                   | Padrão |
| ------------------------------------------------------------------- | ---------------------- | ------------------------------------------- | ------ |
| [Vazamento de Testes](/pt/detectors/test_leakage)                   | `test_leakage`         | Código de teste vazando para produção       | ❌     |
| [Acoplamento com Fornecedor](/pt/detectors/vendor_coupling)         | `vendor_coupling`      | Acoplamento forte com bibliotecas externas  | ❌     |
| [Dependência Hub](/pt/detectors/hub_dependency)                     | `hub_dependency`       | Dependência excessiva de pacotes externos   | ❌     |
| [Importação com Efeito Colateral](/pt/detectors/side_effect_import) | `side_effect_import`   | Importações que disparam efeitos colaterais | ✅     |
| [Estado Mutável Compartilhado](/pt/detectors/shared_mutable_state)  | `shared_mutable_state` | Variáveis mutáveis exportadas               | ❌     |

## Métricas Arquiteturais

| Detector                                                 | ID                 | Descrição                                  | Padrão |
| -------------------------------------------------------- | ------------------ | ------------------------------------------ | ------ |
| [Violação de Abstratividade](/pt/detectors/abstractness) | `abstractness`     | Zonas de Dor/Inutilidade (métrica I+A)     | ❌     |
| [Configuração Dispersa](/pt/detectors/scattered_config)  | `scattered_config` | Configuração espalhada por muitos arquivos | ❌     |
| [Clone de Código](/pt/detectors/code_clone)              | `code_clone`       | Código duplicado no projeto                | ✅     |
