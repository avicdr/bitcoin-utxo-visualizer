"use client";

import React, { useEffect, useState } from "react";
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  Tooltip,
  ResponsiveContainer,
  PieChart,
  Pie,
  Cell,
  CartesianGrid,
  Legend,
} from "recharts";
import {
  Coins,
  Database,
  Layers,
  PieChart as PieIcon,
  TrendingUp,
  Clock,
  RotateCw,
  ShieldCheck,
  AlertTriangle,
} from "lucide-react";
import {
  fetchUtxoAnalytics,
  fetchValueDistribution,
  fetchAgeDistribution,
  fetchScriptDistribution,
} from "@/lib/api";

const SCRIPT_COLORS = [
  "#f7931a", // btc orange
  "#10b981", // emerald
  "#3b82f6", // blue
  "#8b5cf6", // purple
  "#ec4899", // pink
  "#eab308", // yellow
  "#64748b", // slate
];

export function UtxoAnalyticsView() {
  const [stats, setStats] = useState<any>(null);
  const [valueDist, setValueDist] = useState<any[]>([]);
  const [ageDist, setAgeDist] = useState<any[]>([]);
  const [scriptDist, setScriptDist] = useState<any[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const loadData = async () => {
    setIsLoading(true);
    setError(null);
    try {
      const [s, v, a, sc] = await Promise.all([
        fetchUtxoAnalytics(),
        fetchValueDistribution(),
        fetchAgeDistribution(),
        fetchScriptDistribution(),
      ]);
      setStats(s);
      setValueDist(v);
      setAgeDist(a);
      setScriptDist(sc);
    } catch (err: any) {
      setError(err.message || "Failed to load UTXO analytics data");
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    loadData();
  }, []);

  if (isLoading) {
    return (
      <div className="flex flex-col items-center justify-center p-16 space-y-4 font-mono">
        <RotateCw className="w-8 h-8 text-btc animate-spin" />
        <span className="text-gray-400 text-sm">Computing UTXO analytics & distributions...</span>
      </div>
    );
  }

  if (error) {
    return (
      <div className="p-8 bg-red-950/40 border border-red-800 rounded-xl text-center font-mono">
        <AlertTriangle className="w-8 h-8 text-red-400 mx-auto mb-2" />
        <p className="text-red-300 text-sm">{error}</p>
        <button
          onClick={loadData}
          className="mt-4 px-4 py-1.5 bg-surface hover:bg-card border border-border text-white text-xs rounded-lg transition"
        >
          Retry
        </button>
      </div>
    );
  }

  return (
    <div className="space-y-6 font-mono text-xs">
      {/* KPI Overview Cards */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        <div className="bg-surface border border-border rounded-xl p-4 shadow-xl">
          <div className="flex items-center justify-between text-gray-400 mb-1">
            <span className="text-[11px] uppercase tracking-wider">Active UTXO Count</span>
            <Database className="w-4 h-4 text-btc" />
          </div>
          <div className="text-2xl font-bold text-white">
            {stats?.total_indexed_utxos?.toLocaleString() ?? 0}
          </div>
          <div className="text-[10px] text-gray-400 mt-1 flex justify-between">
            <span>Spent: {stats?.spent_outputs_count?.toLocaleString()}</span>
            <span>Ratio: {((stats?.spent_ratio ?? 0) * 100).toFixed(1)}%</span>
          </div>
        </div>

        <div className="bg-surface border border-border rounded-xl p-4 shadow-xl">
          <div className="flex items-center justify-between text-gray-400 mb-1">
            <span className="text-[11px] uppercase tracking-wider">Total Value (BTC)</span>
            <Coins className="w-4 h-4 text-emerald-400" />
          </div>
          <div className="text-2xl font-bold text-white">
            {stats?.total_indexed_btc ? stats.total_indexed_btc.toFixed(4) : "0.0000"}{" "}
            <span className="text-xs text-gray-400 font-normal">BTC</span>
          </div>
          <div className="text-[10px] text-gray-400 mt-1">
            {stats?.total_indexed_sats?.toLocaleString()} sats
          </div>
        </div>

        <div className="bg-surface border border-border rounded-xl p-4 shadow-xl">
          <div className="flex items-center justify-between text-gray-400 mb-1">
            <span className="text-[11px] uppercase tracking-wider">Coin Days Destroyed</span>
            <Clock className="w-4 h-4 text-purple-400" />
          </div>
          <div className="text-2xl font-bold text-white">
            {stats?.total_coin_days ? stats.total_coin_days.toFixed(1) : "0.0"}
          </div>
          <div className="text-[10px] text-gray-400 mt-1">Sum(Value × Lifespan in Days)</div>
        </div>

        <div className="bg-surface border border-border rounded-xl p-4 shadow-xl">
          <div className="flex items-center justify-between text-gray-400 mb-1">
            <span className="text-[11px] uppercase tracking-wider">Audit Scope</span>
            <ShieldCheck className="w-4 h-4 text-blue-400" />
          </div>
          <div className="text-sm font-semibold text-emerald-400 mt-1">Consensus-Indexed</div>
          <div className="text-[10px] text-gray-400 mt-1 line-clamp-2">
            Local indexed chain state (non-fabricated).
          </div>
        </div>
      </div>

      {/* Distribution Charts */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Value Distribution */}
        <div className="bg-surface border border-border rounded-xl p-5 shadow-xl">
          <div className="flex items-center justify-between mb-4">
            <h4 className="text-sm font-semibold text-white flex items-center gap-2">
              <TrendingUp className="w-4 h-4 text-btc" />
              UTXO Value Distribution
            </h4>
            <span className="text-[10px] text-gray-400">By output count</span>
          </div>
          <div className="h-64 w-full">
            <ResponsiveContainer width="100%" height="100%">
              <BarChart data={valueDist}>
                <CartesianGrid strokeDasharray="3 3" stroke="#232b3e" />
                <XAxis dataKey="range_label" stroke="#64748b" tick={{ fontSize: 10 }} />
                <YAxis stroke="#64748b" tick={{ fontSize: 10 }} />
                <Tooltip
                  contentStyle={{ backgroundColor: "#151b28", borderColor: "#2d3748", fontSize: 11 }}
                  formatter={(val: any) => [val.toLocaleString(), "UTXOs"]}
                />
                <Bar dataKey="count" fill="#f7931a" radius={[4, 4, 0, 0]} />
              </BarChart>
            </ResponsiveContainer>
          </div>
        </div>

        {/* Age Distribution */}
        <div className="bg-surface border border-border rounded-xl p-5 shadow-xl">
          <div className="flex items-center justify-between mb-4">
            <h4 className="text-sm font-semibold text-white flex items-center gap-2">
              <Clock className="w-4 h-4 text-emerald-400" />
              UTXO Age Distribution (HODL Bands)
            </h4>
            <span className="text-[10px] text-gray-400">Lifespan</span>
          </div>
          <div className="h-64 w-full">
            <ResponsiveContainer width="100%" height="100%">
              <BarChart data={ageDist}>
                <CartesianGrid strokeDasharray="3 3" stroke="#232b3e" />
                <XAxis dataKey="bucket" stroke="#64748b" tick={{ fontSize: 10 }} />
                <YAxis stroke="#64748b" tick={{ fontSize: 10 }} />
                <Tooltip
                  contentStyle={{ backgroundColor: "#151b28", borderColor: "#2d3748", fontSize: 11 }}
                  formatter={(val: any) => [val.toLocaleString(), "UTXOs"]}
                />
                <Bar dataKey="count" fill="#10b981" radius={[4, 4, 0, 0]} />
              </BarChart>
            </ResponsiveContainer>
          </div>
        </div>
      </div>

      {/* Script Types Pie Chart */}
      <div className="bg-surface border border-border rounded-xl p-5 shadow-xl">
        <div className="flex items-center justify-between mb-4">
          <h4 className="text-sm font-semibold text-white flex items-center gap-2">
            <PieIcon className="w-4 h-4 text-purple-400" />
            Script Type Distribution
          </h4>
          <span className="text-[10px] text-gray-400">P2PKH, P2SH, SegWit, Taproot</span>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6 items-center">
          <div className="h-64 w-full">
            <ResponsiveContainer width="100%" height="100%">
              <PieChart>
                <Pie
                  data={scriptDist}
                  dataKey="count"
                  nameKey="script_type"
                  cx="50%"
                  cy="50%"
                  outerRadius={80}
                  label={({ script_type, percentage }) =>
                    `${script_type} (${(percentage * 100).toFixed(0)}%)`
                  }
                >
                  {scriptDist.map((_, index) => (
                    <Cell
                      key={`cell-${index}`}
                      fill={SCRIPT_COLORS[index % SCRIPT_COLORS.length]}
                    />
                  ))}
                </Pie>
                <Tooltip
                  contentStyle={{ backgroundColor: "#151b28", borderColor: "#2d3748", fontSize: 11 }}
                  formatter={(val: any) => [val.toLocaleString(), "Outputs"]}
                />
              </PieChart>
            </ResponsiveContainer>
          </div>
          <div className="space-y-2">
            {scriptDist.map((item, i) => (
              <div
                key={item.script_type}
                className="flex items-center justify-between p-2 rounded bg-card/60 border border-border/50"
              >
                <div className="flex items-center gap-2">
                  <span
                    className="w-3 h-3 rounded-full"
                    style={{ backgroundColor: SCRIPT_COLORS[i % SCRIPT_COLORS.length] }}
                  />
                  <span className="text-white font-medium uppercase">{item.script_type}</span>
                </div>
                <div className="text-right">
                  <span className="text-white">{item.count.toLocaleString()}</span>
                  <span className="text-gray-400 ml-2">
                    ({(item.percentage * 100).toFixed(1)}%)
                  </span>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
