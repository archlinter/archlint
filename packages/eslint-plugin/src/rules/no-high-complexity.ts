import { createArchlintRule } from '../utils/rule-factory';

export const noHighComplexity = createArchlintRule({
  detectorId: 'high_complexity',
  messageId: 'smell',
  description: 'Disallow functions with high cyclomatic complexity',
  category: 'Code Quality',
  recommended: true,
  strategy: 'all-files',
  messages: {
    smell: 'High complexity detected: {{reason}}',
  },
});
