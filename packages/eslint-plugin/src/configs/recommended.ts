// Legacy configs (ESLint < 9) - uses eslintrc format, not typed by ESLint 9
export const recommended = {
  plugins: ['@archlinter'],
  rules: {
    '@archlinter/no-cycles': 'error',
    '@archlinter/no-god-modules': 'warn',
    '@archlinter/no-dead-code': 'warn',
    '@archlinter/no-dead-symbols': 'warn',
    '@archlinter/no-high-coupling': 'warn',
    '@archlinter/no-high-complexity': 'error',
    '@archlinter/no-layer-violations': 'error',
    '@archlinter/no-code-clone': 'warn',
  },
} as const;

export const strict = {
  plugins: ['@archlinter'],
  rules: {
    '@archlinter/no-cycles': 'error',
    '@archlinter/no-god-modules': 'error',
    '@archlinter/no-dead-code': 'error',
    '@archlinter/no-dead-symbols': 'error',
    '@archlinter/no-high-coupling': 'error',
    '@archlinter/no-high-complexity': 'error',
    '@archlinter/no-barrel-abuse': 'error',
    '@archlinter/no-layer-violations': 'error',
    '@archlinter/no-sdp-violations': 'error',
    '@archlinter/no-hub-modules': 'warn',
    '@archlinter/no-deep-nesting': 'error',
    '@archlinter/no-long-params': 'warn',
    '@archlinter/no-code-clone': 'error',
    '@archlinter/no-regression': ['error', { failOn: 'medium' }],
  },
} as const;
