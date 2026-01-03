import { RuleTester } from '@typescript-eslint/rule-tester';
import { afterAll, it, describe } from 'vitest';

// Configure RuleTester to use vitest
RuleTester.afterAll = afterAll;
RuleTester.it = it;
RuleTester.describe = describe;

export const ruleTester = new RuleTester({
  languageOptions: {
    parserOptions: {
      ecmaVersion: 2022,
      sourceType: 'module',
    },
  },
});
