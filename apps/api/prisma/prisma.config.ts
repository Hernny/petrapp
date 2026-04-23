import { defineConfig } from '@prisma/config';
import { PrismaClient } from '@prisma/client';

export default defineConfig({
  earlyAccess: true,
  studio: {
    port: 5555,
  },
});
