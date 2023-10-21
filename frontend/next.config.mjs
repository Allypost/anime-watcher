/**
 * Run `build` or `dev` with `SKIP_ENV_VALIDATION` to skip env validation. This is especially useful
 * for Docker builds.
 */
await import("./src/env.mjs");

/** @type {import("next").NextConfig} */
const config = {
  poweredByHeader: false,
  reactStrictMode: true,
  devIndicators: {
    buildActivity: true,
    buildActivityPosition: "bottom-right",
  },
  output: "standalone",
  experimental: {
    adjustFontFallbacks: true,
    adjustFontFallbacksWithSizeAdjust: true,
    serverActions: true,
    typedRoutes: true,
  },
};

export default config;
