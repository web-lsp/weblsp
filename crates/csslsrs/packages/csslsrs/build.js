import { cp, mkdir } from "node:fs/promises";
await mkdir("./dist/wasm", { recursive: true });
await cp("./src/wasm", "./dist/wasm", { recursive: true });
