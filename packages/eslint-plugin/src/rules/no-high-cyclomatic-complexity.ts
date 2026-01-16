import { createArchlintRule } from '../utils/rule-factory';

export const noHighCyclomaticComplexity = createArchlintRule({
  detectorId: 'cyclomatic_complexity',
  messageId: 'smell',
  description: 'Disallow functions with high cyclomatic complexity',
  category: 'Code Quality',
  recommended: true,
  strategy: 'all-files',
  messages: {
    smell: 'High cyclomatic complexity detected: {{reason}}',
  },
});
