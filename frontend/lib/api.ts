const API_BASE = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080";

export async function fetchNodeStatus() {
  const res = await fetch(`${API_BASE}/api/node`, { cache: "no-store" });
  if (!res.ok) throw new Error(`Failed to fetch node status: ${res.statusText}`);
  return res.json();
}

export async function fetchBlocks() {
  const res = await fetch(`${API_BASE}/api/blocks`, { cache: "no-store" });
  if (!res.ok) throw new Error(`Failed to fetch blocks: ${res.statusText}`);
  return res.json();
}

export async function fetchTransaction(txid: string) {
  const res = await fetch(`${API_BASE}/api/transactions/${txid}`, {
    cache: "no-store",
  });
  if (!res.ok) throw new Error(`Failed to fetch transaction ${txid}: ${res.statusText}`);
  return res.json();
}

export async function fetchTransactionGraph(txid: string, depth = 2, mode = "dual") {
  const res = await fetch(
    `${API_BASE}/api/transactions/${txid}/graph?depth=${depth}&mode=${mode}`,
    { cache: "no-store" }
  );
  if (!res.ok) throw new Error(`Failed to fetch graph for ${txid}: ${res.statusText}`);
  return res.json();
}

export async function fetchUtxos(limit = 50, offset = 0) {
  const res = await fetch(`${API_BASE}/api/utxos?limit=${limit}&offset=${offset}`, {
    cache: "no-store",
  });
  if (!res.ok) throw new Error(`Failed to fetch UTXOs: ${res.statusText}`);
  return res.json();
}

export async function fetchUtxo(txid: string, vout: number) {
  const res = await fetch(`${API_BASE}/api/utxos/${txid}/${vout}`, {
    cache: "no-store",
  });
  if (!res.ok) throw new Error(`Failed to fetch UTXO ${txid}:${vout}: ${res.statusText}`);
  return res.json();
}
