import { createArchlintRule } from '../utils/rule-factory';

export const noHighCoupling = createArchlintRule({
  detectorId: 'high_coupling',
  messageId: 'smell',
  description: 'Disallow modules with excessively high coupling',
  category: 'Architecture',
  recommended: true,
  strategy: 'primary-file',
  messages: {
    smell: 'High coupling detected: {{reason}}',
  },
});
