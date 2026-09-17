"use client";

import React from "react";
import {
  ArrowDownRight,
  ArrowUpRight,
  CheckCircle2,
  Clock,
  Copy,
  ExternalLink,
  GitCommit,
  Layers,
  ShieldAlert,
  ShieldCheck,
} from "lucide-react";

export interface TransactionDetailData {
  txid: String;
  block_hash?: string | null;
  block_height?: number | null;
  version: number;
  locktime: number;
  size: number;
  vsize: number;
  weight: number;
  is_coinbase: boolean;
  fee?: number | null;
  fee_rate?: number | null;
  status: string;
  first_seen: string;
  inputs: Array<{
    vin: number;
    prev_txid: string;
    prev_vout: number;
    sequence: number;
    script_sig_asm?: string | null;
    witness_items?: string[] | null;
    value?: number | null;
    is_coinbase?: boolean;
  }>;
  outputs: Array<{
    vout: number;
    value: number;
    value_btc: number;
    script_pubkey_asm: string;
    script_type: string;
    address?: string | null;
    is_spent: boolean;
    spent_by_txid?: string | null;
    spent_by_vin?: number | null;
  }>;
}

interface TxDetailViewProps {
  tx: TransactionDetailData;
  onSelectTxid?: (txid: string) => void;
  onSelectOutpoint?: (txid: string, vout: number) => void;
}

export function TxDetailView({ tx, onSelectTxid, onSelectOutpoint }: TxDetailViewProps) {
  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  const isRbfSignaling = tx.inputs.some(
    (inp) =>
      !tx.is_coinbase &&
      inp.prev_txid !== "0000000000000000000000000000000000000000000000000000000000000000" &&
      inp.sequence < 0xfffffffe
  );

  return (
    <div className="space-y-6">
      {/* Transaction Overview Card */}
      <div className="bg-surface border border-border rounded-xl p-6 shadow-xl">
        <div className="flex flex-col lg:flex-row lg:items-center justify-between gap-4 pb-4 border-b border-border">
          <div>
            <div className="flex items-center gap-2 mb-1">
              <span className="text-xs font-mono uppercase px-2 py-0.5 rounded bg-btc/20 text-btc border border-btc/30 font-medium">
                {tx.is_coinbase ? "Coinbase Transaction" : "Standard Transaction"}
              </span>
              <span
                className={`text-xs font-mono px-2 py-0.5 rounded border ${
                  tx.status === "confirmed"
                    ? "bg-emerald-950/60 border-emerald-800/50 text-emerald-400"
                    : "bg-amber-950/60 border-amber-800/50 text-amber-400"
                }`}
              >
                {tx.status === "confirmed" ? "Confirmed" : "In Mempool (0-conf)"}
              </span>
              {isRbfSignaling && (
                <span className="text-xs font-mono px-2 py-0.5 rounded bg-purple-950/60 border border-purple-800/50 text-purple-300">
                  BIP 125 RBF Opt-In
                </span>
              )}
            </div>
            <div className="flex items-center gap-2">
              <span className="text-xs text-gray-400">TXID:</span>
              <code className="text-sm font-mono text-white break-all">{tx.txid}</code>
              <button
                onClick={() => copyToClipboard(tx.txid.toString())}
                className="p-1 hover:text-btc text-gray-400 transition-colors"
                title="Copy TXID"
              >
                <Copy className="w-3.5 h-3.5" />
              </button>
            </div>
          </div>

          <div className="flex items-center gap-4 text-xs font-mono">
            {tx.block_height !== null && tx.block_height !== undefined && (
              <div className="flex items-center gap-1.5 text-gray-300 bg-card px-3 py-1.5 rounded-lg border border-border">
                <Layers className="w-4 h-4 text-btc" />
                Block #{tx.block_height}
              </div>
            )}
            <div className="flex items-center gap-1.5 text-gray-400 bg-card px-3 py-1.5 rounded-lg border border-border">
              <Clock className="w-4 h-4" />
              {new Date(tx.first_seen).toLocaleTimeString()}
            </div>
          </div>
        </div>

        {/* Technical Stats Grid */}
        <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-4 pt-4 text-xs font-mono">
          <div>
            <span className="text-gray-400 block mb-0.5">Size / vSize</span>
            <span className="text-white font-medium">
              {tx.size} B / <span className="text-btc">{tx.vsize} vB</span>
            </span>
          </div>
          <div>
            <span className="text-gray-400 block mb-0.5">Weight Units</span>
            <span className="text-white font-medium">{tx.weight} WU</span>
          </div>
          <div>
            <span className="text-gray-400 block mb-0.5">Version</span>
            <span className="text-white font-medium">{tx.version}</span>
          </div>
          <div>
            <span className="text-gray-400 block mb-0.5">Locktime</span>
            <span className="text-white font-medium">{tx.locktime}</span>
          </div>
          <div>
            <span className="text-gray-400 block mb-0.5">Fee</span>
            <span className="text-amber-300 font-medium">
              {tx.fee ? `${tx.fee.toLocaleString()} sats` : tx.is_coinbase ? "Coinbase (0 sats)" : "Unknown"}
            </span>
          </div>
          <div>
            <span className="text-gray-400 block mb-0.5">Fee Rate</span>
            <span className="text-amber-300 font-medium">
              {tx.fee_rate ? `${tx.fee_rate.toFixed(2)} sat/vB` : "N/A"}
            </span>
          </div>
        </div>
      </div>

      {/* Inputs and Outputs Columns */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Inputs Vector */}
        <div className="bg-surface border border-border rounded-xl p-5 shadow-xl">
          <div className="flex items-center justify-between pb-3 border-b border-border mb-4">
            <h3 className="text-sm font-semibold text-white flex items-center gap-2 font-mono">
              <ArrowDownRight className="w-4 h-4 text-rose-400" />
              Inputs ({tx.inputs.length})
            </h3>
            <span className="text-xs text-gray-400 font-mono">Referenced UTXOs</span>
          </div>

          <div className="space-y-3">
            {tx.inputs.map((input) => (
              <div
                key={input.vin}
                className="p-3 bg-card rounded-lg border border-border/80 text-xs font-mono space-y-1.5"
              >
                <div className="flex items-center justify-between">
                  <span className="text-gray-400">vin #{input.vin}</span>
                  <span className="text-gray-400">Sequence: 0x{input.sequence.toString(16)}</span>
                </div>

                {input.is_coinbase ||
                tx.is_coinbase ||
                input.prev_txid ===
                  "0000000000000000000000000000000000000000000000000000000000000000" ? (
                  <div className="p-2 rounded bg-amber-950/30 border border-amber-800/30 text-amber-300 text-[11px]">
                    Coinbase Input (New subsidy generation)
                  </div>
                ) : (
                  <div>
                    <span className="text-gray-400 block mb-0.5">Spends Outpoint:</span>
                    <button
                      onClick={() =>
                        onSelectOutpoint && onSelectOutpoint(input.prev_txid, input.prev_vout)
                      }
                      className="text-btc hover:underline break-all text-left flex items-center gap-1 group"
                    >
                      {input.prev_txid}:{input.prev_vout}
                      <ExternalLink className="w-3 h-3 opacity-0 group-hover:opacity-100 transition-opacity flex-shrink-0" />
                    </button>
                  </div>
                )}

                {input.witness_items && input.witness_items.length > 0 && (
                  <div className="pt-1 text-[11px] text-gray-400">
                    <span>Witness stack: {input.witness_items.length} items</span>
                  </div>
                )}
              </div>
            ))}
          </div>
        </div>

        {/* Outputs Vector (UTXOs) */}
        <div className="bg-surface border border-border rounded-xl p-5 shadow-xl">
          <div className="flex items-center justify-between pb-3 border-b border-border mb-4">
            <h3 className="text-sm font-semibold text-white flex items-center gap-2 font-mono">
              <ArrowUpRight className="w-4 h-4 text-emerald-400" />
              Outputs ({tx.outputs.length})
            </h3>
            <span className="text-xs text-gray-400 font-mono">Created Spendable UTXOs</span>
          </div>

          <div className="space-y-3">
            {tx.outputs.map((output) => (
              <div
                key={output.vout}
                className="p-3 bg-card rounded-lg border border-border/80 text-xs font-mono space-y-2"
              >
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-2">
                    <span className="text-gray-400 font-bold">vout #{output.vout}</span>
                    <span className="text-[10px] uppercase px-1.5 py-0.5 rounded bg-border text-gray-300 font-medium">
                      {output.script_type}
                    </span>
                  </div>

                  <span
                    className={`text-[11px] px-2 py-0.5 rounded border ${
                      output.is_spent
                        ? "bg-slate-800/60 border-slate-700 text-slate-400"
                        : "bg-emerald-950/60 border-emerald-800/50 text-emerald-400"
                    }`}
                  >
                    {output.is_spent ? "Spent" : "Unspent UTXO"}
                  </span>
                </div>

                <div className="flex items-center justify-between">
                  <span className="text-base font-bold text-white">
                    {output.value.toLocaleString()} <span className="text-xs text-gray-400 font-normal">sats</span>
                  </span>
                  <span className="text-gray-400 text-xs">({output.value_btc.toFixed(8)} BTC)</span>
                </div>

                {output.address && (
                  <div className="flex items-center justify-between text-[11px]">
                    <span className="text-gray-400 truncate max-w-[280px]">
                      Address: {output.address}
                    </span>
                    <button
                      onClick={() => copyToClipboard(output.address!)}
                      className="text-gray-400 hover:text-white"
                      title="Copy Address"
                    >
                      <Copy className="w-3 h-3" />
                    </button>
                  </div>
                )}

                {output.is_spent && output.spent_by_txid && (
                  <div className="text-[11px] text-gray-400 pt-1 border-t border-border/40">
                    <span>Spent by: </span>
                    <button
                      onClick={() => onSelectTxid && onSelectTxid(output.spent_by_txid!)}
                      className="text-purple-400 hover:underline break-all"
                    >
                      {output.spent_by_txid} (vin #{output.spent_by_vin})
                    </button>
                  </div>
                )}
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
