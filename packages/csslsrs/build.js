import { cp, mkdir, rm } from "node:fs/promises"
await rm(new URL("./src/generated/package.json", import.meta.url).pathname)
await mkdir(new URL("./dist/generated", import.meta.url).pathname, {
  recursive: true,
})
await cp(
  new URL("./src/generated", import.meta.url).pathname,
  new URL("./dist/generated", import.meta.url).pathname,
  { recursive: true }
)
