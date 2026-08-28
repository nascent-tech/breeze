import js from '@eslint/js';
import globals from 'globals';

export default [
  { ignores: ['dist/**', 'out/**', 'coverage/**'] },
  js.configs.recommended,
  {
    languageOptions: {
      ecmaVersion: 2022,
      sourceType: 'module',
      globals: { ...globals.node },
    },
    rules: {
      'no-unused-vars': 'error',
      'no-console': 'error',
      'no-restricted-syntax': [
        'error',
        {
          selector: "CallExpression[callee.name='require']",
          message: 'Only native-bridge-loader.js may load a CommonJS module.',
        },
      ],
    },
  },
  {
    files: ['build/**'],
    rules: { 'no-console': 'off' },
  },
  {
    files: ['**/native-bridge-loader.js'],
    rules: { 'no-restricted-syntax': 'off' },
  },
];
