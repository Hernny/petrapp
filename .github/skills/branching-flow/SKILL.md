---
name: branching-flow
description: 'Applies the project branching flow and naming conventions. Use when creating features, fixes, refactors, or chores, and when opening Pull Requests to main.'
---

# Branching Flow

This skill defines the project's conventions and rules for naming Git branches and establishing the Pull Request workflow.

## When to Use

- When you are going to propose or create a new branch for development.
- When you need to review branch naming or version control conventions.
- When preparing a task for review or merge via a Pull Request.

## Branch Types and Conventions

Depending on the nature of the work, use the following format to name branches: `<type>/<app>/<name>`

The `<app>` segment must indicate the target scope of the change. Use the specific app name (`api`, `web`, `admin`, `ticketing`) or `nx` for general architecture/monorepo changes.

- **New Feature:**
  `feature/<app>/feature-name`
  _(Example: `feature/api/user-auth`)_

- **Bug or Error Fix:**
  `fix/<app>/bug-name`
  _(Example: `fix/web/checkout-crash`)_

- **Refactor or Structural Improvement:**
  `refactor/<app>/component-name`
  _(Note: for implementation improvements that don't change functional behavior)._

- **Maintenance or Technical Task:**
  `chore/<app>/task-name`
  _(Note: tasks without direct functional impact, e.g., `chore/nx/update-deps`)._

## General Rules

- **Lowercase**: The entire branch name must be entirely in lowercase.
- **Hyphens**: Separate words using hyphens (`-`).
- **Clarity**: Use short, concise, and descriptive names explaining the nature of the change.

## Delivery Process

1. Finish and implement your code on the branch following these naming validations.
2. Once the task is completed, ready, and approved, open a **Pull Request targeting the `main` branch**.
