import { createArchlintRule } from '../utils/rule-factory';

export const noHighComplexity = createArchlintRule({
  detectorId: 'high_complexity',
  messageId: 'smell',
  description: 'Disallow functions with high cyclomatic complexity',
  category: 'Code Quality',
  recommended: false,
  strategy: 'all-files',
  messages: {
    smell: 'High complexity detected: {{reason}}',
  },
});
