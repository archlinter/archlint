import type { Rule } from 'eslint';
import { createArchlintRule } from '../utils/rule-factory';

export const noBarrelAbuse: Rule.RuleModule = createArchlintRule({
  detectorId: 'barrel_file_abuse',
  messageId: 'smell',
  description: 'Disallow abuse of barrel files',
  category: 'Architecture',
  recommended: false,
  strategy: 'primary-file',
  messages: {
    smell: 'Barrel file abuse: {{reason}}',
  },
});
