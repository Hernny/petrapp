---
name: create-pr
description: 'Autogenerates a detailed description and checklist to prepare a Pull Request pointing directly to the main branch. Use when finishing your tasks.'
---

Please review all the active or introduced changes in my current branch and generate a complete Markdown description with the proper format for a Pull Request targeting the `main` branch.

Align the PR with our predefined branching flow and deduce the correct type based on the changes (`feature`, `fix`, `refactor`, `chore`).

The PR should follow this template exactly:

```markdown
## Description
[Generate a detailed and explanatory summary of why this code changes and what the new feature or fixed bug consists of]

## Type of Change
- [ ] 🚀 New feature (feature)
- [ ] 🐛 Bug fix (fix)
- [ ] 🔨 Refactoring (refactor)
- [ ] ⚙️ Maintenance/Configuration (chore)

## Screenshots / Evidence of the Change
(Add images or text blocks, if applicable)

## Technical Review Checklist
- [ ] The code follows the project's templates and styling.
- [ ] Verified that the branch name conforms to the proper format and convention.
- [ ] I have tested my changes locally and they work correctly.
- [ ] My commit messages are clear and concise (Conventional Commits).
```

Please proactively fill in the "Description" section, pre-check (with an `x`) the correct "Type of Change", and identify areas where a PR reviewer should pay special attention.
