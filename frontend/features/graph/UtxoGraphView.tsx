"use client";

import React, { useMemo } from "react";
import {
  ReactFlow,
  Background,
  Controls,
  Handle,
  Position,
  Node,
  Edge,
  MarkerType,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import { Activity, CheckCircle2, Copy, GitCommit, Layers } from "lucide-react";

interface UtxoGraphViewProps {
  nodes: Node[];
  edges: Edge[];
  depth: number;
  onDepthChange: (depth: number) => void;
  onNodeClick?: (node: Node) => void;
}

// Custom Transaction Node
function CustomTxNode({ data }: { data: any }) {
  const isCoinbase = data.is_coinbase;
  const isConfirmed = data.confirmed;

  return (
    <div className="bg-surface border-2 border-btc/70 rounded-xl p-3 shadow-2xl min-w-[200px] text-xs font-mono">
      <Handle type="target" position={Position.Left} className="w-2.5 h-2.5 bg-btc border-2 border-background" />

      <div className="flex items-center justify-between gap-2 pb-1.5 border-b border-border/80">
        <div className="flex items-center gap-1 text-white font-semibold">
          <GitCommit className="w-3.5 h-3.5 text-btc" />
          <span>Tx: {data.txid ? `${data.txid.slice(0, 8)}...` : "Tx"}</span>
        </div>
        <span
          className={`text-[10px] px-1.5 py-0.5 rounded ${
            isConfirmed
              ? "bg-emerald-950/80 text-emerald-400 border border-emerald-800/60"
              : "bg-amber-950/80 text-amber-400 border border-amber-800/60"
          }`}
        >
          {isConfirmed ? "Confirmed" : "0-conf"}
        </span>
      </div>

      <div className="pt-2 space-y-1 text-[11px]">
        <div className="flex justify-between text-gray-400">
          <span>vSize:</span>
          <span className="text-gray-200">{data.vsize} vB</span>
        </div>
        <div className="flex justify-between text-gray-400">
          <span>Fee Rate:</span>
          <span className="text-amber-300">
            {data.fee_rate ? `${data.fee_rate.toFixed(1)} sat/vB` : isCoinbase ? "0 sat (Subsidy)" : "N/A"}
          </span>
        </div>
        {data.block_height && (
          <div className="flex justify-between text-gray-400">
            <span>Block:</span>
            <span className="text-gray-200">#{data.block_height}</span>
          </div>
        )}
      </div>

      <Handle type="source" position={Position.Right} className="w-2.5 h-2.5 bg-btc border-2 border-background" />
    </div>
  );
}

// Custom UTXO Node
function CustomUtxoNode({ data }: { data: any }) {
  const isSpent = data.is_spent;

  return (
    <div
      className={`bg-surface border-2 rounded-xl p-3 shadow-2xl min-w-[200px] text-xs font-mono transition-all ${
        isSpent
          ? "border-slate-700 opacity-80"
          : "border-utxo-unspent shadow-[0_0_15px_rgba(16,185,129,0.15)]"
      }`}
    >
      <Handle type="target" position={Position.Left} className="w-2.5 h-2.5 bg-utxo-unspent border-2 border-background" />

      <div className="flex items-center justify-between gap-2 pb-1.5 border-b border-border/80">
        <span className="text-white font-medium truncate max-w-[120px]">
          UTXO :{data.vout}
        </span>
        <span
          className={`text-[10px] px-1.5 py-0.5 rounded font-medium ${
            isSpent
              ? "bg-slate-800 text-slate-400 border border-slate-700"
              : "bg-emerald-950 text-utxo-unspent border border-emerald-800/80"
          }`}
        >
          {isSpent ? "Spent" : "Active UTXO"}
        </span>
      </div>

      <div className="pt-2 space-y-1 text-[11px]">
        <div className="text-sm font-bold text-white">
          {data.value_sats ? data.value_sats.toLocaleString() : 0}{" "}
          <span className="text-[10px] text-gray-400 font-normal">sats</span>
        </div>
        <div className="flex justify-between text-gray-400">
          <span>Type:</span>
          <span className="text-gray-300 uppercase text-[10px]">{data.script_type}</span>
        </div>
      </div>

      <Handle type="source" position={Position.Right} className="w-2.5 h-2.5 bg-utxo-unspent border-2 border-background" />
    </div>
  );
}

export function UtxoGraphView({
  nodes,
  edges,
  depth,
  onDepthChange,
  onNodeClick,
}: UtxoGraphViewProps) {
  const nodeTypes = useMemo(
    () => ({
      transaction: CustomTxNode,
      utxo: CustomUtxoNode,
    }),
    []
  );

  return (
    <div className="h-[600px] w-full bg-background border border-border rounded-xl relative overflow-hidden">
      {/* Graph Toolbar */}
      <div className="absolute top-4 left-4 z-10 flex items-center gap-2 bg-surface/90 backdrop-blur border border-border px-3 py-1.5 rounded-lg text-xs font-mono">
        <span className="text-gray-400">Traversal Depth:</span>
        {[1, 2, 3, 5].map((d) => (
          <button
            key={d}
            onClick={() => onDepthChange(d)}
            className={`px-2 py-0.5 rounded ${
              depth === d
                ? "bg-btc text-background font-bold"
                : "bg-card text-gray-300 hover:text-white"
            }`}
          >
            {d}
          </button>
        ))}
      </div>

      {/* Legend Badge */}
      <div className="absolute top-4 right-4 z-10 flex items-center gap-3 bg-surface/90 backdrop-blur border border-border px-3 py-1.5 rounded-lg text-[11px] font-mono">
        <span className="flex items-center gap-1.5 text-btc">
          <span className="w-2 h-2 rounded-full bg-btc inline-block" /> Transaction
        </span>
        <span className="flex items-center gap-1.5 text-utxo-unspent">
          <span className="w-2 h-2 rounded-full bg-utxo-unspent inline-block" /> Unspent UTXO
        </span>
        <span className="flex items-center gap-1.5 text-slate-400">
          <span className="w-2 h-2 rounded-full bg-slate-500 inline-block" /> Spent Output
        </span>
      </div>

      <ReactFlow
        nodes={nodes}
        edges={edges}
        nodeTypes={nodeTypes}
        onNodeClick={(_, node) => onNodeClick && onNodeClick(node)}
        fitView
      >
        <Background color="#252d3d" gap={20} size={1} />
        <Controls className="bg-surface border-border text-white fill-white" />
      </ReactFlow>
    </div>
  );
}
