# context.md – PetrApp Copilot

Este archivo define el contexto base que tu agente Copilot debe conocer para ayudarte a desarrollar y operar **PetrApp**, una plataforma de gestión y venta de tickets de eventos con soporte Web2 + Web3 (Solana).

## Descripción del producto

- **Nombre del proyecto**: PetrApp  
- **Dominio**: Ticketing / Eventos  
- **Modelo**:
  - Venta de tickets para eventos (fiat y crypto).
  - Emisión de tickets como activos en Solana (NFT).
  - Validación de acceso mediante QR y verificación on-chain.
  - Panel para organizadores (gestión de eventos, tickets, ventas, reembolsos).

## Stack tecnológico (resumen)

- **Infraestructura**: Docker, Nx (monorepo)
- **Bases de datos**: PostgreSQL, Redis
- **Backend**: NestJS (TypeScript), Anchor (Rust/Solana)
- **Frontend**: Next.js (TypeScript), Tailwind CSS


## Infra / Orquestación

- Monorepo con Nx.
- Apps:
  - `api`: backend NestJS (TypeScript).
  - `web`: frontend Next.js (TypeScript).
  - `admin`: frontend Next.js (TypeScript).
  - `blockchain`: programas Anchor (Rust/Solana).
- Libs:
  - `libs/shared-*`: tipos y utilidades TS compartidas.
  - `libs/anchor-client`: IDL + tipos TS generados de Anchor.
- Docker para desarrollo y producción (docker-compose local para Postgres + Redis).

## Bases de datos / Cache

- PostgreSQL como base principal (users, events, orders, tickets, transfers).
- Redis para:
  - Caché (eventos/tickets).
  - Rate limiting y sesiones.
  - Colas (BullMQ) para tareas async (emails, minting).

## Backend (api – NestJS)

- API REST modular:
  - Auth (JWT), Users, Wallets, Events, Orders, Tickets, Payments, Admin, Blockchain.
- ORM: Prisma sobre Postgres.
- Integración Solana: `@solana/web3.js` + `libs/anchor-client`.
- Seguridad: validación de datos con DTOs y class-validator, control de acceso con guards y roles, configuración de CORS y gestión de secretos con variables de entorno en desarrollo, GitHub Actions Secrets para CI/CD y servicios de secretos/variables de entorno en AWS en producción.

## Frontend (web y admin – Next.js)

- Next.js (App Router) con TypeScript.
- Estilos y componentes:
  - **Tailwind CSS** como sistema de estilos utilitario.
  - **shadcn/ui** como librería base de componentes (botones, tablas, layouts, dashboard) personalizada para PetrApp.
- Estado y data:
  - **TanStack Query** para fetch/caché de datos desde Nest/Anchor (eventos, tickets, órdenes, etc.).
  - **Zustand** para estado global de UI (auth client-side, filtros, carrito/checkout, flags de UI).
- web:
  - Público: catálogo, detalle de evento, compra.
  - Usuario: mis tickets, QR.
- admin:
  - Organizador: CRUD de eventos, métricas básicas.
  - Administrador: CRUD de organizadores, métricas avanzadas (dashboard tipo shadcn/ui).
- Integraciones:
  - API Nest.
  - Stripe (Hosted Checkout).
  - Wallets Solana: `@solana/wallet-adapter-*`.

## Blockchain (anchor – Solana)

- Programa Anchor principal:
  - `create_event`, `mint_ticket`, `transfer_ticket`, `validate_entry`.
- Flujo:
  - Backend/Frontend usan `libs/anchor-client` (IDL + tipos) para hablar con el programa.
  - Tickets como NFTs/tokens en Solana.

## Reglas de desarrollo

- Utilizar preferentemente las CLI oficiales (Nest, Nx, Next, Anchor) para generar recursos (módulos, componentes, configs).
- Features simples y concretas (alcance acotado por rama).
- Crear una rama por cada nuevo feature.
- Generar los tests necesarios para cada feature y verificar que se aprueben.
- Crear PR a la rama `main` una vez aprobados los tests.
