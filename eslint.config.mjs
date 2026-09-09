import js from '@eslint/js';
import tseslint from 'typescript-eslint';
import hooks from 'eslint-plugin-react-hooks';
import globals from 'globals';

export default tseslint.config(
  { ignores: ['**/dist/**', '**/node_modules/**', 'target/**'] },
  js.configs.recommended,
  ...tseslint.configs.recommended,
  {
    files: ['apps/desktop/src/**/*.{ts,tsx}'],
    languageOptions: { globals: globals.browser },
    plugins: { 'react-hooks': hooks },
    rules: {
      'react-hooks/rules-of-hooks': 'error',
      'react-hooks/exhaustive-deps': 'error',
      '@typescript-eslint/no-explicit-any': 'error',
    },
  },
  {
    files: ['scripts/*.mjs', 'apps/desktop/*.ts'],
    languageOptions: { globals: globals.node },
  },
);
