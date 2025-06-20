# Contributing to Jikan_moe

We are thrilled that you want to contribute to **Affinity Bot**! This document will guide you through the contribution process, from reporting issues to submitting pull requests.

## Table of Contents
- [Contributing to Affinity Bot](#contributing-to-affinity-bot)
  - [Table of Contents](#table-of-contents)
  - [How to Contribute](#how-to-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Features](#suggesting-features)
  - [Pull Request Process](#pull-request-process)
  - [Code Guidelines](#code-guidelines)
    - [Rust Style Guidelines](#rust-style-guidelines)
    - [Commit Message Format](#commit-message-format)
---

## How to Contribute

You can contribute in several ways:
1. Reporting bugs.
2. Suggesting new features.
3. Submitting pull requests with fixes or improvements.
4. Improving documentation.

We welcome all contributions, but please ensure that your pull requests align with the project's vision and coding standards.

---

## Reporting Bugs

If you encounter a bug, please open an issue and include:
- A clear title and description.
- Steps to reproduce the bug.
- Expected behavior.
- Relevant logs, screenshots, or code snippets, if available.
---
## Suggesting Features
Feature suggestions are welcome! Please open an issue and provide:
- A detailed description of the feature.
- Use cases for the feature.
- Any screenshots, if applicable.
---
## Pull Request Process

1. **Fork the repository** and create your branch from `main`:
   
   `git checkout -b feature/new-feature`
  
2. Commit your changes with clear and descriptive commit messages:
   
   `git commit -m "Add feature: implement pagination"`
3. Push your branch to your forked repository:
   
    `git push origin feature/new-feature`

4. Open a pull request:
  - Ensure your pull request title and description are clear.
  - Reference any related issues or feature requests.
  - Tag reviewers, if necessary.


## Code Guidelines
### Rust Style Guidelines

  - Follow Rust’s official style guide.
  - Run `cargo fmt` to format your code.
  - Run `cargo clippy` to maintain idiomatic code

### Commit Message Format
  We follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification for writing commit messages:
  ```
feat: add new feature to API
fix: resolve issue with database connection
docs: update README with usage instructions
```
