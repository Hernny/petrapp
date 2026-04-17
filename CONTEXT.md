# context.md – PetrApp Copilot

This file defines the base context that your Copilot agent must know to help you develop and operate **PetrApp**, an event ticketing platform with Web2 + Web3 (Solana) support.

## Product Description

- **Project Name**: PetrApp
- **Domain**: SaaS Ticketing / Events
- **Objective**: A SaaS platform for selling tickets to various events (concerts, theater plays, conferences, standup comedy, sports games, etc.).
- **Model & Core Business Flow**:
  - **1. Discovery & Selection**: An `Attendee` logs in (Web2/Web3 via Dynamic), explores events (geolocated feed or global promoted carousel), and selects tickets up to a per-user limit.
  - **2. Direct Payment**: The Attendee pays the `Creator` directly via Crypto (USDT/USDC routed to the Creator's accumulator wallet) or Fiat (Pago Móvil, Zelle, Cash, POS, Stripe).
  - **3. Minting Execution**:
    - _Crypto Payments_: Tickets (Solana NFTs) are minted immediately and automatically upon chain confirmation.
    - _Fiat/Offline Payments_: Tickets require manual approval in the `Creator`'s dashboard before minting.
  - **4. Monetization (PetrApp Balance/Credits)**: Upon minting either payment type, PetrApp deducts its platform commission and `Affiliate` payouts from the `Creator`'s pre-funded "PetrApp Balance".
  - **5. Network Fees Subsidized**: To accelerate Creator adoption, PetrApp covers all Solana network/minting transaction fees (gas/SOL) centrally.
  - **6. Balance Management**: Creators can top up their PetrApp Balance with crypto or credit cards (via Dynamic). If credits run out, Creators must either enable "Auto-recharge" or the system will freeze ticket minting for their events.
  - **7. B2B Embed Widgets**: Promoters can embed their PetrApp events into their own external websites or e-commerce platforms using an iframe.
  - **Authentication (Users & Creators)**: Web2 and Web3 hybrid authentication using **Dynamic (dynamic.xyz)** SDK. Creators only need to configure an accumulator wallet for crypto revenue (no SOL fee-payer wallet needed due to subsidies).

## Technology Stack (Summary)

- **Infrastructure**: Docker, Nx (monorepo)
- **Databases**: PostgreSQL, Redis
- **Backend**: NestJS (TypeScript), Anchor (Rust/Solana)
- **Frontend**: Next.js (TypeScript), Tailwind CSS

## Infra / Orchestration

- Monorepo using Nx.
- Apps:
  - `api`: NestJS backend (TypeScript).
  - `web`: Next.js frontend (TypeScript).
  - `admin`: Next.js frontend (TypeScript).
  - `ticketing`: Anchor programs (Rust/Solana).
- Libs:
  - `libs/shared-*`: shared TS types and utilities.
  - `libs/anchor-client`: IDL + generated TS types from Anchor.
- Docker for development and production (local docker-compose for Postgres + Redis).

## Databases / Cache

- PostgreSQL as the primary database (users, events, orders, tickets, transfers).
- Redis for:
  - Caching (events/tickets).
  - Rate limiting and sessions.
  - Queues (BullMQ) for async tasks (emails, minting).

## Backend (api – NestJS)

- Modular REST API:
  - Auth (JWT), Users, Wallets, Events, Orders, Tickets, Payments, Admin, Blockchain.
- ORM: Prisma over Postgres.
- Solana Integration: `@solana/web3.js` + `libs/anchor-client`.
- Security: data validation using DTOs and class-validator, access control using guards and roles, CORS configuration, and secrets management using environment variables in development, GitHub Actions Secrets for CI/CD, and AWS secrets/environment variable services in production.

## Frontend (web and admin – Next.js)

- Next.js (App Router) using TypeScript.
- Styling and UI components:
  - **Tailwind CSS** as the utility-first CSS framework.
  - **shadcn/ui** as the base component library (buttons, tables, layouts, dashboard) customized for PetrApp.
- State and data fetching:
  - **TanStack Query** for fetching/caching data from Nest/Anchor (events, tickets, orders, etc.).
  - **Zustand** for global UI state (client-side auth, filters, cart/checkout, UI flags).
- web:
  - Public: catalog, event details, purchase.
  - User: my tickets, QR access.
- admin:
  - Organizer: Event CRUD, basic metrics.
  - Admin: Organizer CRUD, advanced metrics (shadcn/ui style dashboard).
- Integrations:
  - Nest API.
  - Stripe (Hosted Checkout).
  - **Dynamic SDK** for hybrid Web2 + Web3 authentication and wallet connectivity (Solana + EVM).

## Blockchain (anchor – Solana)

- Main Anchor Program:
  - `create_event`, `mint_ticket`, `transfer_ticket`, `validate_entry`.
- Flow:
  - Backend/Frontend use `libs/anchor-client` (IDL + types) to interact with the program.
  - Tickets are NFTs/tokens on Solana.

## Development Rules

- Preferably use official CLIs (Nest, Nx, Next, Anchor) to generate resources (modules, components, configs).
- Features should be simple and concrete (narrow scope per branch).
- Create a feature branch for every new feature.
- Follow the branching-flow skill rules.
- Write the necessary tests for each feature and ensure they pass.
- Open a PR to the `main` branch once tests are approved.

## User Roles / Actors

The system is structured around four primary actors with distinct permissions, access levels, and features:

- **Attendee (Cliente Final)**: The end-user who creates an account to discover events, purchase tickets (NFTs), view their collected tickets, and access events via QR codes. They interact primarily with the public frontend (`apps/web`).
- **Creator (Promotor de Eventos)**: The event organizer who uses the platform SaaS tools to create and manage events (Simple or Promoted), issue complimentary tickets, generate iframe widgets, track analytics, and validate tickets at the door using the scanner app. They interact primarily with the organizer dashboard (`apps/admin`).
- **Affiliate (Afiliado / Revendedor)**: External promoters, influencers, or resellers who partner with PetrApp or specific Creators. They generate trackable discount codes or referral links to promote events and earn commissions for successful ticket sales. They have a specialized portal to track their referrals and commission payouts.
- **Platform Admin (PetrApp Admin)**: The overarching super-administrator (your company). Manages the SaaS platform globally. Responsible for onboarding/managing Creators, handling global reports, configuration of fees, platform subscription management, and overseeing the entire ecosystem's health.
