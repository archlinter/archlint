import type { Linter } from 'eslint';
import { rules } from '../rules';

export const flatRecommended: Linter.Config = {
  plugins: {
    '@archlinter': { rules } as any,
  },
  rules: {
    '@archlinter/no-cycles': 'error',
    '@archlinter/no-god-modules': 'warn',
    '@archlinter/no-dead-code': 'warn',
    '@archlinter/no-high-coupling': 'warn',
    '@archlinter/no-layer-violations': 'error',
  },
};

export const flatStrict: Linter.Config = {
  plugins: {
    '@archlinter': { rules } as any,
  },
  rules: {
    '@archlinter/no-cycles': 'error',
    '@archlinter/no-god-modules': 'error',
    '@archlinter/no-dead-code': 'error',
    '@archlinter/no-high-coupling': 'error',
    '@archlinter/no-barrel-abuse': 'error',
    '@archlinter/no-layer-violations': 'error',
    '@archlinter/no-sdp-violations': 'error',
    '@archlinter/no-hub-modules': 'warn',
    '@archlinter/no-deep-nesting': 'error',
    '@archlinter/no-long-params': 'warn',
  },
};
