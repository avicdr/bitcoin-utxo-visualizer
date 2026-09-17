"use client";

import React from "react";
import {
  ArrowRight,
  CheckCircle2,
  Clock,
  Copy,
  ExternalLink,
  Layers,
  ShieldCheck,
  Zap,
} from "lucide-react";

export interface UtxoData {
  txid: string;
  vout: number;
  outpoint: string;
  value_sats: number;
  value_btc: number;
  script_pubkey_asm: string;
  script_pubkey_hex: string;
  script_type: string;
  address?: string | null;
  is_spent: boolean;
  confirmation_state: string;
  created_at_height?: number | null;
  created_at_time?: string | null;
  spent_by?: {
    spending_txid: string;
    spending_vin: number;
    spent_at_height?: number | null;
    spent_at_time?: string | null;
  } | null;
}

interface UtxoDetailViewProps {
  utxo: UtxoData;
  onNavigateTxid?: (txid: string) => void;
  onOpenInGraph?: (txid: string) => void;
}

export function UtxoDetailView({
  utxo,
  onNavigateTxid,
  onOpenInGraph,
}: UtxoDetailViewProps) {
  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  return (
    <div className="space-y-6">
      {/* Outpoint Header */}
      <div className="bg-surface border border-border rounded-xl p-6 shadow-xl">
        <div className="flex flex-col lg:flex-row lg:items-center justify-between gap-4 pb-4 border-b border-border">
          <div>
            <div className="flex items-center gap-2 mb-1.5">
              <span className="text-xs font-mono uppercase px-2 py-0.5 rounded bg-btc/20 text-btc border border-btc/30 font-medium">
                Outpoint (Txid:Vout)
              </span>
              <span
                className={`text-xs font-mono px-2 py-0.5 rounded border ${
                  utxo.is_spent
                    ? "bg-slate-800/80 border-slate-700 text-slate-400"
                    : "bg-emerald-950/80 border-emerald-800/60 text-emerald-400"
                }`}
              >
                {utxo.is_spent ? "Spent Output (Consumed)" : "Active UTXO (Spendable)"}
              </span>
              <span className="text-xs font-mono px-2 py-0.5 rounded bg-card border border-border text-gray-300 uppercase">
                {utxo.script_type}
              </span>
            </div>

            <div className="flex items-center gap-2">
              <code className="text-sm font-mono text-white break-all">{utxo.outpoint}</code>
              <button
                onClick={() => copyToClipboard(utxo.outpoint)}
                className="p-1 hover:text-btc text-gray-400 transition-colors"
                title="Copy Outpoint"
              >
                <Copy className="w-3.5 h-3.5" />
              </button>
            </div>
          </div>

          <div className="flex items-center gap-3">
            <button
              onClick={() => onOpenInGraph && onOpenInGraph(utxo.txid)}
              className="px-3 py-1.5 rounded-lg bg-btc/10 border border-btc/40 hover:bg-btc/20 text-btc text-xs font-mono flex items-center gap-1.5 transition-all"
            >
              <Zap className="w-3.5 h-3.5" />
              Visualize in Graph
            </button>
          </div>
        </div>

        {/* Technical Value & Block Summary */}
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 pt-4 text-xs font-mono">
          <div>
            <span className="text-gray-400 block mb-0.5">Value (Satoshis)</span>
            <span className="text-base font-bold text-white">
              {utxo.value_sats.toLocaleString()}{" "}
              <span className="text-xs text-gray-400 font-normal">sats</span>
            </span>
          </div>

          <div>
            <span className="text-gray-400 block mb-0.5">Value (Bitcoin)</span>
            <span className="text-base font-bold text-amber-300">
              {utxo.value_btc.toFixed(8)} BTC
            </span>
          </div>

          <div>
            <span className="text-gray-400 block mb-0.5">Creation Block Height</span>
            <span className="text-white font-medium flex items-center gap-1">
              <Layers className="w-3.5 h-3.5 text-btc" />
              {utxo.created_at_height ? `#${utxo.created_at_height}` : "Mempool (0-conf)"}
            </span>
          </div>

          <div>
            <span className="text-gray-400 block mb-0.5">Confirmation State</span>
            <span className="text-emerald-400 font-medium capitalize">
              {utxo.confirmation_state}
            </span>
          </div>
        </div>
      </div>

      {/* UTXO Lifecycle Timeline */}
      <div className="bg-surface border border-border rounded-xl p-6 shadow-xl">
        <h3 className="text-sm font-semibold text-white font-mono mb-4 flex items-center gap-2">
          <Clock className="w-4 h-4 text-btc" />
          UTXO Lifecycle State Progression
        </h3>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-xs font-mono">
          {/* Step 1: Created */}
          <div className="p-4 rounded-lg bg-card border border-border space-y-2">
            <div className="flex items-center gap-2 text-emerald-400 font-semibold">
              <CheckCircle2 className="w-4 h-4" />
              1. Created by Output
            </div>
            <div className="text-gray-400 text-[11px]">
              Output index #{utxo.vout} of transaction:
            </div>
            <button
              onClick={() => onNavigateTxid && onNavigateTxid(utxo.txid)}
              className="text-btc hover:underline break-all text-left text-[11px] block"
            >
              {utxo.txid}
            </button>
            {utxo.created_at_time && (
              <div className="text-[10px] text-gray-500">
                {new Date(utxo.created_at_time).toLocaleString()}
              </div>
            )}
          </div>

          {/* Step 2: Available in UTXO Set */}
          <div
            className={`p-4 rounded-lg border space-y-2 ${
              !utxo.is_spent
                ? "bg-emerald-950/20 border-emerald-800/60 text-emerald-300"
                : "bg-card border-border text-gray-400"
            }`}
          >
            <div className="flex items-center gap-2 font-semibold">
              <CheckCircle2 className="w-4 h-4" />
              2. Available in UTXO Set
            </div>
            <div className="text-[11px]">
              {!utxo.is_spent
                ? "Currently present in Bitcoin UTXO set. Can be spent by anyone who satisfies the locking script."
                : "Formerly unspent. Was held in the active UTXO set until spent."}
            </div>
            <div className="text-[11px] text-white font-medium">
              Encumbrance: {utxo.script_type.toUpperCase()}
            </div>
          </div>

          {/* Step 3: Spent or Unspent */}
          <div
            className={`p-4 rounded-lg border space-y-2 ${
              utxo.is_spent
                ? "bg-purple-950/20 border-purple-800/50"
                : "bg-card/40 border-dashed border-border text-gray-500"
            }`}
          >
            <div
              className={`flex items-center gap-2 font-semibold ${
                utxo.is_spent ? "text-purple-300" : "text-gray-500"
              }`}
            >
              {utxo.is_spent ? (
                <CheckCircle2 className="w-4 h-4 text-purple-400" />
              ) : (
                <span className="w-4 h-4 rounded-full border border-gray-600 inline-block" />
              )}
              3. Spent & Consumed
            </div>

            {utxo.is_spent && utxo.spent_by ? (
              <div className="space-y-1.5 text-[11px]">
                <div className="text-gray-400">Consumed by vin #{utxo.spent_by.spending_vin} in:</div>
                <button
                  onClick={() =>
                    onNavigateTxid && onNavigateTxid(utxo.spent_by!.spending_txid)
                  }
                  className="text-purple-400 hover:underline break-all text-left block"
                >
                  {utxo.spent_by.spending_txid}
                </button>
                {utxo.spent_by.spent_at_height && (
                  <div className="text-[10px] text-gray-400">
                    Confirmed in Block #{utxo.spent_by.spent_at_height}
                  </div>
                )}
              </div>
            ) : (
              <div className="text-[11px] text-gray-400">
                Not spent. Remains in the global active UTXO set.
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Script & Address Details */}
      <div className="bg-surface border border-border rounded-xl p-6 shadow-xl space-y-4">
        <h3 className="text-sm font-semibold text-white font-mono pb-2 border-b border-border">
          Locking Script (scriptPubKey)
        </h3>

        <div className="space-y-3 text-xs font-mono">
          <div>
            <span className="text-gray-400 block mb-1">Human-Readable Disassembly (ASM):</span>
            <div className="p-3 bg-card rounded-lg border border-border/80 text-emerald-400 font-mono break-all selection:bg-emerald-900">
              {utxo.script_pubkey_asm}
            </div>
          </div>

          <div>
            <span className="text-gray-400 block mb-1">Hex Encoded:</span>
            <div className="p-2.5 bg-card rounded-lg border border-border/80 text-gray-300 font-mono break-all text-[11px]">
              {utxo.script_pubkey_hex}
            </div>
          </div>

          {utxo.address && (
            <div>
              <span className="text-gray-400 block mb-1">Standard Address Representation:</span>
              <div className="flex items-center justify-between p-2.5 bg-card rounded-lg border border-border/80">
                <span className="text-btc font-mono break-all">{utxo.address}</span>
                <button
                  onClick={() => copyToClipboard(utxo.address!)}
                  className="text-gray-400 hover:text-white p-1"
                  title="Copy Address"
                >
                  <Copy className="w-3.5 h-3.5" />
                </button>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
