"use client";

import React from "react";
import { Copy, Database, ExternalLink, Layers } from "lucide-react";
import { UtxoData } from "./UtxoDetailView";

interface UtxoListViewProps {
  utxos: UtxoData[];
  onSelectUtxo: (utxo: UtxoData) => void;
}

export function UtxoListView({ utxos, onSelectUtxo }: UtxoListViewProps) {
  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  return (
    <div className="bg-surface border border-border rounded-xl shadow-xl overflow-hidden">
      <div className="p-4 sm:px-6 border-b border-border flex items-center justify-between">
        <div>
          <h3 className="text-sm font-semibold text-white font-mono flex items-center gap-2">
            <Database className="w-4 h-4 text-btc" />
            Active UTXO Set ({utxos.length})
          </h3>
          <p className="text-xs text-gray-400 mt-0.5">
            Indexed unspent transaction outputs currently available for spending.
          </p>
        </div>
      </div>

      <div className="overflow-x-auto">
        <table className="w-full text-left text-xs font-mono">
          <thead className="bg-card/70 border-b border-border text-gray-400 text-[11px] uppercase tracking-wider">
            <tr>
              <th className="py-3 px-4">Outpoint (Txid:Vout)</th>
              <th className="py-3 px-4">Value (Sats)</th>
              <th className="py-3 px-4">Value (BTC)</th>
              <th className="py-3 px-4">Script Type</th>
              <th className="py-3 px-4">Block Height</th>
              <th className="py-3 px-4 text-right">Action</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-border/60">
            {utxos.map((utxo) => (
              <tr
                key={utxo.outpoint}
                className="hover:bg-card/50 transition-colors group cursor-pointer"
                onClick={() => onSelectUtxo(utxo)}
              >
                <td className="py-3 px-4 text-white">
                  <div className="flex items-center gap-1.5">
                    <span className="truncate max-w-[220px]">{utxo.outpoint}</span>
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        copyToClipboard(utxo.outpoint);
                      }}
                      className="text-gray-500 hover:text-white p-0.5 opacity-0 group-hover:opacity-100 transition-opacity"
                    >
                      <Copy className="w-3 h-3" />
                    </button>
                  </div>
                </td>
                <td className="py-3 px-4 text-emerald-400 font-bold">
                  {utxo.value_sats.toLocaleString()}
                </td>
                <td className="py-3 px-4 text-gray-300">
                  {utxo.value_btc.toFixed(8)}
                </td>
                <td className="py-3 px-4">
                  <span className="px-2 py-0.5 rounded bg-card border border-border text-gray-300 uppercase text-[10px]">
                    {utxo.script_type}
                  </span>
                </td>
                <td className="py-3 px-4 text-gray-400">
                  {utxo.created_at_height ? (
                    <span className="flex items-center gap-1">
                      <Layers className="w-3 h-3 text-btc" />
                      #{utxo.created_at_height}
                    </span>
                  ) : (
                    "0-conf"
                  )}
                </td>
                <td className="py-3 px-4 text-right">
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      onSelectUtxo(utxo);
                    }}
                    className="px-2.5 py-1 rounded bg-card hover:bg-btc/20 border border-border hover:border-btc/40 text-gray-300 hover:text-btc transition-colors text-[11px]"
                  >
                    Inspect
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
