# PetrApp Project Guidelines

## Code Style

- Use official CLIs (Nest, Nx, Next, Anchor) to generate resources (modules, components, configs).
- Keep features simple with a narrow scope per branch.
- Next.js: Use App Router (`app/` directory), Tailwind CSS, and `shadcn/ui` for components.
- State: Use TanStack Query for data fetching/caching and Zustand for global UI state.

## Architecture

PetrApp is an event ticketing platform with Web2 + Web3 (Solana) support.

- **Monorepo**: Managed by Nx.
- **Backend (`apps/api`)**: NestJS REST API using Prisma (PostgreSQL) and Redis for BullMQ queues/caching. When generating Prisma models, **always** refer to the exact structure and relationships in [tentative-db.md](../tentative-db.md).
- **Frontend (`apps/web`, `apps/admin`)**: Next.js frontend apps for public/attendee ticketing and the organizer/admin dashboard.
- **Blockchain (`apps/ticketing`)**: Solana Anchor program containing events and NFT ticket mechanisms. Uses `libs/anchor-client` for frontend-backend interaction.
- See [CONTEXT.md](CONTEXT.md) for complete architecture details and actor permissions.

## Build and Test

- Build tasks: Run `npx nx build <app-name>` (e.g., `web`, `api`, `admin`, `ticketing`).
- Infrastructure: Use local Docker Compose for Postgres + Redis.
- Branches: Follow the branching-flow skill rules before opening Pull Requests.
- Tests: Write necessary tests for each feature and ensure they pass before opening a PR to `main`.

## Conventions

- **Authentication**: Uses Dynamic (dynamic.xyz) SDK for hybrid Web2 + Web3 auth.
- **Model Context Protocol (MCP)**: Copilot MCP servers (Solana, Dynamic, Pinata) and their environment variables/settings are explicitly configured and managed inside [.vscode/settings.json](../.vscode/settings.json).
- **Anti-Hallucination**: When writing deep configuration/code for Solana, Anchor, Next.js App Router, or NestJS, prioritize fetching official documentation before generating code.
