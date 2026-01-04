import type { Rule } from 'eslint';
import { createArchlintRule } from '../utils/rule-factory';

export const noHighCoupling: Rule.RuleModule = createArchlintRule({
  detectorId: 'high_coupling',
  messageId: 'smell',
  description: 'Disallow modules with excessively high coupling',
  category: 'Architecture',
  recommended: true,
  strategy: 'primary-file',
  messages: {
    smell: 'High coupling detected: {{reason}}',
  },
});
