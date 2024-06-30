import tseslint from 'typescript-eslint';
import pluginVue from 'eslint-plugin-vue';
import eslintConfigPrettier from 'eslint-config-prettier';
import eslintPluginPrettierRecommended from 'eslint-plugin-prettier/recommended';


 export default tseslint.config(
    ...tseslint.configs.recommended,
    ...pluginVue.configs['flat/recommended'],
    {
      plugins: {
        'typescript-eslint': tseslint.plugin,
      },
      languageOptions: {
        parserOptions: {
          parser: tseslint.parser,
          extraFileExtensions: ['.vue'],
          sourceType: 'module',
        },
      },
      rules: {
        "prefer-const": "error",
        "prettier/prettier": "error"
      },
    },
    eslintPluginPrettierRecommended,
    eslintConfigPrettier
 );