import codspeedPlugin from "@codspeed/vitest-plugin";
import { defineConfig } from "vitest/config";

const plugins = process.env.CODSPEED ? [codspeedPlugin()] : [];

export default defineConfig({
	plugins,
});
