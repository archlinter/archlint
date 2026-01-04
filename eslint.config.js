import js from '@eslint/js';
import tseslint from 'typescript-eslint';
import archlint from './packages/eslint-plugin/dist/index.js';

export default [
  js.configs.recommended,
  ...tseslint.configs.recommended,
  archlint.configs['flat/recommended'],
  {
    ignores: [
      '**/dist/',
      '**/node_modules/',
      '**/*.d.ts',
      'target/',
      'crates/archlint/test_data/',
      '**/__tests__/fixtures/**',
      '**/*.config.ts',
      '**/vitest.config.ts',
      '**/tsup.config.ts',
      'packages/core/__tests__/**',
    ],
  },
  {
    files: ['**/*.{js,ts}'],
    languageOptions: {
      ecmaVersion: 2022,
      sourceType: 'module',
      globals: {
        process: 'readonly',
        __dirname: 'readonly',
        require: 'readonly',
        module: 'readonly',
        console: 'readonly',
      },
    },
    rules: {
      'no-console': 'warn',
      '@typescript-eslint/no-explicit-any': 'error',
      '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
      '@typescript-eslint/no-require-imports': 'off',
      'no-unused-vars': 'off',
    },
  },
  ...tseslint.configs.recommendedTypeChecked.map((config) => ({
    ...config,
    files: ['**/*.ts'],
    languageOptions: {
      ...config.languageOptions,
      parserOptions: {
        ...config.languageOptions?.parserOptions,
        project: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
  })),
  {
    files: ['**/__tests__/**/*.ts', '**/*.test.ts'],
    rules: {
      '@typescript-eslint/no-explicit-any': 'off',
      '@typescript-eslint/no-unsafe-assignment': 'off',
      '@typescript-eslint/no-unsafe-call': 'off',
      '@typescript-eslint/no-unsafe-member-access': 'off',
      '@typescript-eslint/no-unsafe-return': 'off',
      '@typescript-eslint/no-unsafe-argument': 'off',
    },
  },
  {
    files: ['packages/eslint-plugin/src/utils/rule-factory.ts', 'packages/eslint-plugin/src/utils/smell-filter.ts', 'packages/eslint-plugin/src/rules/*.ts', 'packages/eslint-plugin/src/configs/*.ts'],
    rules: {
      '@typescript-eslint/no-unsafe-assignment': 'off',
      '@typescript-eslint/no-unsafe-call': 'off',
      '@typescript-eslint/no-unsafe-member-access': 'off',
      '@typescript-eslint/no-unsafe-argument': 'off',
      '@typescript-eslint/no-explicit-any': 'off',
    },
  },
];
