import { createArchlintRule } from '../utils/rule-factory';

export const noHubModules = createArchlintRule({
  detectorId: 'hub_module',
  messageId: 'smell',
  description: 'Disallow modules that act as too many dependencies',
  category: 'Architecture',
  recommended: false,
  strategy: 'primary-file',
  messages: {
    smell: 'Hub module detected: {{reason}}',
  },
});
