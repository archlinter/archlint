import { createArchlintRule } from '../utils/rule-factory';

export const noCodeClone = createArchlintRule({
  detectorId: 'code_clone',
  messageId: 'smell',
  description: 'Disallow duplicated code blocks (code clones)',
  category: 'Code Quality',
  recommended: true,
  strategy: 'all-files',
  messages: {
    smell: 'Code clone detected: {{reason}}',
  },
});
