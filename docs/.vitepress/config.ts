import { defineConfig } from 'vitepress'
import { groupIconMdPlugin, groupIconVitePlugin } from 'vitepress-plugin-group-icons'

export default defineConfig({
  base: '/archlint/',
  title: "archlint",
  description: "Stop architecture degradation",
  head: [['link', { rel: 'icon', href: '/logo.svg' }]],
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
    nav: [
      { text: 'Getting Started', link: '/getting-started/' },
      { text: 'Detectors', link: '/detectors/' },
      { text: 'Configuration', link: '/configuration/' },
      { text: 'CLI', link: '/cli/' },
      { text: 'Integrations', link: '/integrations/eslint' },
    ],
    sidebar: {
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
            { text: 'Thresholds', link: '/configuration/thresholds' },
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
    },
    socialLinks: [
      { icon: 'github', link: 'https://github.com/archlinter/archlint' }
    ],
    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2024-present'
    }
  }
})
