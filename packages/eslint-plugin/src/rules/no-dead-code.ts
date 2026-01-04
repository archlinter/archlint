import type { Rule } from 'eslint';
import { createArchlintRule } from '../utils/rule-factory';

export const noDeadCode: Rule.RuleModule = createArchlintRule({
  detectorId: 'dead_code',
  messageId: 'smell',
  description: 'Disallow dead code and unused exports',
  category: 'Architecture',
  recommended: true,
  strategy: 'primary-file',
  messages: {
    smell: 'Dead code detected: {{reason}}',
  },
});
