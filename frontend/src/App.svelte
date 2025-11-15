<script>
import { onMount } from "svelte";
import Map from "./components/Map.svelte";
import Sidebar from "./components/Sidebar.svelte";

// ✅ IMPORTAR TODAS LAS FUNCIONES
import init, { run_astar, run_bfs, run_dfs, run_dijkstra } from "../pkg/nicaragua_graph.js";
import { handleWasmResult } from "./lib/utils";

let graph = null;
let geojson = null;
let wasmInitialized = false;

let startNode = null;
let endNode = null;
let selectedAlgorithm = "a_star";

let pathResult = [];
let visitedNodes = [];
let totalCost = 0;
let isRunning = false;

// Cargar datos y WASM
onMount(async () => {
    try {
        const [graphResponse, geoResponse] = await Promise.all([
            fetch("data/nicaragua_graph.json"),
            fetch("data/nicaragua_municipios_simplificado.geojson")
        ]);
        
        graph = await graphResponse.json();
        geojson = await geoResponse.json();

            await init();
            wasmInitialized = true;
    } catch (error) {
        console.error("Error loading data:", error);
    }
});

function handleMapClick(event) {
    const clickedId = event.detail;
    if (isRunning) return;
    
    if (!startNode) {
        startNode = clickedId;
    } else if (!endNode && clickedId !== startNode) {
        endNode = clickedId;
    } else {
        startNode = clickedId;
        endNode = null;
        pathResult = [];
        visitedNodes = [];
        totalCost = 0;
    }
}

function findNode(nodeId) {
    return graph?.nodes?.find(node => node.id === nodeId);
}

async function runAlgorithm() {
    if (!wasmInitialized || !graph) {
        console.error("❌ WASM or graph not initialized");
        return;
    }

    if (!startNode) {
        console.error("❌ No start node selected");
        return;
    }

    if ((selectedAlgorithm === "a_star" || selectedAlgorithm === "bfs" || selectedAlgorithm === "dfs" || selectedAlgorithm === "dijkstra") && !endNode) {
        console.error("❌ No end node selected for", selectedAlgorithm);
        return;
    }

    if (!findNode(startNode)) {
        console.error("❌ Start node not found in graph:", startNode);
        return;
    }

    if (endNode && !findNode(endNode)) {
        console.error("❌ End node not found in graph:", endNode);
        return;
    }

    if (isRunning) return;
    isRunning = true;

    try {
        // Reset resultados anteriores
        pathResult = [];
        visitedNodes = [];
        totalCost = 0;

        
        
        let result;
        
        switch (selectedAlgorithm) {
            case "a_star":
                result = run_astar(startNode, endNode, graph);
                break;
            case "bfs":
                result = run_bfs(startNode, endNode, graph);
                break;
            case "dfs":
                result = run_dfs(startNode, endNode, graph);
                break;
            case "dijkstra":
                result = run_dijkstra(startNode,endNode, graph);
                break;
            default:
                console.warn("⚠️ Algorithm not implemented:", selectedAlgorithm);
                isRunning = false;
                return;
        }

        const { path, visited, cost } = handleWasmResult(result);
        // Animación mejorada
        if (visited && visited.length > 0) {
            const tempVisited = [];
            for (let i = 0; i < visited.length; i++) {
                tempVisited.push(visited[i]);
                visitedNodes = [...tempVisited];

                // Velocidad adaptable según cantidad de nodos
                const baseSpeed = visited.length > 100 ? 10 : 30;
                const speed = i < visited.length * 0.8 ? baseSpeed : baseSpeed * 2;
                await new Promise(resolve => setTimeout(resolve, speed));
            }

            // Pequeña pausa antes de mostrar el camino
            await new Promise(resolve => setTimeout(resolve, 200));

            // Animación del camino final (solo si hay camino)
            if (path && path.length > 0) {
                const tempPath = [];
                for (let i = 0; i < path.length; i++) {
                    tempPath.push(path[i]);
                    pathResult = [...tempPath];
                    await new Promise(resolve => setTimeout(resolve, 150));
                }
            }
        } else {
            // Si no hay datos de visitados, mostrar el camino directamente
            pathResult = path || [];
        }

        totalCost = cost || 0;
        
        // Algoritmo completado
        
    } catch (error) {
        console.error("❌ Error running", selectedAlgorithm, ":", error);
        console.error("Error details:", error.stack);
    } finally {
        isRunning = false;
    }
}

function reset() {
    startNode = null;
    endNode = null;
    pathResult = [];
    visitedNodes = [];
    totalCost = 0;
    isRunning = false;
    
}

</script>

<div class="grid grid-cols-[70%_30%] gap-4 h-[90vh] mt-5 items-center">
    <Map
        {graph}
        {geojson}
        {startNode}
        {endNode}
        {pathResult}
        {visitedNodes}
        on:click={handleMapClick}
    />

    <Sidebar
        {pathResult}
        {totalCost}
        {visitedNodes}
        {startNode}
        {endNode}
        {isRunning}
        on:runAlgorithm={runAlgorithm}
        on:reset={reset}
        bind:selectedAlgorithm
    />

    
</div>