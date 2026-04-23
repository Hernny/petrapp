-- ==========================================================
-- 1. IDENTIDAD Y ACCESO (IAM) - ADAPTADO A DYNAMIC.XYZ
-- ==========================================================

CREATE TABLE public.user_types (
id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
name text NOT NULL UNIQUE, -- 'platform_admin', 'promoter', 'affiliate', 'attendee'
description text,
created_at timestamptz DEFAULT now()
);

CREATE TABLE public.users (
id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
user_type_id uuid NOT NULL REFERENCES public.user_types(id),
dynamic_id text UNIQUE, -- ID único de Dynamic.xyz
username text UNIQUE,
email text UNIQUE,
created_at timestamptz DEFAULT now(),
updated_at timestamptz DEFAULT now(),
deleted_at timestamptz
);

CREATE TABLE public.user_wallets (
id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
user_id uuid NOT NULL REFERENCES public.users(id) ON DELETE CASCADE,
wallet_address text NOT NULL, -- Public Key de Solana o EVM
wallet_type text NOT NULL DEFAULT 'solana',
is_primary boolean DEFAULT false,
connected_at timestamptz DEFAULT now(),
UNIQUE (wallet_address)
);

CREATE TABLE public.roles (
id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
user_type_id uuid NOT NULL REFERENCES public.user_types(id),
name text NOT NULL UNIQUE,
description text,
deleted_at timestamptz
);

CREATE TABLE public.user_roles (
user_id uuid NOT NULL REFERENCES public.users(id) ON DELETE CASCADE,
role_id uuid NOT NULL REFERENCES public.roles(id) ON DELETE CASCADE,
assigned_at timestamptz DEFAULT now(),
PRIMARY KEY (user_id, role_id)
);

-- ==========================================================
-- 2. INFRAESTRUCTURA DE EVENTOS (VENUES Y CATEGORÍAS)
-- ==========================================================

CREATE TABLE public.venues (
id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
name text NOT NULL,
address text NOT NULL,
city text NOT NULL,
country text NOT NULL,
capacity integer,
coordinates point, -- Lat/Long para mapas
timezone text NOT NULL DEFAULT 'UTC',
created_at timestamptz DEFAULT now()
);

CREATE TABLE public.seats (
id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
venue_id uuid NOT NULL REFERENCES public.venues(id) ON DELETE CASCADE,
section_name text NOT NULL,
row_name text NOT NULL,
seat_number text NOT NULL,
is_accessible boolean DEFAULT false,
UNIQUE(venue_id, section_name, row_name, seat_number)
);

CREATE TABLE public.categories (
id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
name text NOT NULL UNIQUE,
slug text NOT NULL UNIQUE,
icon_url text
);

-- ==========================================================
-- 3. NEGOCIO: EVENTOS Y SESIONES
-- ==========================================================

-- Planes de facturación (Cobro por ticket en créditos según tipo de evento y método de pago)
CREATE TABLE public.billing_plans (
id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
name text NOT NULL UNIQUE, -- 'Free', 'Pro', 'Enterprise'
event_creation_credits numeric(10,2) NOT NULL DEFAULT 10.00, -- Costo por crear el evento en créditos
base_ticket_credits numeric(10,2) NOT NULL DEFAULT 1.00, -- Costo fijo en créditos por ticket minteado
platform_fee_percentage numeric(5,2) NOT NULL DEFAULT 5.00, -- % sobre el valor del ticket (ej: 5.00)
description text,
created_at timestamptz DEFAULT now()
);

CREATE TABLE public.promoters (
id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
name text NOT NULL UNIQUE,
description text,
billing_plan_id uuid REFERENCES public.billing_plans(id), -- Asociar al promotor su plan de costos
created_at timestamptz DEFAULT now(),
updated_at timestamptz DEFAULT now(),
deleted_at timestamptz
);

CREATE TABLE public.user_promoters (
id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
user_id uuid NOT NULL REFERENCES public.users(id),
promoter_id uuid NOT NULL REFERENCES public.promoters(id),
created_at timestamptz DEFAULT now(),
deleted_at timestamptz
);

CREATE TABLE public.events (
id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
user_promoter_id uuid REFERENCES public.user_promoters(id),
venue_id uuid REFERENCES public.venues(id),
name text NOT NULL,
description text,
-- Web3 Metaplex
collection_mint text UNIQUE, -- Address de la Colección en Solana
metadata_uri text, -- IPFS CID de Pinata (JSON global)
created_at timestamptz DEFAULT now(),
updated_at timestamptz DEFAULT now(),
deleted_at timestamptz
);

CREATE TABLE public.event_categories (
event_id uuid REFERENCES public.events(id) ON DELETE CASCADE,
category_id uuid REFERENCES public.categories(id) ON DELETE CASCADE,
PRIMARY KEY (event_id, category_id)
);

CREATE TABLE public.event_sessions (
id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
event_id uuid NOT NULL REFERENCES public.events(id) ON DELETE CASCADE,
name text, -- Ej: 'Función Sábado Matiné'
start_time timestamptz NOT NULL,
end_time timestamptz NOT NULL,
created_at timestamptz DEFAULT now()
);

-- ==========================================================
-- 4. TICKETS, PRECIOS Y AFILIADOS
-- ==========================================================

CREATE TABLE public.ticket_tiers (
id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
event_id uuid NOT NULL REFERENCES public.events(id) ON DELETE CASCADE,
name text NOT NULL, -- 'VIP', 'General'
price numeric(10,2) NOT NULL,
currency text NOT NULL DEFAULT 'usd',
max_capacity integer NOT NULL,
metadata_uri text, -- IPFS CID específico para el arte de este Tier
created_at timestamptz DEFAULT now()
);

CREATE TABLE public.affiliate_codes (
id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
event_id uuid NOT NULL REFERENCES public.events(id),
affiliate_id uuid NOT NULL REFERENCES public.users(id),
code text NOT NULL UNIQUE,
discount_percentage numeric(5,2) NOT NULL,
created_at timestamptz DEFAULT now()
);

CREATE TABLE public.tickets (
id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
session_id uuid NOT NULL REFERENCES public.event_sessions(id),
tier_id uuid NOT NULL REFERENCES public.ticket_tiers(id),
attendee_id uuid NOT NULL REFERENCES public.users(id),
seat_id uuid REFERENCES public.seats(id), -- Nullable para General Admission
affiliate_code_id uuid REFERENCES public.affiliate_codes(id),
-- Web3 Minting
mint_address text UNIQUE, -- Solana Mint Address
status text NOT NULL CHECK (status IN ('pending_payment', 'minting', 'minted', 'used', 'cancelled')),
purchase_time timestamptz DEFAULT now(),
created_at timestamptz DEFAULT now(),
deleted_at timestamptz
);

-- ==========================================================
-- 5. FINANZAS Y TRANSACCIONES
-- ==========================================================

CREATE TABLE public.transactions (
id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
user_id uuid NOT NULL REFERENCES public.users(id),
amount numeric(10,2) NOT NULL, -- Total pagado por el usuario
platform_fee_amount numeric(10,2) DEFAULT 0.00, -- Monto deducido directamente del pago (comisión PetrApp en dinero)
promoter_amount numeric(10,2) DEFAULT 0.00, -- Monto a liquidar/entregar al promotor (amount - platform_fee_amount)
currency text NOT NULL CHECK (currency IN ('usd', 'usdt', 'usdc', 'sol')),
payment_method text NOT NULL, -- 'petrapp_gateway', 'stripe', 'external_pos', 'bank_transfer', 'cash', etc.
-- Web3 Trazabilidad
tx_signature text UNIQUE, -- Solana Signature
sender_wallet text,
status text DEFAULT 'pending',
transaction_time timestamptz DEFAULT now()
);

-- ==========================================================
-- 6. BALANCES, CRÉDITOS Y COMISIONES (LEDGER)
-- ==========================================================

-- Historial de balance/créditos del Promotor (Recargas, deducciones por crear eventos, y cobro de tickets según método)
CREATE TABLE public.promoter_ledgers (
id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
promoter_id uuid NOT NULL REFERENCES public.promoters(id),
transaction_id uuid REFERENCES public.transactions(id), -- Asociado si fue una recarga externa (topup)
ticket_id uuid REFERENCES public.tickets(id), -- Boleto que desencadenó el cobro (si aplica)
event_id uuid REFERENCES public.events(id), -- Evento si el cobro fue 'event_creation_fee'
amount numeric(10,2) NOT NULL, -- Positivo: recarga, Negativo: cobro (en créditos)
currency text NOT NULL DEFAULT 'credits', -- Sistema basado en "créditos" internos (1 crédito = 1 USD)
entry_type text NOT NULL CHECK (entry_type IN ('topup', 'event_creation_fee', 'ticket_internal_payment_fee', 'ticket_external_payment_fee', 'affiliate_payout', 'refund')),
description text,
created_at timestamptz DEFAULT now()
);

-- Registro de gastos operativos de la Plataforma (Ej: Subsidio de Network Fees / Gastos de gas Solana)
CREATE TABLE public.platform_expenses (
id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
ticket_id uuid REFERENCES public.tickets(id), -- Ticket por el que se está cubriendo el costo
tx_signature text UNIQUE, -- Solana tx signature de donde salió el fee
network_fee_sol numeric(18,9) NOT NULL, -- El fee en lamports o SOL (generalmente ~0.000005 a 0.01 dependiendo del mint)
description text,
created_at timestamptz DEFAULT now()
);

-- Registro de comisiones ganadas y pagadas a los Afiliados
CREATE TABLE public.affiliate_commissions (
id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
affiliate_id uuid NOT NULL REFERENCES public.users(id),
ticket_id uuid NOT NULL REFERENCES public.tickets(id), -- Ticket que generó la venta
amount numeric(10,2) NOT NULL,
currency text NOT NULL DEFAULT 'usd',
status text NOT NULL CHECK (status IN ('pending', 'paid', 'cancelled')),
payout_transaction_id uuid REFERENCES public.transactions(id), -- Transacción de liquidación hacia la wallet del afiliado
created_at timestamptz DEFAULT now(),
paid_at timestamptz
);

-- Índices optimizados para búsquedas frecuentes
CREATE INDEX idx_tickets_session ON public.tickets(session_id);
CREATE INDEX idx_tickets_attendee ON public.tickets(attendee_id);
CREATE INDEX idx_sessions_start ON public.event_sessions(start_time);
CREATE INDEX idx_users_dynamic ON public.users(dynamic_id);
CREATE INDEX idx_wallets_address ON public.user_wallets(wallet_address);
