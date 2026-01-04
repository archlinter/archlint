import type { Rule } from 'eslint';
import { createArchlintRule } from '../utils/rule-factory';

export const noGodModules: Rule.RuleModule = createArchlintRule({
  detectorId: 'god_module',
  messageId: 'smell',
  description: 'Disallow overly large and complex modules (God Modules)',
  category: 'Architecture',
  recommended: true,
  strategy: 'primary-file',
  messages: {
    smell: 'God Module detected: {{reason}}',
  },
});
