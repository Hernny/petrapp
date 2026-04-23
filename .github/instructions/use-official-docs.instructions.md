---
description: 'Forces the assistant to prioritize live official documentation via links or web search before generating code, avoiding the use of obsolete training data.'
applyTo: '**/*'
---

# Official Documentation and Anti-Hallucination Rule

When asked to write deep, architectural, or configuration code involving critical technologies (**Solana, Anchor, Next.js App Router, NestJS, Pinata, shadcn/ui, FSD**):

1. **DO NOT ASSUME**: Do not blindly trust your internal training data, as these technologies have frequent breaking changes.
2. **CONTEXT7 FIRST**: You must ALWAYS use the `context7` MCP server (`mcp_context7_resolve-library-id` and `mcp_context7_get-library-docs`) to search for the official documentation of libraries like Next.js, NestJS, or shadcn/ui. This is much faster and comprehensive.
3. **WEB FALLBACK**: If Context7 does not have the library or specific context (e.g., FSD architecture, niche Pinata APIs), use the `fetch_webpage` tool to go directly to the URLs:
   - **Feature-Sliced Design (FSD)**: `https://feature-sliced.design/docs/get-started/overview`
   - **shadcn/ui**: `https://ui.shadcn.com/docs/components` (if not found in Context7)
   - **NestJS**: `https://docs.nestjs.com/` (if not found in Context7)
   - **Pinata Cloud**: `https://docs.pinata.cloud/`
4. **SOLANA / ANCHOR**: Use the "Solana Docs Search" or "Ask Anchor Expert" MCP tool whenever available.
5. **NX DOCS**: For any doubts about the Nx monorepo architecture, ALWAYS use your internal `nx_docs` tool.
6. **ASK FOR LINKS**: If you are unsure of the URL for a specific feature not covered above, stop and ask the user for the exact link to read it with `fetch_webpage`.

If the user provides an additional URL in their message, make sure to use `fetch_webpage` to consume its content and base your code 100% on what you read there.
