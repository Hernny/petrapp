---
name: commits
description: 'Generates a semantic commit message based on the current changes using conventional commits according to the branching flow.'
---

Based on the code changes that have been made, write a commit message following the [Conventional Commits](https://www.conventionalcommits.org/) convention.

Match the commit type with our predefined branching rules:
- `feat:` (if we are in a `feature/...` branch)
- `fix:` (if we are in a `fix/...` branch)
- `refactor:` (if we are in a `refactor/...` branch)
- `chore:` (if we are in a `chore/...` branch)

**Rules for the message:**
1. **Subject:** Must be short (maximum 50 characters), written in lowercase (except for proper nouns), and use an imperative tone (e.g., "add" instead of "added" or "adds").
2. **Body (Optional):** If the changes are extensive or complex, add a blank line after the subject and explain "what" changed and "why" (not "how", as that is seen in the code).

Generate only the final commit message text or a few viable options inside code blocks so they can be easily copied.
