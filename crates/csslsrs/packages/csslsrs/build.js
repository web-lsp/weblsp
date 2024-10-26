import { cp, mkdir, rm } from "node:fs/promises";
await rm("./src/generated/package.json");
await mkdir("./dist/wasm", { recursive: true });
await cp("./src/generated", "./dist/generated", { recursive: true });
