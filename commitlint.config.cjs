module.exports = {
  extends: ['@commitlint/config-conventional'],
  rules: {
    'type-enum': [
      2,
      'always',
      [
        'feat',     // New feature
        'fix',      // Bug fix
        'perf',     // Performance improvement
        'refactor', // Code refactoring
        'docs',     // Documentation
        'test',     // Tests
        'chore',    // Maintenance
        'ci',       // CI/CD changes
        'build',    // Build system
      ],
    ],
    'body-max-line-length': [0, 'always', Infinity],
    'footer-max-line-length': [0, 'always', Infinity],
  },
};
