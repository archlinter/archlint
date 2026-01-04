import type { Rule } from 'eslint';
import { createArchlintRule } from '../utils/rule-factory';

export const noDeepNesting: Rule.RuleModule = createArchlintRule({
  detectorId: 'deep_nesting',
  messageId: 'smell',
  description: 'Disallow excessively deep directory nesting',
  category: 'Architecture',
  recommended: false,
  strategy: 'all-files',
  messages: {
    smell: 'Deep nesting: {{reason}}',
  },
});
