export function handleWasmResult(resultMap) {
    if (!(resultMap instanceof Map)) {
        console.error("❌ Result is not a Map:", resultMap);
        return { path: [], visited: [], cost: 0 };
    }

    const path = Array.isArray(resultMap.get("path")) ? resultMap.get("path") : [];
    const visited = Array.isArray(resultMap.get("visited")) ? resultMap.get("visited") : [];
    const cost = typeof resultMap.get("cost") === "number" ? resultMap.get("cost") : 0;

    return { path, visited, cost };
}

export async function id_to_name(nodeId) {
    const graphData = await fetchData();
    if (!graphData) {
        console.error("❌ No graph data available");
        return null;
    }
    return graphData.nodes.find(node => node.id === nodeId);
}

export async function fetchData() {
    try {
        const response = await fetch('/data/graph.json');
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data = await response.json();
        return data;
    } catch (error) {
        console.error("❌ Error fetching data:", error);
        return null;
    }
}