import type { Rule } from 'eslint';
import { createArchlintRule } from '../utils/rule-factory';

export const noSdpViolations: Rule.RuleModule = createArchlintRule({
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
