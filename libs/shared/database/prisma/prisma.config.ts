import { defineConfig } from '@prisma/config';

export default defineConfig({
  // En lugar de usar el objeto { kind: 'folder' },
  // apuntamos directamente a la carpeta si tu versión lo permite,
  // o usamos la ruta al archivo principal.
  schema: './schema/main.prisma',
  datasource: {
    url: process.env['DATABASE_URL'],
  },
});
