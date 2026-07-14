import { PrismaClient } from '@prisma/client'
import { PrismaBetterSqlite3 } from '@prisma/adapter-better-sqlite3'
import path from 'path'

// Since Prisma 7, PrismaClient must be constructed with an explicit driver
// adapter (there's no more "resolve automatically from schema.prisma" magic).
// schema.prisma currently declares `provider = "sqlite"`, so this must be a
// SQLite-compatible adapter, not @prisma/adapter-pg (which is installed as a
// dependency but unused today - see prisma.config.ts for the note on what a
// real Postgres migration would require: changing the schema provider and
// regenerating migrations, not just swapping the adapter here).
function sqliteFilePath(): string {
  const url = process.env.DATABASE_URL ?? `file:${path.join(process.cwd(), 'prisma', 'dev.db')}`
  return url.startsWith('file:') ? url.slice('file:'.length) : url
}

const prismaClientSingleton = () => {
  const adapter = new PrismaBetterSqlite3({ url: sqliteFilePath() })
  return new PrismaClient({ adapter })
}

declare global {
  var prisma: undefined | ReturnType<typeof prismaClientSingleton>
}

const prisma = globalThis.prisma ?? prismaClientSingleton()

if (process.env.NODE_ENV !== 'production') globalThis.prisma = prisma

export default prisma
