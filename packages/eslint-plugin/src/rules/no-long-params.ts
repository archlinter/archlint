import { createArchlintRule } from '../utils/rule-factory';

export const noLongParams = createArchlintRule({
  detectorId: 'long_params',
  messageId: 'smell',
  description: 'Disallow functions with too many parameters',
  category: 'Architecture',
  recommended: false,
  strategy: 'all-files',
  messages: {
    smell: 'Too many parameters: {{reason}}',
  },
});
