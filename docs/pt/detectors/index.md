---
title: Visão Geral dos Detectores
description: Explore mais de 28 detectores de code smells de arquitetura no `archlint`, incluindo dependências cíclicas, violações de camadas, módulos deus e muito mais.
---

# Visão Geral dos Detectores

O `archlint` vem com mais de 28 detectores integrados categorizados pelo tipo de problema arquitetural ou de qualidade de código que identificam.

## Problemas de Dependência

| Detector                                             | ID                   | Descrição                                       | Padrão |
| ---------------------------------------------------- | -------------------- | ----------------------------------------------- | ------ |
| [Dependências Cíclicas](/pt/detectors/cycles)        | `cycles`             | Dependências circulares entre arquivos          | ✅     |
| [Ciclos de Tipos](/pt/detectors/circular-type-deps)  | `circular_type_deps` | Dependências circulares apenas de tipos         | ❌     |
| [Ciclos de Pacotes](/pt/detectors/package-cycle)     | `package_cycles`     | Dependências cíclicas entre pacotes             | ❌     |
| [Violação de Camadas](/pt/detectors/layer-violation) | `layer_violation`    | Violações de camadas arquiteturais definidas    | ❌     |
| [Violação de SDP](/pt/detectors/sdp-violation)       | `sdp_violation`      | Violações do Princípio de Dependências Estáveis | ❌     |

## Design de Módulo e Classe

| Detector                                          | ID                | Descrição                                           | Padrão |
| ------------------------------------------------- | ----------------- | --------------------------------------------------- | ------ |
| [Módulo Deus](/pt/detectors/god-module)           | `god_module`      | Módulos com muitas responsabilidades                | ✅     |
| [Módulo Hub](/pt/detectors/hub-module)            | `hub_module`      | Módulos "hub" altamente conectados                  | ❌     |
| [Baixa Coesão](/pt/detectors/lcom)                | `lcom`            | Classes com baixa coesão interna (LCOM4)            | ❌     |
| [Alto Acoplamento](/pt/detectors/high-coupling)   | `high_coupling`   | Módulos com muitas dependências                     | ❌     |
| [Módulo Disperso](/pt/detectors/scattered-module) | `module_cohesion` | Funcionalidade dispersa em muitos arquivos          | ❌     |
| [Inveja de Recursos](/pt/detectors/feature-envy)  | `feature_envy`    | Métodos que usam mais outra classe do que a própria | ❌     |

## Qualidade do Código e Organização

| Detector                                                     | ID                    | Descrição                                              | Padrão |
| ------------------------------------------------------------ | --------------------- | ------------------------------------------------------ | ------ |
| [Código Morto](/pt/detectors/dead-code)                      | `dead_code`           | Exports não utilizados                                 | ✅     |
| [Símbolos Mortos](/pt/detectors/dead-symbols)                | `dead_symbols`        | Funções e variáveis locais não utilizadas              | ✅     |
| [Tipos Órfãos](/pt/detectors/orphan-types)                   | `orphan_types`        | Tipos não conectados à base de código                  | ✅     |
| [Abuso de Barrel](/pt/detectors/barrel-abuse)                | `barrel_file`         | Arquivos barrel grandes causando acoplamento           | ✅     |
| [Obsessão por Primitivos](/pt/detectors/primitive-obsession) | `primitive_obsession` | Uso excessivo de primitivos em vez de tipos de domínio | ❌     |

## Complexidade e Tamanho

| Detector                                           | ID             | Descrição                                 | Padrão |
| -------------------------------------------------- | -------------- | ----------------------------------------- | ------ |
| [Alta Complexidade](/pt/detectors/complexity)      | `complexity`   | Funções com alta complexidade ciclomática | ✅     |
| [Aninhamento Profundo](/pt/detectors/deep-nesting) | `deep_nesting` | Blocos de código profundamente aninhados  | ✅     |
| [Muitos Parâmetros](/pt/detectors/long-params)     | `long_params`  | Funções com muitos parâmetros             | ✅     |
| [Arquivo Grande](/pt/detectors/large-file)         | `large_file`   | Arquivos fonte que são muito grandes      | ✅     |

## Padrões de Mudança

| Detector                                                 | ID                   | Descrição                                          | Padrão |
| -------------------------------------------------------- | -------------------- | -------------------------------------------------- | ------ |
| [Cirurgia por Perdigotos](/pt/detectors/shotgun-surgery) | `shotgun_surgery`    | Mudanças que exigem modificação em muitos arquivos | ❌     |
| [Interface Instável](/pt/detectors/unstable-interface)   | `unstable_interface` | Interfaces públicas que mudam frequentemente       | ❌     |

## Execução e Segurança

| Detector                                                            | ID                   | Descrição                                   | Padrão |
| ------------------------------------------------------------------- | -------------------- | ------------------------------------------- | ------ |
| [Vazamento de Testes](/pt/detectors/test-leakage)                   | `test_leakage`       | Código de teste vazando para produção       | ❌     |
| [Acoplamento com Fornecedor](/pt/detectors/vendor-coupling)         | `vendor_coupling`    | Acoplamento forte com bibliotecas externas  | ❌     |
| [Importação com Efeito Colateral](/pt/detectors/side-effect-import) | `side_effect_import` | Importações que disparam efeitos colaterais | ✅     |
| [Estado Mutável Compartilhado](/pt/detectors/shared-mutable-state)  | `shared_state`       | Variáveis mutáveis exportadas               | ❌     |

## Métricas Arquiteturais

| Detector                                                 | ID                 | Descrição                                  | Padrão |
| -------------------------------------------------------- | ------------------ | ------------------------------------------ | ------ |
| [Violação de Abstratividade](/pt/detectors/abstractness) | `abstractness`     | Zonas de Dor/Inutilidade (métrica I+A)     | ❌     |
| [Configuração Dispersa](/pt/detectors/scattered-config)  | `scattered_config` | Configuração espalhada por muitos arquivos | ❌     |
| [Clone de Código](/pt/detectors/code-clone)              | `code_clone`       | Código duplicado no projeto                | ✅     |
