"use client";

import React, { useState } from "react";
import {
  Activity,
  Layers,
  Search,
  Cpu,
  GitCommit,
  ArrowRight,
  Database,
  Radio,
  CheckCircle2,
  Clock,
  ExternalLink,
  ShieldCheck,
  AlertTriangle,
} from "lucide-react";

export default function DashboardPage() {
  const [searchQuery, setSearchQuery] = useState("");
  const [activeTab, setActiveTab] = useState<"graph" | "flow" | "tx" | "utxos" | "mempool">("graph");

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    if (!searchQuery.trim()) return;
    // Will route to transaction or outpoint based on query pattern
  };

  return (
    <div className="flex flex-col min-h-screen bg-background text-gray-200">
      {/* Top Header */}
      <header className="border-b border-border bg-surface/80 backdrop-blur sticky top-0 z-50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between gap-4">
          <div className="flex items-center gap-3">
            <div className="w-9 h-9 rounded-lg bg-btc/10 border border-btc/30 flex items-center justify-center text-btc font-bold shadow-[0_0_15px_rgba(247,147,26,0.2)]">
              ₿
            </div>
            <div>
              <h1 className="text-base font-semibold tracking-tight text-white flex items-center gap-2">
                Bitcoin UTXO Visualizer
                <span className="text-[10px] font-mono uppercase px-2 py-0.5 rounded bg-btc/20 text-btc border border-btc/30 font-medium">
                  Regtest
                </span>
              </h1>
              <p className="text-xs text-gray-400 font-mono">
                Directed Acyclic Graph & Protocol Explorer
              </p>
            </div>
          </div>

          {/* Search bar */}
          <form onSubmit={handleSearch} className="flex-1 max-w-xl">
            <div className="relative">
              <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
              <input
                type="text"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                placeholder="Search TXID, Outpoint (txid:vout), Block Height, or Address..."
                className="w-full bg-card border border-border rounded-lg pl-9 pr-4 py-1.5 text-xs text-gray-100 placeholder-gray-500 focus:outline-none focus:border-btc/60 focus:ring-1 focus:ring-btc/40 font-mono transition-all"
              />
            </div>
          </form>

          {/* Node Health Badge */}
          <div className="flex items-center gap-3">
            <div className="flex items-center gap-2 px-3 py-1.5 rounded-lg bg-card border border-border text-xs">
              <span className="relative flex h-2 w-2">
                <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                <span className="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
              </span>
              <span className="text-gray-300 font-mono text-[11px]">Node Sync: OK</span>
            </div>
          </div>
        </div>
      </header>

      {/* Hero Metrics Row */}
      <div className="border-b border-border bg-card/40">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-3 grid grid-cols-2 md:grid-cols-5 gap-4">
          <div className="flex flex-col">
            <span className="text-[11px] text-gray-400 uppercase tracking-wider font-mono">
              Indexed Height
            </span>
            <span className="text-lg font-bold font-mono text-white flex items-center gap-1.5">
              <Layers className="w-4 h-4 text-btc" /> 101
            </span>
          </div>

          <div className="flex flex-col">
            <span className="text-[11px] text-gray-400 uppercase tracking-wider font-mono">
              Total Transactions
            </span>
            <span className="text-lg font-bold font-mono text-white flex items-center gap-1.5">
              <GitCommit className="w-4 h-4 text-cyan-400" /> 101
            </span>
          </div>

          <div className="flex flex-col">
            <span className="text-[11px] text-gray-400 uppercase tracking-wider font-mono">
              Active UTXOs
            </span>
            <span className="text-lg font-bold font-mono text-utxo-unspent flex items-center gap-1.5">
              <CheckCircle2 className="w-4 h-4" /> 101
            </span>
          </div>

          <div className="flex flex-col">
            <span className="text-[11px] text-gray-400 uppercase tracking-wider font-mono">
              Total UTXO Value
            </span>
            <span className="text-lg font-bold font-mono text-amber-300">
              5,050.00 <span className="text-xs text-gray-400 font-normal">BTC</span>
            </span>
          </div>

          <div className="flex flex-col">
            <span className="text-[11px] text-gray-400 uppercase tracking-wider font-mono">
              Mempool Unconfirmed
            </span>
            <span className="text-lg font-bold font-mono text-utxo-mempool flex items-center gap-1.5">
              <Radio className="w-4 h-4 animate-pulse" /> 0 txs
            </span>
          </div>
        </div>
      </div>

      {/* Main Tab Navigation */}
      <div className="border-b border-border bg-surface">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 flex items-center gap-2">
          {[
            { id: "graph", label: "UTXO Graph (DAG)", icon: Activity },
            { id: "flow", label: "Value Flow & Fees", icon: ArrowRight },
            { id: "tx", label: "Transaction Explorer", icon: GitCommit },
            { id: "utxos", label: "UTXO Set & Lifecycles", icon: Database },
            { id: "mempool", label: "Live Mempool Feed", icon: Radio },
          ].map((tab) => {
            const Icon = tab.icon;
            const active = activeTab === tab.id;
            return (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id as any)}
                className={`flex items-center gap-2 py-3 px-4 text-xs font-medium border-b-2 transition-colors ${
                  active
                    ? "border-btc text-white bg-card/60"
                    : "border-transparent text-gray-400 hover:text-gray-200 hover:bg-card/20"
                }`}
              >
                <Icon className={`w-4 h-4 ${active ? "text-btc" : "text-gray-400"}`} />
                {tab.label}
              </button>
            );
          })}
        </div>
      </div>

      {/* Main Content Area */}
      <main className="flex-1 max-w-7xl mx-auto w-full px-4 sm:px-6 lg:px-8 py-6">
        {/* Placeholder / Initial Canvas Container */}
        <div className="bg-surface border border-border rounded-xl p-6 relative overflow-hidden min-h-[500px] flex flex-col justify-between">
          <div className="flex items-center justify-between pb-4 border-b border-border">
            <div>
              <h2 className="text-base font-semibold text-white flex items-center gap-2">
                Directed Acyclic Graph Canvas
                <span className="text-[11px] font-mono px-2 py-0.5 rounded bg-emerald-950/60 border border-emerald-800/40 text-emerald-400">
                  Mode: Dual (Tx ↔ UTXO)
                </span>
              </h2>
              <p className="text-xs text-gray-400 mt-0.5">
                Explicit visualization: Transaction outputs form UTXO nodes, which are consumed by subsequent transaction inputs.
              </p>
            </div>
            <div className="flex items-center gap-2 text-xs font-mono">
              <span className="flex items-center gap-1 text-utxo-unspent">
                <span className="w-2.5 h-2.5 rounded-full bg-utxo-unspent inline-block" /> Unspent UTXO
              </span>
              <span className="flex items-center gap-1 text-utxo-spent ml-3">
                <span className="w-2.5 h-2.5 rounded-full bg-utxo-spent inline-block" /> Spent Output
              </span>
            </div>
          </div>

          {/* Interactive Graph Placeholder / Canvas State */}
          <div className="flex-1 flex flex-col items-center justify-center my-8 text-center">
            <div className="w-16 h-16 rounded-2xl bg-card border border-border flex items-center justify-center text-btc mb-4 shadow-xl">
              <Activity className="w-8 h-8" />
            </div>
            <h3 className="text-sm font-semibold text-white mb-1">
              Ready to Visualize Bitcoin UTXO Trees
            </h3>
            <p className="text-xs text-gray-400 max-w-md mb-6">
              Enter any Transaction ID or Outpoint (<code className="text-btc font-mono">TXID:VOUT</code>) in the search bar above to trace ancestor inputs, descendant outputs, and satoshi value movement.
            </p>
            <div className="flex items-center gap-3">
              <button
                onClick={() => setSearchQuery("coinbase:block-101")}
                className="px-3 py-1.5 rounded-lg bg-card border border-border hover:border-btc/40 text-xs font-mono text-gray-300 hover:text-white transition-all"
              >
                Inspect Block 101 Coinbase UTXO
              </button>
            </div>
          </div>

          {/* Educational Callout */}
          <div className="mt-4 p-4 rounded-lg bg-card/60 border border-border/80 flex items-start gap-3">
            <ShieldCheck className="w-5 h-5 text-btc flex-shrink-0 mt-0.5" />
            <div className="text-xs text-gray-300">
              <strong className="text-white font-medium">Protocol Principle: </strong>
              Transactions do not spend transactions. Transactions consume individual <em>Unspent Transaction Outputs</em> (UTXOs) referenced by outpoints (<code className="font-mono text-gray-100">TXID:VOUT</code>). The difference between total input satoshis and output satoshis is the mining fee, claimed in the coinbase transaction.
            </div>
          </div>
        </div>
      </main>

      {/* Footer */}
      <footer className="border-t border-border bg-surface py-4 text-center text-xs text-gray-500 font-mono">
        Bitcoin UTXO Visualizer · Open-Source Systems Tooling · Rust + PostgreSQL + Next.js
      </footer>
    </div>
  );
}
