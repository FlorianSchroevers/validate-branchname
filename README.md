# validate-branchname

A [pre-commit](https://pre-commit.com/) hook for git branch name validation.

## ✨ Features

- ✅ Validates the **current Git branch name** against a regex pattern.
- ✅ Works seamlessly with the [pre-commit framework](https://pre-commit.com/).

## ▶ Usage

Add the following to your `.pre-commit-config.yaml`:

```yaml
- repo: https://github.com/FlorianSchroevers/validate-branchname
  rev: 0.0.1
  hooks:
    - id: validate-branchname
      name: validate branchname
      args: ['--pattern', '^(fix|hotfix|feature)\/[0-9a-z\-]+$']
      pass_filenames: false
```

## ✅ Requirements

- Git
- [pre-commit](https://pre-commit.com/)

P.S. use [prek](https://github.com/j178/prek) for a faster implementation of pre-commit.
