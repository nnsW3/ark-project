const { resolve } = require("node:path");

const project = resolve(process.cwd(), "tsconfig.json");

/** @type {import("eslint").Linter.Config} */
module.exports = {
  extends: ["eslint:recommended", "prettier", "eslint-config-turbo"],
  plugins: ["only-warn", "unused-imports"],
  globals: {
    React: true,
    JSX: true
  },
  env: {
    node: true
  },
  settings: {
    "import/resolver": {
      typescript: {
        project
      }
    }
  },
  // rules: {
  //   "no-unused-vars": "warn",
  //   "@typescript-eslint/no-unused-vars": "warn",
  //   "unused-imports/no-unused-imports": "warn",
  //   "unused-imports/no-unused-vars": [
  //     "warn",
  //     {
  //       vars: "all",
  //       varsIgnorePattern: "^_",
  //       args: "after-used",
  //       argsIgnorePattern: "^_"
  //     }
  //   ]
  // },
  ignorePatterns: [
    // Ignore dotfiles
    ".*.js",
    "node_modules/",
    "dist/"
  ],
  overrides: [
    {
      files: ["*.js?(x)", "*.ts?(x)"]
    }
  ]
};
