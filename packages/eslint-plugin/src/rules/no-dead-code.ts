import { createArchlintRule } from '../utils/rule-factory';

export const noDeadCode = createArchlintRule({
  detectorId: 'dead_code',
  messageId: 'smell',
  description: 'Disallow dead code and unused exports',
  category: 'Architecture',
  recommended: true,
  strategy: 'primary-file',
  messages: {
    smell: 'Dead code detected: {{reason}}',
  },
});
