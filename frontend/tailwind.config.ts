import type { Config } from "tailwindcss";

const config: Config = {
  darkMode: ["class"],
  content: [
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./features/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        background: "#0a0c10",
        surface: "#11141c",
        card: "#171c26",
        border: "#252d3d",
        btc: {
          DEFAULT: "#f7931a",
          hover: "#e08212",
          dim: "#422806",
        },
        utxo: {
          unspent: "#10b981", // bright green
          spent: "#64748b",   // slate / consumed
          mempool: "#f59e0b", // amber unconfirmed
          coinbase: "#fbbf24", // gold
        },
      },
      fontFamily: {
        mono: [
          "JetBrains Mono",
          "ui-monospace",
          "SFMono-Regular",
          "Menlo",
          "Monaco",
          "Consolas",
          "monospace",
        ],
        sans: [
          "Inter",
          "-apple-system",
          "BlinkMacSystemFont",
          "Segoe UI",
          "Roboto",
          "sans-serif",
        ],
      },
    },
  },
  plugins: [],
};
export default config;
