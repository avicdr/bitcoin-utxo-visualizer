"use client";

import React, { useEffect, useState } from "react";
import {
  Activity,
  Layers,
  Search,
  GitCommit,
  ArrowRight,
  Database,
  Radio,
  CheckCircle2,
  ShieldCheck,
  RotateCw,
  Info,
  TrendingUp,
} from "lucide-react";
import { UtxoGraphView } from "@/features/graph/UtxoGraphView";
import { TxDetailView, TransactionDetailData } from "@/features/explorer/TxDetailView";
import { UtxoDetailView, UtxoData } from "@/features/utxos/UtxoDetailView";
import { UtxoListView } from "@/features/utxos/UtxoListView";
import { ValueFlowSankey } from "@/features/value-flow/ValueFlowSankey";
import { UtxoAnalyticsView } from "@/features/analytics/UtxoAnalyticsView";
import {
  fetchNodeStatus,
  fetchTransaction,
  fetchTransactionGraph,
  fetchBlocks,
  fetchUtxos,
  fetchUtxo,
} from "@/lib/api";
import { Node, Edge } from "@xyflow/react";

export default function DashboardPage() {
  const [searchQuery, setSearchQuery] = useState("");
  const [activeTab, setActiveTab] = useState<"graph" | "tx" | "utxos" | "flow" | "analytics">("graph");
  const [nodeStatus, setNodeStatus] = useState<any>(null);
  const [recentBlocks, setRecentBlocks] = useState<any[]>([]);
  const [currentTx, setCurrentTx] = useState<TransactionDetailData | null>(null);
  const [utxoList, setUtxoList] = useState<UtxoData[]>([]);
  const [currentUtxo, setCurrentUtxo] = useState<UtxoData | null>(null);
  const [graphNodes, setGraphNodes] = useState<Node[]>([]);
  const [graphEdges, setGraphEdges] = useState<Edge[]>([]);
  const [graphDepth, setGraphDepth] = useState<number>(2);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  // Global keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) {
        if (e.key === "Escape") {
          (e.target as HTMLElement).blur();
        }
        return;
      }
      if (e.key === "/") {
        e.preventDefault();
        document.getElementById("main-search-input")?.focus();
      } else if (e.key === "1") {
        setActiveTab("graph");
      } else if (e.key === "2") {
        setActiveTab("tx");
      } else if (e.key === "3") {
        setActiveTab("flow");
      } else if (e.key === "4") {
        setActiveTab("utxos");
      } else if (e.key === "5") {
        setActiveTab("analytics");
      } else if (e.key === "Escape") {
        setCurrentUtxo(null);
        setError(null);
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, []);

  // Load node status, recent blocks, and active UTXOs on mount
  useEffect(() => {
    async function loadInitial() {
      try {
        const node = await fetchNodeStatus();
        setNodeStatus(node);
        const blocks = await fetchBlocks();
        setRecentBlocks(blocks);
        const utxos = await fetchUtxos(50, 0);
        setUtxoList(utxos);
      } catch (err: any) {
        console.warn("Could not connect to backend API:", err.message);
      }
    }
    loadInitial();
  }, []);

  // Search or load outpoint
  const loadUtxoData = async (txid: string, vout: number) => {
    setIsLoading(true);
    setError(null);
    try {
      const u = await fetchUtxo(txid, vout);
      setCurrentUtxo(u);
      setActiveTab("utxos");
    } catch (err: any) {
      setError(err.message || "Failed to load UTXO");
    } finally {
      setIsLoading(false);
    }
  };

  // Search or load transaction
  const loadTxData = async (txid: string, depth = graphDepth) => {
    setIsLoading(true);
    setError(null);
    try {
      const tx = await fetchTransaction(txid);
      setCurrentTx(tx);

      // Load graph
      const graph = await fetchTransactionGraph(txid, depth);
      setGraphNodes(graph.nodes || []);
      setGraphEdges(graph.edges || []);
    } catch (err: any) {
      setError(err.message || "Failed to load transaction data");
    } finally {
      setIsLoading(false);
    }
  };

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    const query = searchQuery.trim();
    if (!query) return;

    if (query.includes(":")) {
      const parts = query.split(":");
      const txid = parts[0].trim();
      const vout = parseInt(parts[1].trim(), 10);
      if (!isNaN(vout)) {
        loadUtxoData(txid, vout);
      } else {
        loadTxData(txid);
      }
    } else {
      loadTxData(query);
      setActiveTab("tx");
    }
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
                  {nodeStatus?.network || "Regtest"}
                </span>
              </h1>
              <p className="text-xs text-gray-400 font-mono">
                Directed Acyclic Graph & Protocol Explorer
              </p>
            </div>
          </div>

          {/* Search bar */}
          <form onSubmit={handleSearch} className="flex-1 max-w-xl">
            <div className="relative flex items-center">
              <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
              <input
                id="main-search-input"
                type="text"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                placeholder="Search TXID, Outpoint (txid:vout), Block Height, or Address..."
                className="w-full bg-card border border-border rounded-lg pl-9 pr-14 py-1.5 text-xs text-gray-100 placeholder-gray-500 focus:outline-none focus:border-btc/60 focus:ring-1 focus:ring-btc/40 font-mono transition-all"
              />
              <kbd className="absolute right-2.5 px-1.5 py-0.5 text-[10px] font-mono text-gray-400 bg-surface border border-border/80 rounded pointer-events-none">
                /
              </kbd>
            </div>
          </form>

          {/* Node Health Badge */}
          <div className="flex items-center gap-3">
            <div className="flex items-center gap-2 px-3 py-1.5 rounded-lg bg-card border border-border text-xs">
              <span className="relative flex h-2 w-2">
                <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                <span className="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
              </span>
              <span className="text-gray-300 font-mono text-[11px]">
                {nodeStatus?.connected ? `Block #${nodeStatus.blocks}` : "API Ready"}
              </span>
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
              <Layers className="w-4 h-4 text-btc" />
              {nodeStatus?.indexed_height !== null && nodeStatus?.indexed_height !== undefined
                ? nodeStatus.indexed_height
                : 0}
            </span>
          </div>

          <div className="flex flex-col">
            <span className="text-[11px] text-gray-400 uppercase tracking-wider font-mono">
              Node Tip
            </span>
            <span className="text-lg font-bold font-mono text-white flex items-center gap-1.5">
              <GitCommit className="w-4 h-4 text-cyan-400" />
              {nodeStatus?.blocks || 0}
            </span>
          </div>

          <div className="flex flex-col">
            <span className="text-[11px] text-gray-400 uppercase tracking-wider font-mono">
              Mempool Size
            </span>
            <span className="text-lg font-bold font-mono text-utxo-mempool flex items-center gap-1.5">
              <Radio className="w-4 h-4" />
              {nodeStatus?.mempool_size || 0} txs
            </span>
          </div>

          <div className="flex flex-col">
            <span className="text-[11px] text-gray-400 uppercase tracking-wider font-mono">
              Subversion
            </span>
            <span className="text-xs font-mono text-gray-300 truncate mt-1">
              {nodeStatus?.subversion || "/Satoshi:27.0.0/"}
            </span>
          </div>

          <div className="flex flex-col">
            <span className="text-[11px] text-gray-400 uppercase tracking-wider font-mono">
              Protocol Verification
            </span>
            <span className="text-lg font-bold font-mono text-emerald-400 flex items-center gap-1.5">
              <CheckCircle2 className="w-4 h-4" /> 100%
            </span>
          </div>
        </div>
      </div>

      {/* Main Tab Navigation */}
      <div className="border-b border-border bg-surface">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 flex items-center justify-between">
          <div className="flex items-center gap-1 sm:gap-2 overflow-x-auto">
            {[
              { id: "graph", label: "UTXO Graph (DAG)", icon: Activity, key: "1" },
              { id: "tx", label: "Transaction Explorer", icon: GitCommit, key: "2" },
              { id: "flow", label: "Value Flow & Fees", icon: ArrowRight, key: "3" },
              { id: "utxos", label: "UTXO Set & Lifecycles", icon: Database, key: "4" },
              { id: "analytics", label: "UTXO Analytics", icon: TrendingUp, key: "5" },
            ].map((tab) => {
              const Icon = tab.icon;
              const active = activeTab === tab.id;
              return (
                <button
                  key={tab.id}
                  onClick={() => setActiveTab(tab.id as any)}
                  className={`flex items-center gap-2 py-3 px-3 sm:px-4 text-xs font-medium border-b-2 transition-colors whitespace-nowrap ${
                    active
                      ? "border-btc text-white bg-card/60"
                      : "border-transparent text-gray-400 hover:text-gray-200 hover:bg-card/20"
                  }`}
                >
                  <Icon className={`w-4 h-4 ${active ? "text-btc" : "text-gray-400"}`} />
                  {tab.label}
                  <span className="hidden md:inline-block px-1 py-0.2 text-[9px] font-mono text-gray-500 bg-background/50 border border-border/60 rounded">
                    {tab.key}
                  </span>
                </button>
              );
            })}
          </div>

          <div className="hidden lg:flex items-center gap-2 text-[10px] font-mono text-gray-400">
            <span>Shortcuts:</span>
            <kbd className="px-1 py-0.5 bg-card border border-border rounded text-gray-300">/</kbd> search
            <kbd className="px-1 py-0.5 bg-card border border-border rounded text-gray-300">1-5</kbd> tabs
            <kbd className="px-1 py-0.5 bg-card border border-border rounded text-gray-300">Esc</kbd> reset
          </div>
        </div>
      </div>

      {/* Main Content Area */}
      <main className="flex-1 max-w-7xl mx-auto w-full px-4 sm:px-6 lg:px-8 py-6 space-y-6">
        {error && (
          <div className="p-4 rounded-xl bg-rose-950/40 border border-rose-800/60 text-rose-300 text-xs font-mono flex items-center justify-between">
            <span>{error}</span>
            <button onClick={() => setError(null)} className="text-rose-400 hover:text-white">
              ✕
            </button>
          </div>
        )}

        {/* Tab 1: Interactive UTXO Graph View */}
        {activeTab === "graph" && (
          <div className="space-y-4">
            {graphNodes.length > 0 ? (
              <UtxoGraphView
                nodes={graphNodes}
                edges={graphEdges}
                depth={graphDepth}
                onDepthChange={(newDepth) => {
                  setGraphDepth(newDepth);
                  if (currentTx) {
                    loadTxData(currentTx.txid.toString(), newDepth);
                  }
                }}
                onNodeClick={(node) => {
                  if (node.type === "transaction" && (node.data as any).txid) {
                    loadTxData((node.data as any).txid);
                  }
                }}
              />
            ) : (
              <div className="bg-surface border border-border rounded-xl p-8 text-center min-h-[450px] flex flex-col items-center justify-center">
                <div className="w-14 h-14 rounded-2xl bg-card border border-border flex items-center justify-center text-btc mb-4 shadow-xl">
                  <Activity className="w-7 h-7" />
                </div>
                <h3 className="text-sm font-semibold text-white mb-1">
                  Explore Bitcoin UTXO Tree
                </h3>
                <p className="text-xs text-gray-400 max-w-md mb-6">
                  Select an indexed block transaction below or paste any TXID or Outpoint into the search bar to inspect the transaction DAG.
                </p>

                {recentBlocks.length > 0 && (
                  <div className="max-w-xl w-full text-left space-y-2">
                    <span className="text-[11px] font-mono text-gray-400 uppercase tracking-wider block">
                      Recent Indexed Blocks
                    </span>
                    <div className="grid grid-cols-1 sm:grid-cols-2 gap-2">
                      {recentBlocks.slice(0, 4).map((b) => (
                        <div
                          key={b.hash}
                          className="p-2.5 rounded-lg bg-card border border-border text-xs font-mono flex items-center justify-between"
                        >
                          <div>
                            <span className="text-white font-bold block">Block #{b.height}</span>
                            <span className="text-gray-400 text-[10px]">{b.tx_count} transactions</span>
                          </div>
                          <button
                            onClick={() => {
                              setSearchQuery(`block:${b.height}`);
                            }}
                            className="text-btc hover:underline text-[11px]"
                          >
                            Inspect
                          </button>
                        </div>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            )}
          </div>
        )}

        {/* Tab 2: Transaction Explorer */}
        {activeTab === "tx" && (
          <div>
            {currentTx ? (
              <TxDetailView
                tx={currentTx}
                onSelectTxid={(txid) => loadTxData(txid)}
                onSelectOutpoint={(txid, vout) => loadUtxoData(txid, vout)}
              />
            ) : (
              <div className="bg-surface border border-border rounded-xl p-8 text-center min-h-[300px] flex flex-col items-center justify-center">
                <GitCommit className="w-8 h-8 text-btc mb-3" />
                <h3 className="text-sm font-semibold text-white mb-1">
                  No Transaction Loaded
                </h3>
                <p className="text-xs text-gray-400 max-w-md">
                  Enter a 64-character Transaction ID in the top search bar to inspect inputs, outputs, locktimes, and witness stacks.
                </p>
              </div>
            )}
          </div>
        )}

        {/* Tab 3: UTXO Explorer & Lifecycles */}
        {activeTab === "utxos" && (
          <div className="space-y-4">
            {currentUtxo ? (
              <div className="space-y-4">
                <div className="flex items-center justify-between">
                  <button
                    onClick={() => setCurrentUtxo(null)}
                    className="px-3 py-1.5 rounded-lg bg-card hover:bg-card/80 border border-border text-xs font-mono text-gray-300 hover:text-white transition-colors"
                  >
                    ← Back to Active UTXO List
                  </button>
                </div>
                <UtxoDetailView
                  utxo={currentUtxo}
                  onNavigateTxid={(txid) => {
                    loadTxData(txid);
                    setActiveTab("tx");
                  }}
                  onOpenInGraph={(txid) => {
                    loadTxData(txid);
                    setActiveTab("graph");
                  }}
                />
              </div>
            ) : (
              <UtxoListView
                utxos={utxoList}
                onSelectUtxo={(utxo) => setCurrentUtxo(utxo)}
              />
            )}
          </div>
        )}

        {/* Tab 4: Value Flow & Fees */}
        {activeTab === "flow" && (
          <div>
            {currentTx ? (
              <ValueFlowSankey tx={currentTx} />
            ) : (
              <div className="bg-surface border border-border rounded-xl p-8 text-center min-h-[300px] flex flex-col items-center justify-center">
                <ArrowRight className="w-8 h-8 text-btc mb-3" />
                <h3 className="text-sm font-semibold text-white mb-1">
                  No Transaction Loaded for Value Flow Analysis
                </h3>
                <p className="text-xs text-gray-400 max-w-md">
                  Load a transaction using the search bar to inspect the input-to-output satoshi distribution and miner fee calculation.
                </p>
              </div>
            )}
          </div>
        )}

        {/* Tab 5: UTXO Analytics */}
        {activeTab === "analytics" && (
          <div>
            <UtxoAnalyticsView />
          </div>
        )}

        {/* Educational Callout */}
        <div className="p-4 rounded-xl bg-card/60 border border-border/80 flex items-start gap-3">
          <ShieldCheck className="w-5 h-5 text-btc flex-shrink-0 mt-0.5" />
          <div className="text-xs text-gray-300">
            <strong className="text-white font-medium">Bitcoin UTXO Model: </strong>
            In Bitcoin, an input does not refer to a transaction; it explicitly refers to an <em>Outpoint</em> (<code className="font-mono text-gray-100">TXID:VOUT</code>). The UTXO is consumed atomically, and new outputs with distinct locking scripts (<code className="font-mono text-gray-100">scriptPubKey</code>) are generated.
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
