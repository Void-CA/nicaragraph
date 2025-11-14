<script>
import { onMount } from "svelte";
import Map from "./components/Map.svelte";
import Sidebar from "./components/Sidebar.svelte";

import init, { run_astar } from "../pkg/nicaragua_graph.js";

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

// ✅ CORREGIDO: Recibir solo el ID, no el evento
function handleMapClick(event) {
    const clickedId = event.detail; // ✅ Ahora event.detail contiene el ID
    if (isRunning) return;
    
    if (!startNode) {
        startNode = clickedId;
    } else if (!endNode && clickedId !== startNode) {
        endNode = clickedId;
        console.log("End node set:", clickedId);
    } else {
        startNode = clickedId;
        endNode = null;
        pathResult = [];
        visitedNodes = [];
        totalCost = 0;
    }
}

// Ejecutar algoritmo con validaciones mejoradas
async function runAlgorithm() {
    if (!wasmInitialized || !graph) {
        console.error("WASM or graph not initialized");
        return;
    }

    if (!startNode) {
        console.error("No start node selected");
        return;
    }

    if (selectedAlgorithm !== "bfs" && !endNode) {
        console.error("No end node selected for this algorithm");
        return;
    }


    // ✅ Validar que los nodos existen en el grafo
    if (!graph.nodes.find(node => node.id === startNode)) {
        console.error("Start node not found in graph:", startNode);
        return;
    }

    if (endNode && !graph.nodes.find(node => node.id === endNode)) {
        console.error("End node not found in graph:", endNode);
        return;
    }


    if (isRunning) return;
    isRunning = true;

    try {
        // Reset resultados anteriores
        pathResult = [];
        visitedNodes = [];
        totalCost = 0;

        console.log("Running A* from", startNode, "to", endNode);
        
        let result;
        switch (selectedAlgorithm) {
            case "a_star":
                // ✅ Pasar los IDs correctamente validados
                result = run_astar(startNode, endNode, graph);
                break;
            default:
                console.warn("Algorithm not implemented:", selectedAlgorithm);
                return;
        }

        console.log("Algorithm raw result:", result);

        if (!result || !Array.isArray(result)) {
            console.error("Invalid result format from algorithm");
            return;
        }

        // ✅ Manejar diferentes estructuras de resultado
        const path = Array.isArray(result[0]) ? result[0] : [];
        const cost = typeof result[1] === 'number' ? result[1] : 0;
        const visited = Array.isArray(result[2]) ? result[2] : [];

        console.log("Processed - Path:", path, "Cost:", cost, "Visited:", visited.length);

        // Animar nodos visitados
        if (visited && visited.length > 0) {
            const tempVisited = [];
            for (let i = 0; i < visited.length; i++) {
                tempVisited.push(visited[i]);
                visitedNodes = [...tempVisited];
                await new Promise(resolve => setTimeout(resolve, 30)); // Más rápido para debug
            }
        }

        pathResult = path;
        totalCost = cost;
        
        console.log("Algorithm completed successfully");
        
    } catch (error) {
        console.error("Error running algorithm:", error);
        console.error("Error details:", {
            startNode,
            endNode, 
            graphKeys: Object.keys(graph).length,
            wasmInitialized
        });
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
    console.log("Reset complete");
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