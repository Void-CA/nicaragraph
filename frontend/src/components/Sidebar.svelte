<script>
// Props normales
export let pathResult = [];
export let totalCost = 0;
export let visitedNodes = [];
export let startNode = null;
export let endNode = null;
export let selectedAlgorithm = "a_star";
export let isRunning = false;

// ✅ Crear dispatcher para eventos personalizados
import { createEventDispatcher } from 'svelte';
const dispatch = createEventDispatcher();

function handleRunAlgorithm() {
    dispatch('runAlgorithm'); // ✅ Emitir evento sin datos
}

function handleReset() {
    dispatch('reset'); // ✅ Emitir evento sin datos
}
</script>

<div class="bg-white p-6 rounded-lg shadow-lg h-full overflow-y-auto">
    <h2 class="text-2xl font-bold mb-4">Panel de Control</h2>
    
    <!-- Debug info -->
    <div class="mb-4 p-3 bg-gray-100 rounded">
        <h3 class="font-semibold">Estado actual:</h3>
        <p>Inicio: {startNode || 'No seleccionado'}</p>
        <p>Fin: {endNode || 'No seleccionado'}</p>
        <p>Nodos visitados: {visitedNodes.length}</p>
        <p>Camino encontrado: {pathResult.length} nodos</p>
    </div>
    
    <div class="mb-4">
        <label class="block text-sm font-medium mb-2">Algoritmo:</label>
        <select bind:value={selectedAlgorithm} class="w-full p-2 border rounded">
            <option value="a_star">A*</option>
            <option value="dijkstra">Dijkstra</option>
            <option value="bfs">BFS</option>
        </select>
    </div>
    
    <button 
        on:click={handleRunAlgorithm}
        disabled={isRunning || !startNode || (selectedAlgorithm !== 'bfs' && !endNode)}
        class="w-full bg-blue-500 text-white p-2 rounded disabled:bg-gray-400 mb-2"
    >
        {isRunning ? 'Ejecutando...' : 'Ejecutar Algoritmo'}
    </button>
    
    <button 
        on:click={handleReset}
        class="w-full bg-gray-500 text-white p-2 rounded"
    >
        Reiniciar
    </button>
    
    <!-- Mostrar resultados -->
    {#if pathResult.length > 0}
    <div class="mt-4 p-3 bg-green-50 rounded">
        <h3 class="font-semibold">Resultados:</h3>
        <p>Coste total: {totalCost}</p>
        <p>Longitud del camino: {pathResult.length} nodos</p>
    </div>
    {/if}
</div>