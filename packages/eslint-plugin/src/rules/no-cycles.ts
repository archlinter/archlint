import { createArchlintRule } from '../utils/rule-factory';

export const noCycles = createArchlintRule({
  detectorId: 'cycles',
  messageId: 'cycle',
  description: 'Disallow cyclic dependencies between files',
  category: 'Architecture',
  recommended: true,
  strategy: 'critical-edges',
  messages: {
    cycle: "Cyclic import to '{{target}}' ({{impact}} impact)",
    cycleCluster: 'File is part of cyclic dependency cluster ({{size}} files)',
  },
});
