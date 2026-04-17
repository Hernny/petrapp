---
description: 'Forces the assistant to prioritize live official documentation via links or web search before generating code, avoiding the use of obsolete training data.'
applyTo: '**/*'
---

# Official Documentation and Anti-Hallucination Rule

When asked to write deep, architectural, or configuration code involving critical technologies (**Solana, Anchor, Next.js App Router, NestJS, Pinata, shadcn/ui, FSD**):

1. **DO NOT ASSUME**: Do not blindly trust your internal training data, as these technologies have frequent breaking changes.
2. **LIVE VERIFICATION**: You must ALWAYS use the `fetch_webpage` tool to go directly to the official documentation to solve a problem:
   - **Feature-Sliced Design (FSD)**: `https://feature-sliced.design/docs/get-started/overview` (to see architecture and frontend folder conventions).
   - **shadcn/ui**: `https://ui.shadcn.com/docs/components` (to check components and commands like `npx shadcn@latest add ...`).
   - **NestJS**: `https://docs.nestjs.com/` (for framework-specific references, guards, pipes, controllers).
   - **Pinata Cloud**: `https://docs.pinata.cloud/` (if you need to know how to interact with its API, although the external MCP server will facilitate many direct processes with tools).
   - **Solana / Anchor**: Use the "Solana Docs Search" or "Ask Anchor Expert" MCP tool whenever available.
3. **ASK FOR LINKS**: If you are unsure of the URL for a specific feature, stop and ask the user for the exact link to the documentation page to read it with `fetch_webpage`.
4. **NX DOCS**: For any doubts about the Nx monorepo architecture, ALWAYS use your internal `nx_docs` tool.

If the user provides an additional URL in their message, make sure to use `fetch_webpage` to consume its content and base your code 100% on what you read there.
