import { cp, mkdir, rm } from "node:fs/promises";
await rm("./src/wasm/package.json");
await mkdir("./dist/wasm", { recursive: true });
await cp("./src/wasm", "./dist/wasm", { recursive: true });
