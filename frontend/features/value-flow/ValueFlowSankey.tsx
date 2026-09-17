"use client";

import React from "react";
import { TransactionDetailData } from "@/features/explorer/TxDetailView";
import { ArrowRight, Flame, ShieldAlert, Sparkles } from "lucide-react";

interface ValueFlowSankeyProps {
  tx: TransactionDetailData;
}

export function ValueFlowSankey({ tx }: ValueFlowSankeyProps) {
  const totalOutputSats = tx.outputs.reduce((acc, out) => acc + out.value, 0);

  // If inputs have known values, sum them up. Otherwise estimate from output + fee
  const knownInputSats = tx.inputs.reduce(
    (acc, inp) => acc + (inp.value || 0),
    0
  );

  const totalInputSats =
    knownInputSats > 0
      ? knownInputSats
      : tx.fee
      ? totalOutputSats + tx.fee
      : totalOutputSats;

  const feeSats = tx.fee || (totalInputSats > totalOutputSats ? totalInputSats - totalOutputSats : 0);
  const feePercent = totalInputSats > 0 ? (feeSats / totalInputSats) * 100 : 0;

  return (
    <div className="bg-surface border border-border rounded-xl p-6 shadow-xl space-y-6">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-2 pb-4 border-b border-border">
        <div>
          <h3 className="text-sm font-semibold text-white font-mono flex items-center gap-2">
            <ArrowRight className="w-4 h-4 text-btc" />
            Satoshi Value Flow & Fee Destruction
          </h3>
          <p className="text-xs text-gray-400 mt-0.5">
            Exact satoshi accounting: Total Inputs must equal Total Outputs + Miner Fee.
          </p>
        </div>

        <div className="flex items-center gap-3 text-xs font-mono">
          <span className="text-gray-400">Total Throughput:</span>
          <span className="text-white font-bold">
            {totalInputSats.toLocaleString()} sats ({(totalInputSats / 100_000_000).toFixed(8)} BTC)
          </span>
        </div>
      </div>

      {/* Main Flow Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-11 gap-4 items-center">
        {/* Left Column: Inputs (4 cols) */}
        <div className="lg:col-span-4 space-y-3">
          <div className="text-xs font-mono text-gray-400 uppercase tracking-wider flex items-center justify-between">
            <span>Inputs ({tx.inputs.length})</span>
            <span className="text-emerald-400 font-bold">
              {totalInputSats.toLocaleString()} sats
            </span>
          </div>

          <div className="space-y-2 max-h-[350px] overflow-y-auto pr-1">
            {tx.inputs.map((inp) => {
              const inputVal = inp.value || (totalInputSats / (tx.inputs.length || 1));
              const pct = totalInputSats > 0 ? (inputVal / totalInputSats) * 100 : 100;

              return (
                <div
                  key={inp.vin}
                  className="p-3 rounded-lg bg-card border border-border/80 text-xs font-mono space-y-1"
                >
                  <div className="flex justify-between items-center text-gray-400">
                    <span>vin #{inp.vin}</span>
                    <span className="text-white font-semibold">
                      {inputVal.toLocaleString()} sats
                    </span>
                  </div>
                  <div className="flex justify-between text-[11px] text-gray-500">
                    <span className="truncate max-w-[150px]">
                      {inp.is_coinbase ? "Coinbase Subsidy" : `${inp.prev_txid.slice(0, 10)}...:${inp.prev_vout}`}
                    </span>
                    <span>{pct.toFixed(1)}%</span>
                  </div>
                  {/* Visual Proportion Bar */}
                  <div className="w-full h-1.5 rounded-full bg-border overflow-hidden mt-1">
                    <div
                      className="h-full bg-emerald-500 rounded-full"
                      style={{ width: `${Math.min(pct, 100)}%` }}
                    />
                  </div>
                </div>
              );
            })}
          </div>
        </div>

        {/* Center: Transaction Engine (3 cols) */}
        <div className="lg:col-span-3 flex flex-col items-center justify-center p-5 bg-card/60 border-2 border-btc/40 rounded-xl text-center font-mono space-y-3 shadow-[0_0_20px_rgba(247,147,26,0.1)]">
          <div className="w-12 h-12 rounded-xl bg-btc/20 border border-btc/40 flex items-center justify-center text-btc font-bold text-lg">
            ⚡
          </div>
          <div>
            <span className="text-xs text-gray-400 uppercase tracking-widest block">
              Transaction Engine
            </span>
            <span className="text-sm font-bold text-white break-all">
              {tx.txid ? `${tx.txid.slice(0, 12)}...` : ""}
            </span>
          </div>

          <div className="w-full pt-2 border-t border-border/80 space-y-1 text-xs">
            <div className="flex justify-between text-gray-400">
              <span>Virtual Size:</span>
              <span className="text-white">{tx.vsize} vB</span>
            </div>
            <div className="flex justify-between text-gray-400">
              <span>Fee Rate:</span>
              <span className="text-amber-300">
                {tx.fee_rate ? `${tx.fee_rate.toFixed(2)} sat/vB` : "N/A"}
              </span>
            </div>
          </div>
        </div>

        {/* Right Column: Outputs + Fee (4 cols) */}
        <div className="lg:col-span-4 space-y-3">
          <div className="text-xs font-mono text-gray-400 uppercase tracking-wider flex items-center justify-between">
            <span>Outputs ({tx.outputs.length}) + Fee</span>
            <span className="text-white font-bold">
              {totalInputSats.toLocaleString()} sats
            </span>
          </div>

          <div className="space-y-2 max-h-[350px] overflow-y-auto pr-1">
            {/* Outputs */}
            {tx.outputs.map((out) => {
              const pct = totalInputSats > 0 ? (out.value / totalInputSats) * 100 : 0;

              return (
                <div
                  key={out.vout}
                  className="p-3 rounded-lg bg-card border border-border/80 text-xs font-mono space-y-1"
                >
                  <div className="flex justify-between items-center text-gray-400">
                    <div className="flex items-center gap-1.5">
                      <span>vout #{out.vout}</span>
                      <span className="text-[10px] px-1 rounded bg-border text-gray-300 uppercase">
                        {out.script_type}
                      </span>
                    </div>
                    <span className="text-white font-semibold">
                      {out.value.toLocaleString()} sats
                    </span>
                  </div>
                  <div className="flex justify-between text-[11px] text-gray-500">
                    <span className="truncate max-w-[150px]">
                      {out.address ? out.address : "Script output"}
                    </span>
                    <span>{pct.toFixed(1)}%</span>
                  </div>
                  {/* Visual Proportion Bar */}
                  <div className="w-full h-1.5 rounded-full bg-border overflow-hidden mt-1">
                    <div
                      className="h-full bg-btc rounded-full"
                      style={{ width: `${Math.min(pct, 100)}%` }}
                    />
                  </div>
                </div>
              );
            })}

            {/* Miner Fee Output Ribbon */}
            {feeSats > 0 && (
              <div className="p-3 rounded-lg bg-amber-950/20 border border-amber-800/40 text-xs font-mono space-y-1">
                <div className="flex justify-between items-center text-amber-300">
                  <span className="flex items-center gap-1">
                    <Flame className="w-3.5 h-3.5 text-amber-400" />
                    Miner Fee (Consumed)
                  </span>
                  <span className="font-bold">{feeSats.toLocaleString()} sats</span>
                </div>
                <div className="flex justify-between text-[11px] text-amber-400/70">
                  <span>Claimed by Miner in Coinbase</span>
                  <span>{feePercent.toFixed(2)}%</span>
                </div>
                <div className="w-full h-1.5 rounded-full bg-amber-950/60 overflow-hidden mt-1">
                  <div
                    className="h-full bg-amber-500 rounded-full"
                    style={{ width: `${Math.min(feePercent, 100)}%` }}
                  />
                </div>
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Technical Protocol Note */}
      <div className="p-4 rounded-lg bg-card/60 border border-border/80 flex items-start gap-3">
        <ShieldAlert className="w-5 h-5 text-amber-400 flex-shrink-0 mt-0.5" />
        <div className="text-xs text-gray-300 font-mono">
          <strong className="text-white">Protocol Principle: </strong>
          Transaction fees in Bitcoin are <em>never</em> an explicit output in the transaction itself. The fee is implicitly destroyed from the input satoshis and recreated by the block miner in the coinbase transaction.
        </div>
      </div>
    </div>
  );
}
