import { createArchlintRule } from '../utils/rule-factory';

export const noDeepNesting = createArchlintRule({
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
