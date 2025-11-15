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

