import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Bitcoin UTXO Visualizer — Serious Protocol & Graph Explorer",
  description:
    "An open-source, production-grade developer tool to deeply visualize and analyze Bitcoin transactions, outpoints, scripts, and UTXO lifecycles.",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className="dark">
      <body className="bg-background text-gray-100 flex flex-col min-h-screen">
        {children}
      </body>
    </html>
  );
}
