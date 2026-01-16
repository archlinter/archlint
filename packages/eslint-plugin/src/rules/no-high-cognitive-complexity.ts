import { createArchlintRule } from '../utils/rule-factory';

export const noHighCognitiveComplexity = createArchlintRule({
  detectorId: 'cognitive_complexity',
  messageId: 'smell',
  description: 'Disallow functions with high cognitive complexity (how hard it is to understand)',
  category: 'Code Quality',
  recommended: true,
  strategy: 'all-files',
  messages: {
    smell: 'High cognitive complexity detected: {{reason}}',
  },
});
