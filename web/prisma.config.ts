import "dotenv/config"
import { defineConfig } from "prisma/config"
import path from "path"

// Honor DATABASE_URL when set (staging/production should always set this) and
// only fall back to a local SQLite file for zero-config local development.
// NOTE: schema.prisma currently declares `provider = "sqlite"`, so DATABASE_URL
// must point at a `file:` URL until the schema/migrations are ported to
// postgresql (the @prisma/adapter-pg dependency suggests that's the intended
// end state, but the migrations directory is still SQLite-specific and hasn't
// been regenerated for Postgres).
const databaseUrl =
  process.env.DATABASE_URL ?? `file:${path.join(process.cwd(), 'prisma', 'dev.db')}`

export default defineConfig({
  schema: "prisma/schema.prisma",
  migrations: {
    path: "prisma/migrations",
  },
  datasource: {
    url: databaseUrl,
  },
})
