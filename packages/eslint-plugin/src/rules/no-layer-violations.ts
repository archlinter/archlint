import { createArchlintRule } from '../utils/rule-factory';

export const noLayerViolations = createArchlintRule({
  detectorId: 'layer_violation',
  messageId: 'violation',
  description: 'Disallow violations of defined architecture layers',
  category: 'Architecture',
  recommended: true,
  strategy: 'source-file',
  messages: {
    violation: 'Layer violation: {{reason}}',
  },
});
