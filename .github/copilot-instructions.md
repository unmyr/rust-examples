# Commit messages rules
* Use conventional commit messages for all commits. The format is: `<type>(<scope>): <description>`.
* The `<type>` can be one of the following: feat, fix, docs, style, refactor, test, chore, ci.
* Use `ci` for changes to CI/CD automation, including GitHub Actions workflows and their build, validation, packaging, or deployment steps.
* For the GitHub Pages workflow, use the `pages` scope, e.g. `ci(pages): update site deployment`.
* If changes are only under `.github/`, use `ci` when they modify CI/CD automation; use `chore` for other repository metadata, instructions, and configuration.
* Use `docs` for changes to documentation content, including `*.adoc` files.
* For files under the docs directory, specify the full folder path as the scope in the Conventional Commit message.  
  Example: docs/books/algebra/index.adoc → docs(books/algebra)
* To make changes to copilot instructions, set the type to "chore" and scope to "copilot-instructions".
* Keep the subject focused on what changed. Use the commit body to explain why the change was made, especially when the purpose cannot be inferred from the staged diff alone.
