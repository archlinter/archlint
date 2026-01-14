import { defineConfig, type DefaultTheme } from 'vitepress'
import { groupIconMdPlugin, groupIconVitePlugin } from 'vitepress-plugin-group-icons'

const enNav: DefaultTheme.NavItem[] = [
  { text: 'Getting Started', link: '/getting-started/' },
  { text: 'Detectors', link: '/detectors/' },
  { text: 'Configuration', link: '/configuration/' },
  { text: 'CLI', link: '/cli/' },
  { text: 'Integrations', link: '/integrations/eslint' },
]

const enSidebar: DefaultTheme.Sidebar = {
  '/getting-started/': [
    {
      text: 'Getting Started',
      items: [
        { text: 'Introduction', link: '/getting-started/' },
        { text: 'Installation', link: '/getting-started/installation' },
        { text: 'First Scan', link: '/getting-started/first-scan' },
      ]
    }
  ],
  '/detectors/': [
    {
      text: 'Introduction',
      items: [
        { text: 'Overview', link: '/detectors/' },
      ]
    },
    {
      text: 'Dependency Issues',
      items: [
        { text: 'Cyclic Dependencies', link: '/detectors/cycles' },
        { text: 'Type Cycles', link: '/detectors/circular-type-deps' },
        { text: 'Package Cycles', link: '/detectors/package-cycle' },
        { text: 'Layer Violation', link: '/detectors/layer-violation' },
        { text: 'SDP Violation', link: '/detectors/sdp-violation' },
      ]
    },
    {
      text: 'Module Design',
      items: [
        { text: 'God Module', link: '/detectors/god-module' },
        { text: 'Hub Module', link: '/detectors/hub-module' },
        { text: 'High Coupling', link: '/detectors/high-coupling' },
        { text: 'Scattered Module', link: '/detectors/scattered-module' },
        { text: 'Feature Envy', link: '/detectors/feature-envy' },
      ]
    },
    {
      text: 'Code Quality',
      items: [
        { text: 'Dead Code', link: '/detectors/dead-code' },
        { text: 'Dead Symbols', link: '/detectors/dead-symbols' },
        { text: 'Orphan Types', link: '/detectors/orphan-types' },
        { text: 'Barrel Abuse', link: '/detectors/barrel-abuse' },
        { text: 'Primitive Obsession', link: '/detectors/primitive-obsession' },
      ]
    },
    {
      text: 'Complexity & Size',
      items: [
        { text: 'High Complexity', link: '/detectors/complexity' },
        { text: 'Deep Nesting', link: '/detectors/deep-nesting' },
        { text: 'Long Parameters', link: '/detectors/long-params' },
        { text: 'Large File', link: '/detectors/large-file' },
      ]
    },
    {
      text: 'Change Patterns',
      items: [
        { text: 'Shotgun Surgery', link: '/detectors/shotgun-surgery' },
        { text: 'Unstable Interface', link: '/detectors/unstable-interface' },
      ]
    },
    {
      text: 'Runtime & Safety',
      items: [
        { text: 'Test Leakage', link: '/detectors/test-leakage' },
        { text: 'Vendor Coupling', link: '/detectors/vendor-coupling' },
        { text: 'Side Effect Import', link: '/detectors/side-effect-import' },
        { text: 'Shared Mutable State', link: '/detectors/shared-mutable-state' },
      ]
    },
    {
      text: 'Metrics',
      items: [
        { text: 'Abstractness Violation', link: '/detectors/abstractness' },
        { text: 'Scattered Config', link: '/detectors/scattered-config' },
      ]
    }
  ],
  '/cli/': [
    {
      text: 'CLI Reference',
      items: [
        { text: 'Overview', link: '/cli/' },
        { text: 'init', link: '/cli/init' },
        { text: 'scan', link: '/cli/scan' },
        { text: 'diff', link: '/cli/diff' },
        { text: 'snapshot', link: '/cli/snapshot' },
        { text: 'watch', link: '/cli/watch' },
      ]
    }
  ],
  '/configuration/': [
    {
      text: 'Configuration',
      items: [
        { text: 'Overview', link: '/configuration/' },
        { text: 'Layers', link: '/configuration/layers' },
        { text: 'Ignore', link: '/configuration/ignore' },
      ]
    },
    {
      text: 'Frameworks',
      items: [
        { text: 'Overview', link: '/frameworks/' },
        { text: 'NestJS', link: '/frameworks/nestjs' },
        { text: 'Next.js', link: '/frameworks/nextjs' },
        { text: 'React', link: '/frameworks/react' },
      ]
    }
  ],
  '/frameworks/': [
    {
      text: 'Configuration',
      items: [
        { text: 'Overview', link: '/configuration/' },
        { text: 'Layers', link: '/configuration/layers' },
        { text: 'Ignore', link: '/configuration/ignore' },
      ]
    },
    {
      text: 'Frameworks',
      items: [
        { text: 'Overview', link: '/frameworks/' },
        { text: 'NestJS', link: '/frameworks/nestjs' },
        { text: 'Next.js', link: '/frameworks/nextjs' },
        { text: 'React', link: '/frameworks/react' },
      ]
    }
  ],
  '/integrations/': [
    {
      text: 'Integrations',
      items: [
        { text: 'ESLint', link: '/integrations/eslint' },
        { text: 'MCP Server', link: '/integrations/mcp-server' },
        { text: 'GitHub Actions', link: '/integrations/github-actions' },
        { text: 'GitLab CI', link: '/integrations/gitlab-ci' },
      ]
    }
  ],
}

export default defineConfig({
  base: '/archlint/',
  title: "archlint",
  description: "Stop architecture degradation",
  lastUpdated: true,
  sitemap: {
    hostname: 'https://archlinter.github.io/archlint/'
  },
  head: [
    ['link', { rel: 'icon', type: 'image/svg+xml', href: '/archlint/logo.svg' }],
    ['link', { rel: 'icon', type: 'image/png', href: '/archlint/logo.png' }],
    ['link', { rel: 'icon', href: '/archlint/favicon.ico' }],
    ['meta', { name: 'theme-color', content: '#6366F1' }],
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { property: 'og:title', content: 'archlint | Stop architecture degradation' }],
    ['meta', { property: 'og:site_name', content: 'archlint' }],
    ['meta', { property: 'og:image', content: 'https://archlinter.github.io/archlint/logo.png' }],
    ['meta', { property: 'og:url', content: 'https://archlinter.github.io/archlint/' }],
    ['meta', { name: 'twitter:card', content: 'summary_large_image' }],
    ['meta', { name: 'twitter:title', content: 'archlint | Stop architecture degradation' }],
    ['meta', { name: 'twitter:description', content: 'Fast, AST-based architecture smell detector for TypeScript/JavaScript projects.' }],
    ['meta', { name: 'twitter:image', content: 'https://archlinter.github.io/archlint/logo.png' }],
  ],

  locales: {
    root: {
      label: 'English',
      lang: 'en',
      themeConfig: {
        nav: enNav,
        sidebar: enSidebar
      }
    },
    ru: {
      label: 'Русский',
      lang: 'ru',
      title: 'archlint',
      description: 'Остановите деградацию архитектуры',
      themeConfig: {
        nav: [
          { text: 'Начало работы', link: '/ru/getting-started/' },
          { text: 'Детекторы', link: '/ru/detectors/' },
          { text: 'Конфигурация', link: '/ru/configuration/' },
          { text: 'CLI', link: '/ru/cli/' },
          { text: 'Интеграции', link: '/ru/integrations/eslint' },
        ],
        sidebar: {
          '/ru/getting-started/': [
            {
              text: 'Начало работы',
              items: [
                { text: 'Введение', link: '/ru/getting-started/' },
                { text: 'Установка', link: '/ru/getting-started/installation' },
                { text: 'Первое сканирование', link: '/ru/getting-started/first-scan' },
              ]
            }
          ],
          '/ru/detectors/': [
            { text: 'Введение', items: [{ text: 'Обзор', link: '/ru/detectors/' }] },
            {
              text: 'Проблемы зависимостей',
              items: [
                { text: 'Циклические зависимости', link: '/ru/detectors/cycles' },
                { text: 'Циклы типов', link: '/ru/detectors/circular-type-deps' },
                { text: 'Циклы пакетов', link: '/ru/detectors/package-cycle' },
                { text: 'Нарушение слоев', link: '/ru/detectors/layer-violation' },
                { text: 'Нарушение SDP', link: '/ru/detectors/sdp-violation' },
              ]
            },
            {
              text: 'Дизайн модулей',
              items: [
                { text: 'God Module', link: '/ru/detectors/god-module' },
                { text: 'Hub Module', link: '/ru/detectors/hub-module' },
                { text: 'Высокая связанность', link: '/ru/detectors/high-coupling' },
                { text: 'Рассредоточенный модуль', link: '/ru/detectors/scattered-module' },
                { text: 'Feature Envy', link: '/ru/detectors/feature-envy' },
              ]
            },
            {
              text: 'Качество кода',
              items: [
                { text: 'Мертвый код', link: '/ru/detectors/dead-code' },
                { text: 'Мертвые символы', link: '/ru/detectors/dead-symbols' },
                { text: 'Осиротевшие типы', link: '/ru/detectors/orphan-types' },
                { text: 'Злоупотребление Barrel', link: '/ru/detectors/barrel-abuse' },
                { text: 'Одержимость примитивами', link: '/ru/detectors/primitive-obsession' },
              ]
            },
            {
              text: 'Сложность и размер',
              items: [
                { text: 'Высокая сложность', link: '/ru/detectors/complexity' },
                { text: 'Глубокая вложенность', link: '/ru/detectors/deep-nesting' },
                { text: 'Много параметров', link: '/ru/detectors/long-params' },
                { text: 'Большой файл', link: '/ru/detectors/large-file' },
              ]
            },
            {
              text: 'Паттерны изменений',
              items: [
                { text: 'Shotgun Surgery', link: '/ru/detectors/shotgun-surgery' },
                { text: 'Нестабильный интерфейс', link: '/ru/detectors/unstable-interface' },
              ]
            },
            {
              text: 'Runtime и безопасность',
              items: [
                { text: 'Утечка тестов', link: '/ru/detectors/test-leakage' },
                { text: 'Связанность с вендором', link: '/ru/detectors/vendor-coupling' },
                { text: 'Импорт с побочными эффектами', link: '/ru/detectors/side-effect-import' },
                { text: 'Общее мутабельное состояние', link: '/ru/detectors/shared-mutable-state' },
              ]
            },
            {
              text: 'Метрики',
              items: [
                { text: 'Нарушение абстрактности', link: '/ru/detectors/abstractness' },
                { text: 'Рассредоточенная конфигурация', link: '/ru/detectors/scattered-config' },
              ]
            }
          ],
          '/ru/cli/': [
            {
              text: 'Справочник CLI',
              items: [
                { text: 'Обзор', link: '/ru/cli/' },
                { text: 'init', link: '/ru/cli/init' },
                { text: 'scan', link: '/ru/cli/scan' },
                { text: 'diff', link: '/ru/cli/diff' },
                { text: 'snapshot', link: '/ru/cli/snapshot' },
                { text: 'watch', link: '/ru/cli/watch' },
              ]
            }
          ],
          '/ru/configuration/': [
            {
              text: 'Конфигурация',
              items: [
                { text: 'Обзор', link: '/ru/configuration/' },
                { text: 'Слои', link: '/ru/configuration/layers' },
                { text: 'Игнорирование', link: '/ru/configuration/ignore' },
              ]
            },
            {
              text: 'Фреймворки',
              items: [
                { text: 'Обзор', link: '/ru/frameworks/' },
                { text: 'NestJS', link: '/ru/frameworks/nestjs' },
                { text: 'Next.js', link: '/ru/frameworks/nextjs' },
                { text: 'React', link: '/ru/frameworks/react' },
              ]
            }
          ],
          '/ru/frameworks/': [
            {
              text: 'Конфигурация',
              items: [
                { text: 'Обзор', link: '/ru/configuration/' },
                { text: 'Слои', link: '/ru/configuration/layers' },
                { text: 'Игнорирование', link: '/ru/configuration/ignore' },
              ]
            },
            {
              text: 'Фреймворки',
              items: [
                { text: 'Обзор', link: '/ru/frameworks/' },
                { text: 'NestJS', link: '/ru/frameworks/nestjs' },
                { text: 'Next.js', link: '/ru/frameworks/nextjs' },
                { text: 'React', link: '/ru/frameworks/react' },
              ]
            }
          ],
          '/ru/integrations/': [
            {
              text: 'Интеграции',
              items: [
                { text: 'ESLint', link: '/ru/integrations/eslint' },
                { text: 'MCP Server', link: '/ru/integrations/mcp-server' },
                { text: 'GitHub Actions', link: '/ru/integrations/github-actions' },
                { text: 'GitLab CI', link: '/ru/integrations/gitlab-ci' },
              ]
            }
          ],
        }
      }
    },
    zh: {
      label: '简体中文',
      lang: 'zh-CN',
      title: 'archlint',
      description: '阻止架构恶化',
      themeConfig: {
        nav: [
          { text: '开始使用', link: '/zh/getting-started/' },
          { text: '检测器', link: '/zh/detectors/' },
          { text: '配置', link: '/zh/configuration/' },
          { text: 'CLI', link: '/zh/cli/' },
          { text: '集成', link: '/zh/integrations/eslint' },
        ],
        sidebar: {
          '/zh/getting-started/': [
            {
              text: '开始使用',
              items: [
                { text: '简介', link: '/zh/getting-started/' },
                { text: '安装', link: '/zh/getting-started/installation' },
                { text: '第一次扫描', link: '/zh/getting-started/first-scan' },
              ]
            }
          ],
          '/zh/detectors/': [
            { text: '简介', items: [{ text: '概述', link: '/zh/detectors/' }] },
            {
              text: '依赖问题',
              items: [
                { text: '循环依赖', link: '/zh/detectors/cycles' },
                { text: '类型循环', link: '/zh/detectors/circular-type-deps' },
                { text: '包循环', link: '/zh/detectors/package-cycle' },
                { text: '图层违规', link: '/zh/detectors/layer-violation' },
                { text: 'SDP 违规', link: '/zh/detectors/sdp-violation' },
              ]
            },
            {
              text: '模块设计',
              items: [
                { text: '上帝模块', link: '/zh/detectors/god-module' },
                { text: '枢纽模块', link: '/zh/detectors/hub-module' },
                { text: '高耦合', link: '/zh/detectors/high-coupling' },
                { text: '分散模块', link: '/zh/detectors/scattered-module' },
                { text: '特性嫉妒', link: '/zh/detectors/feature-envy' },
              ]
            },
            {
              text: '代码质量',
              items: [
                { text: '死代码', link: '/zh/detectors/dead-code' },
                { text: '死符号', link: '/zh/detectors/dead-symbols' },
                { text: '孤立类型', link: '/zh/detectors/orphan-types' },
                { text: 'Barrel 滥用', link: '/zh/detectors/barrel-abuse' },
                { text: '原始类型偏执', link: '/zh/detectors/primitive-obsession' },
              ]
            },
            {
              text: '复杂度与大小',
              items: [
                { text: '高复杂度', link: '/zh/detectors/complexity' },
                { text: '深层嵌套', link: '/zh/detectors/deep-nesting' },
                { text: '参数过多', link: '/zh/detectors/long-params' },
                { text: '大文件', link: '/zh/detectors/large-file' },
              ]
            },
            {
              text: '变更模式',
              items: [
                { text: '霰弹式修改', link: '/zh/detectors/shotgun-surgery' },
                { text: '不稳定接口', link: '/zh/detectors/unstable-interface' },
              ]
            },
            {
              text: '运行时与安全',
              items: [
                { text: '测试泄漏', link: '/zh/detectors/test-leakage' },
                { text: '供应商耦合', link: '/zh/detectors/vendor-coupling' },
                { text: '副作用导入', link: '/zh/detectors/side-effect-import' },
                { text: '共享可变状态', link: '/zh/detectors/shared-mutable-state' },
              ]
            },
            {
              text: '指标',
              items: [
                { text: '抽象性违规', link: '/zh/detectors/abstractness' },
                { text: '分散配置', link: '/zh/detectors/scattered-config' },
              ]
            }
          ],
          '/zh/cli/': [
            {
              text: 'CLI 参考',
              items: [
                { text: '概述', link: '/zh/cli/' },
                { text: 'init', link: '/zh/cli/init' },
                { text: 'scan', link: '/zh/cli/scan' },
                { text: 'diff', link: '/zh/cli/diff' },
                { text: 'snapshot', link: '/zh/cli/snapshot' },
                { text: 'watch', link: '/zh/cli/watch' },
              ]
            }
          ],
          '/zh/configuration/': [
            {
              text: '配置',
              items: [
                { text: '概述', link: '/zh/configuration/' },
                { text: '层级', link: '/zh/configuration/layers' },
                { text: '忽略', link: '/zh/configuration/ignore' },
              ]
            },
            {
              text: '框架',
              items: [
                { text: '概述', link: '/zh/frameworks/' },
                { text: 'NestJS', link: '/zh/frameworks/nestjs' },
                { text: 'Next.js', link: '/zh/frameworks/nextjs' },
                { text: 'React', link: '/zh/frameworks/react' },
              ]
            }
          ],
          '/zh/frameworks/': [
            {
              text: '配置',
              items: [
                { text: '概述', link: '/zh/configuration/' },
                { text: '层级', link: '/zh/configuration/layers' },
                { text: '忽略', link: '/zh/configuration/ignore' },
              ]
            },
            {
              text: '框架',
              items: [
                { text: '概述', link: '/zh/frameworks/' },
                { text: 'NestJS', link: '/zh/frameworks/nestjs' },
                { text: 'Next.js', link: '/zh/frameworks/nextjs' },
                { text: 'React', link: '/zh/frameworks/react' },
              ]
            }
          ],
          '/zh/integrations/': [
            {
              text: '集成',
              items: [
                { text: 'ESLint', link: '/zh/integrations/eslint' },
                { text: 'MCP Server', link: '/zh/integrations/mcp-server' },
                { text: 'GitHub Actions', link: '/zh/integrations/github-actions' },
                { text: 'GitLab CI', link: '/zh/integrations/gitlab-ci' },
              ]
            }
          ],
        }
      }
    },
    pt: {
      label: 'Português',
      lang: 'pt-BR',
      title: 'archlint',
      description: 'Impedir a degradação da arquitetura',
      themeConfig: {
        nav: [
          { text: 'Primeiros Passos', link: '/pt/getting-started/' },
          { text: 'Detectores', link: '/pt/detectors/' },
          { text: 'Configuração', link: '/pt/configuration/' },
          { text: 'CLI', link: '/pt/cli/' },
          { text: 'Integrações', link: '/pt/integrations/eslint' },
        ],
        sidebar: {
          '/pt/getting-started/': [
            {
              text: 'Primeiros Passos',
              items: [
                { text: 'Introdução', link: '/pt/getting-started/' },
                { text: 'Instalação', link: '/pt/getting-started/installation' },
                { text: 'Primeira Verificação', link: '/pt/getting-started/first-scan' },
              ]
            }
          ],
          '/pt/detectors/': [
            { text: 'Introdução', items: [{ text: 'Visão Geral', link: '/pt/detectors/' }] },
            {
              text: 'Problemas de Dependência',
              items: [
                { text: 'Dependências Cíclicas', link: '/pt/detectors/cycles' },
                { text: 'Ciclos de Tipos', link: '/pt/detectors/circular-type-deps' },
                { text: 'Ciclos de Pacotes', link: '/pt/detectors/package-cycle' },
                { text: 'Violação de Camadas', link: '/pt/detectors/layer-violation' },
                { text: 'Violação de SDP', link: '/pt/detectors/sdp-violation' },
              ]
            },
            {
              text: 'Design de Módulo',
              items: [
                { text: 'Módulo Deus', link: '/pt/detectors/god-module' },
                { text: 'Módulo Hub', link: '/pt/detectors/hub-module' },
                { text: 'Alto Acoplamento', link: '/pt/detectors/high-coupling' },
                { text: 'Módulo Disperso', link: '/pt/detectors/scattered-module' },
                { text: 'Inveja de Recursos', link: '/pt/detectors/feature-envy' },
              ]
            },
            {
              text: 'Qualidade do Código',
              items: [
                { text: 'Código Morto', link: '/pt/detectors/dead-code' },
                { text: 'Símbolos Mortos', link: '/pt/detectors/dead-symbols' },
                { text: 'Tipos Órfãos', link: '/pt/detectors/orphan-types' },
                { text: 'Abuso de Barrel', link: '/pt/detectors/barrel-abuse' },
                { text: 'Obsessão por Primitivos', link: '/pt/detectors/primitive-obsession' },
              ]
            },
            {
              text: 'Complexidade e Tamanho',
              items: [
                { text: 'Alta Complexidade', link: '/pt/detectors/complexity' },
                { text: 'Aninhamento Profundo', link: '/pt/detectors/deep-nesting' },
                { text: 'Muitos Parâmetros', link: '/pt/detectors/long-params' },
                { text: 'Arquivo Grande', link: '/pt/detectors/large-file' },
              ]
            },
            {
              text: 'Padrões de Mudança',
              items: [
                { text: 'Cirurgia por Perdigotos', link: '/pt/detectors/shotgun-surgery' },
                { text: 'Interface Instável', link: '/pt/detectors/unstable-interface' },
              ]
            },
            {
              text: 'Execução e Segurança',
              items: [
                { text: 'Vazamento de Testes', link: '/pt/detectors/test-leakage' },
                { text: 'Acoplamento com Fornecedor', link: '/pt/detectors/vendor-coupling' },
                { text: 'Importação com Efeito Colateral', link: '/pt/detectors/side-effect-import' },
                { text: 'Estado Mutável Compartilhado', link: '/pt/detectors/shared-mutable-state' },
              ]
            },
            {
              text: 'Métricas',
              items: [
                { text: 'Violação de Abstratividade', link: '/pt/detectors/abstractness' },
                { text: 'Configuração Dispersa', link: '/pt/detectors/scattered-config' },
              ]
            }
          ],
          '/pt/cli/': [
            {
              text: 'Referência CLI',
              items: [
                { text: 'Visão Geral', link: '/pt/cli/' },
                { text: 'init', link: '/pt/cli/init' },
                { text: 'scan', link: '/pt/cli/scan' },
                { text: 'diff', link: '/pt/cli/diff' },
                { text: 'snapshot', link: '/pt/cli/snapshot' },
                { text: 'watch', link: '/pt/cli/watch' },
              ]
            }
          ],
          '/pt/configuration/': [
            {
              text: 'Configuração',
              items: [
                { text: 'Visão Geral', link: '/pt/configuration/' },
                { text: 'Camadas', link: '/pt/configuration/layers' },
                { text: 'Ignorar', link: '/pt/configuration/ignore' },
              ]
            },
            {
              text: 'Frameworks',
              items: [
                { text: 'Visão Geral', link: '/pt/frameworks/' },
                { text: 'NestJS', link: '/pt/frameworks/nestjs' },
                { text: 'Next.js', link: '/pt/frameworks/nextjs' },
                { text: 'React', link: '/pt/frameworks/react' },
              ]
            }
          ],
          '/pt/frameworks/': [
            {
              text: 'Configuração',
              items: [
                { text: 'Visão Geral', link: '/pt/configuration/' },
                { text: 'Camadas', link: '/pt/configuration/layers' },
                { text: 'Ignorar', link: '/pt/configuration/ignore' },
              ]
            },
            {
              text: 'Frameworks',
              items: [
                { text: 'Visão Geral', link: '/pt/frameworks/' },
                { text: 'NestJS', link: '/pt/frameworks/nestjs' },
                { text: 'Next.js', link: '/pt/frameworks/nextjs' },
                { text: 'React', link: '/pt/frameworks/react' },
              ]
            }
          ],
          '/pt/integrations/': [
            {
              text: 'Integrações',
              items: [
                { text: 'ESLint', link: '/pt/integrations/eslint' },
                { text: 'MCP Server', link: '/pt/integrations/mcp-server' },
                { text: 'GitHub Actions', link: '/pt/integrations/github-actions' },
                { text: 'GitLab CI', link: '/pt/integrations/gitlab-ci' },
              ]
            }
          ],
        }
      }
    },
    es: {
      label: 'Español',
      lang: 'es-ES',
      title: 'archlint',
      description: 'Detener la degradación de la arquitectura',
      themeConfig: {
        nav: [
          { text: 'Primeros Pasos', link: '/es/getting-started/' },
          { text: 'Detectores', link: '/es/detectors/' },
          { text: 'Configuración', link: '/es/configuration/' },
          { text: 'CLI', link: '/es/cli/' },
          { text: 'Integraciones', link: '/es/integrations/eslint' },
        ],
        sidebar: {
          '/es/getting-started/': [
            {
              text: 'Primeros Pasos',
              items: [
                { text: 'Introducción', link: '/es/getting-started/' },
                { text: 'Instalación', link: '/es/getting-started/installation' },
                { text: 'Primer Escaneo', link: '/es/getting-started/first-scan' },
              ]
            }
          ],
          '/es/detectors/': [
            { text: 'Introducción', items: [{ text: 'Resumen', link: '/es/detectors/' }] },
            {
              text: 'Problemas de Dependencia',
              items: [
                { text: 'Dependencias Cíclicas', link: '/es/detectors/cycles' },
                { text: 'Ciclos de Tipos', link: '/es/detectors/circular-type-deps' },
                { text: 'Ciclos de Paquetes', link: '/es/detectors/package-cycle' },
                { text: 'Violación de Capas', link: '/es/detectors/layer-violation' },
                { text: 'Violación de SDP', link: '/es/detectors/sdp-violation' },
              ]
            },
            {
              text: 'Diseño de Módulos',
              items: [
                { text: 'Módulo Dios', link: '/es/detectors/god-module' },
                { text: 'Módulo Hub', link: '/es/detectors/hub-module' },
                { text: 'Alto Acoplamiento', link: '/es/detectors/high-coupling' },
                { text: 'Módulo Disperso', link: '/es/detectors/scattered-module' },
                { text: 'Envidia de Funcionalidad', link: '/es/detectors/feature-envy' },
              ]
            },
            {
              text: 'Calidad del Código',
              items: [
                { text: 'Código Muerto', link: '/es/detectors/dead-code' },
                { text: 'Símbolos Muertos', link: '/es/detectors/dead-symbols' },
                { text: 'Tipos Huérfanos', link: '/es/detectors/orphan-types' },
                { text: 'Abuso de Barrel', link: '/es/detectors/barrel-abuse' },
                { text: 'Obsesión por Primitivos', link: '/es/detectors/primitive-obsession' },
              ]
            },
            {
              text: 'Complejidad y Tamaño',
              items: [
                { text: 'Alta Complejidad', link: '/es/detectors/complexity' },
                { text: 'Anidamiento Profundo', link: '/es/detectors/deep-nesting' },
                { text: 'Demasiados Parámetros', link: '/es/detectors/long-params' },
                { text: 'Archivo Grande', link: '/es/detectors/large-file' },
              ]
            },
            {
              text: 'Patrones de Cambio',
              items: [
                { text: 'Cirugía de Escopeta', link: '/es/detectors/shotgun-surgery' },
                { text: 'Interfaz Inestable', link: '/es/detectors/unstable-interface' },
              ]
            },
            {
              text: 'Ejecución y Seguridad',
              items: [
                { text: 'Fuga de Pruebas', link: '/es/detectors/test-leakage' },
                { text: 'Acoplamiento con Proveedor', link: '/es/detectors/vendor-coupling' },
                { text: 'Importación con Efecto Secundario', link: '/es/detectors/side-effect-import' },
                { text: 'Estado Mutable Compartido', link: '/es/detectors/shared-mutable-state' },
              ]
            },
            {
              text: 'Métricas',
              items: [
                { text: 'Violación de Abstractez', link: '/es/detectors/abstractness' },
                { text: 'Configuración Dispersa', link: '/es/detectors/scattered-config' },
              ]
            }
          ],
          '/es/cli/': [
            {
              text: 'Referencia CLI',
              items: [
                { text: 'Resumen', link: '/es/cli/' },
                { text: 'init', link: '/es/cli/init' },
                { text: 'scan', link: '/es/cli/scan' },
                { text: 'diff', link: '/es/cli/diff' },
                { text: 'snapshot', link: '/es/cli/snapshot' },
                { text: 'watch', link: '/es/cli/watch' },
              ]
            }
          ],
          '/es/configuration/': [
            {
              text: 'Configuración',
              items: [
                { text: 'Resumen', link: '/es/configuration/' },
                { text: 'Capas', link: '/es/configuration/layers' },
                { text: 'Ignorar', link: '/es/configuration/ignore' },
              ]
            },
            {
              text: 'Frameworks',
              items: [
                { text: 'Resumen', link: '/es/frameworks/' },
                { text: 'NestJS', link: '/es/frameworks/nestjs' },
                { text: 'Next.js', link: '/es/frameworks/nextjs' },
                { text: 'React', link: '/es/frameworks/react' },
              ]
            }
          ],
          '/es/frameworks/': [
            {
              text: 'Configuración',
              items: [
                { text: 'Resumen', link: '/es/configuration/' },
                { text: 'Capas', link: '/es/configuration/layers' },
                { text: 'Ignorar', link: '/es/configuration/ignore' },
              ]
            },
            {
              text: 'Frameworks',
              items: [
                { text: 'Resumen', link: '/es/frameworks/' },
                { text: 'NestJS', link: '/es/frameworks/nestjs' },
                { text: 'Next.js', link: '/es/frameworks/nextjs' },
                { text: 'React', link: '/es/frameworks/react' },
              ]
            }
          ],
          '/es/integrations/': [
            {
              text: 'Integraciones',
              items: [
                { text: 'ESLint', link: '/es/integrations/eslint' },
                { text: 'MCP Server', link: '/es/integrations/mcp-server' },
                { text: 'GitHub Actions', link: '/es/integrations/github-actions' },
                { text: 'GitLab CI', link: '/es/integrations/gitlab-ci' },
              ]
            }
          ],
        }
      }
    },
    ja: {
      label: '日本語',
      lang: 'ja-JP',
      title: 'archlint',
      description: 'アーキテクチャの劣化を止める',
      themeConfig: {
        nav: [
          { text: 'はじめに', link: '/ja/getting-started/' },
          { text: '検出器', link: '/ja/detectors/' },
          { text: '設定', link: '/ja/configuration/' },
          { text: 'CLI', link: '/ja/cli/' },
          { text: '統合', link: '/ja/integrations/eslint' },
        ],
        sidebar: {
          '/ja/getting-started/': [
            {
              text: 'はじめに',
              items: [
                { text: 'イントロダクション', link: '/ja/getting-started/' },
                { text: 'インストール', link: '/ja/getting-started/installation' },
                { text: '最初のスキャン', link: '/ja/getting-started/first-scan' },
              ]
            }
          ],
          '/ja/detectors/': [
            { text: 'イントロダクション', items: [{ text: '概要', link: '/ja/detectors/' }] },
            {
              text: '依存関係の問題',
              items: [
                { text: '循環依存', link: '/ja/detectors/cycles' },
                { text: '型の循環', link: '/ja/detectors/circular-type-deps' },
                { text: 'パッケージの循環', link: '/ja/detectors/package-cycle' },
                { text: 'レイヤー違反', link: '/ja/detectors/layer-violation' },
                { text: 'SDP違反', link: '/ja/detectors/sdp-violation' },
              ]
            },
            {
              text: 'モジュール設計',
              items: [
                { text: 'ゴッドモジュール', link: '/ja/detectors/god-module' },
                { text: 'ハブモジュール', link: '/ja/detectors/hub-module' },
                { text: '高い結合度', link: '/ja/detectors/high-coupling' },
                { text: '分散モジュール', link: '/ja/detectors/scattered-module' },
                { text: '機能への執着', link: '/ja/detectors/feature-envy' },
              ]
            },
            {
              text: 'コード品質',
              items: [
                { text: 'デッドコード', link: '/ja/detectors/dead-code' },
                { text: 'デッドシンボル', link: '/ja/detectors/dead-symbols' },
                { text: '孤立した型', link: '/ja/detectors/orphan-types' },
                { text: 'バレル濫用', link: '/ja/detectors/barrel-abuse' },
                { text: '基本データ型への執着', link: '/ja/detectors/primitive-obsession' },
              ]
            },
            {
              text: '複雑度とサイズ',
              items: [
                { text: '高い複雑度', link: '/ja/detectors/complexity' },
                { text: '深いネスト', link: '/ja/detectors/deep-nesting' },
                { text: '多すぎる引数', link: '/ja/detectors/long-params' },
                { text: '巨大なファイル', link: '/ja/detectors/large-file' },
              ]
            },
            {
              text: '変更パターン',
              items: [
                { text: '散弾銃の手術', link: '/ja/detectors/shotgun-surgery' },
                { text: '不安定なインターフェース', link: '/ja/detectors/unstable-interface' },
              ]
            },
            {
              text: '実行時と安全性',
              items: [
                { text: 'テストの漏洩', link: '/ja/detectors/test-leakage' },
                { text: 'ベンダー結合', link: '/ja/detectors/vendor-coupling' },
                { text: '副作用のあるインポート', link: '/ja/detectors/side-effect-import' },
                { text: '共有された可変状態', link: '/ja/detectors/shared-mutable-state' },
              ]
            },
            {
              text: 'メトリクス',
              items: [
                { text: '抽象性違反', link: '/ja/detectors/abstractness' },
                { text: '分散した設定', link: '/ja/detectors/scattered-config' },
              ]
            }
          ],
          '/ja/cli/': [
            {
              text: 'CLIリファレンス',
              items: [
                { text: '概要', link: '/ja/cli/' },
                { text: 'init', link: '/ja/cli/init' },
                { text: 'scan', link: '/ja/cli/scan' },
                { text: 'diff', link: '/ja/cli/diff' },
                { text: 'snapshot', link: '/ja/cli/snapshot' },
                { text: 'watch', link: '/ja/cli/watch' },
              ]
            }
          ],
          '/ja/configuration/': [
            {
              text: '設定',
              items: [
                { text: '概要', link: '/ja/configuration/' },
                { text: 'レイヤー', link: '/ja/configuration/layers' },
                { text: '無視', link: '/ja/configuration/ignore' },
              ]
            },
            {
              text: 'フレームワーク',
              items: [
                { text: '概要', link: '/ja/frameworks/' },
                { text: 'NestJS', link: '/ja/frameworks/nestjs' },
                { text: 'Next.js', link: '/ja/frameworks/nextjs' },
                { text: 'React', link: '/ja/frameworks/react' },
              ]
            }
          ],
          '/ja/frameworks/': [
            {
              text: '設定',
              items: [
                { text: '概要', link: '/ja/configuration/' },
                { text: 'レイヤー', link: '/ja/configuration/layers' },
                { text: '無視', link: '/ja/configuration/ignore' },
              ]
            },
            {
              text: 'フレームワーク',
              items: [
                { text: '概要', link: '/ja/frameworks/' },
                { text: 'NestJS', link: '/ja/frameworks/nestjs' },
                { text: 'Next.js', link: '/ja/frameworks/nextjs' },
                { text: 'React', link: '/ja/frameworks/react' },
              ]
            }
          ],
          '/ja/integrations/': [
            {
              text: '統合',
              items: [
                { text: 'ESLint', link: '/ja/integrations/eslint' },
                { text: 'MCP Server', link: '/ja/integrations/mcp-server' },
                { text: 'GitHub Actions', link: '/ja/integrations/github-actions' },
                { text: 'GitLab CI', link: '/ja/integrations/gitlab-ci' },
              ]
            }
          ],
        }
      }
    },
  },

  markdown: {
    config(md) {
      md.use(groupIconMdPlugin)
    },
  },
  vite: {
    plugins: [
      groupIconVitePlugin()
    ],
  },
  themeConfig: {
    logo: '/logo.svg',
    socialLinks: [
      { icon: 'github', link: 'https://github.com/archlinter/archlint' }
    ],
    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2024-present'
    }
  }
})
