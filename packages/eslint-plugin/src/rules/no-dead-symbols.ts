import { createArchlintRule } from '../utils/rule-factory';

export const noDeadSymbols = createArchlintRule({
  detectorId: 'dead_symbols',
  messageId: 'smell',
  description: 'Disallow unused functions, classes, and variables',
  category: 'Architecture',
  recommended: true,
  strategy: 'all-files',
  messages: {
    smell: 'Unused symbol detected: {{reason}}',
  },
});
