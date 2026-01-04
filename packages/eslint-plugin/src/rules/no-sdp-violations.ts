import { createArchlintRule } from '../utils/rule-factory';

export const noSdpViolations = createArchlintRule({
  detectorId: 'sdp_violation',
  messageId: 'smell',
  description: 'Disallow violations of the Stable Dependencies Principle',
  category: 'Architecture',
  recommended: false,
  strategy: 'primary-file',
  messages: {
    smell: 'SDP violation: {{reason}}',
  },
});
