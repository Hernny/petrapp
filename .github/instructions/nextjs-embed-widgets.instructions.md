---
description: 'Rules for creating embeddable widget routes (iframes) in Next.js App Router for promoters.'
applyTo: '**/*iframe*'
---

# Next.js Iframe Embed Widgets Rules

When working on creating embeddable web widgets so that organizers/promoters can place PetrApp events on their sites using `<iframe>`:

1. **Content Security Policy (CSP)**: Remember that by default Next.js and modern browsers (via `X-Frame-Options` headers or the `frame-ancestors` CSP) block rendering pages in iframes to prevent Clickjacking attacks.
2. **Headers Configuration**: When structuring the App Router (e.g., `apps/web/src/app/embed/layout.tsx` or in `next.config.js`), make sure to omit the `X-Frame-Options: DENY` header and explicitly send a `Content-Security-Policy: frame-ancestors *;` policy for these `/embed/*` routes, so the promoter can inject it anywhere.
3. **Minimalist Design**: The iframe UI should not contain PetrApp's global headers/footers. Only the actionable content (the event card and purchase button). Use clean nested layouts.
4. **Cookie Management (Third-Party)**: Keep in mind that Safari and other browsers block third-party cookies in iframes. For the checkout flow inside the widget, use JWTs in URLs, short-lived tokens, or delegate back to the parent window (redirection) if it requires complex state or originating transactions on the Solana network.
