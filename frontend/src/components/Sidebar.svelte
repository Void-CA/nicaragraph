<script>
// Props normales
export let pathResult = [];
export let totalCost = 0;
export let visitedNodes = [];
export let startNode = null;
export let endNode = null;
export let selectedAlgorithm = "a_star";
export let isRunning = false;

// Event dispatcher
import { createEventDispatcher } from 'svelte';
const dispatch = createEventDispatcher();

function handleRunAlgorithm() {
    dispatch('runAlgorithm');
}

function handleReset() {
    dispatch('reset');
}

// Estado local para mostrar progreso
let currentStep = 0;
let totalSteps = 0;

// Reactividad para progreso
$: if (isRunning) {
    currentStep = visitedNodes.length;
    totalSteps = Math.max(visitedNodes.length, pathResult.length);
}

$: progressPercentage = totalSteps > 0 ? (currentStep / totalSteps) * 100 : 0;

$: statusText = isRunning ? 
    `${selectedAlgorithm.toUpperCase()} ejecutándose... ${visitedNodes.length} nodos` : 
    pathResult.length > 0 ? `${selectedAlgorithm} completado: ${pathResult.length} pasos` : 
    'Selecciona algoritmo y nodos';

// ✅ Información sobre algoritmos - MOVIDO AQUÍ al script
$: algorithmInfo = {
    a_star: "A* - Camino más corto con heurística (Distancia Euclidiana fue la usada aquí)",
    dijkstra: "Dijkstra - Camino más corto desde el inicio a todos los nodos", 
    bfs: "BFS - Explora por niveles (sin pesos)",
    dfs: "DFS - Explora en profundidad (sin pesos)"
};

// ✅ Texto del botón dinámico
$: buttonText = isRunning ? 
    '   Ejecutando...' : 
    `Ejecutar ${selectedAlgorithm.toUpperCase()}`;
</script>

<div class="bg-white p-6 rounded-lg shadow-lg h-full overflow-y-auto">
    <h2 class="text-2xl font-bold mb-4">Panel de Control</h2>
    
    <!-- Indicador de progreso -->
    <div class="mb-4 p-3 bg-blue-50 rounded-lg border border-blue-200">
        <div class="flex justify-between text-sm text-blue-800 mb-1">
            <span class="font-semibold">{statusText}</span>
            <span>{Math.round(progressPercentage)}%</span>
        </div>
        <div class="w-full bg-blue-200 rounded-full h-2">
            <div 
                class="bg-blue-600 h-2 rounded-full transition-all duration-300" 
                style="width: {progressPercentage}%"
            ></div>
        </div>
        {#if isRunning}
        <div class="text-xs text-blue-600 mt-1">
            Algoritmo: <strong>{selectedAlgorithm.toUpperCase()}</strong>
        </div>
        {/if}
    </div>
    
    <!-- Selector de algoritmo -->
    <div class="mb-4">
        <label class="block text-sm font-medium mb-2">Algoritmo:</label>
        <select 
            bind:value={selectedAlgorithm} 
            class="w-full p-2 border rounded"
            disabled={isRunning}
        >
            <option value="a_star">A* Search</option>
            <option value="dijkstra">Dijkstra</option>
            <option value="bfs">Breadth-First Search</option>
            <option value="dfs">Depth-First Search</option>
        </select>
        <p class="text-xs text-gray-500 mt-1">
            {algorithmInfo[selectedAlgorithm]}
        </p>
    </div>
    
    <!-- Información de nodos seleccionados -->
    <div class="mb-4 p-3 bg-gray-50 rounded">
        <h3 class="font-semibold text-sm mb-2">Nodos seleccionados:</h3>
        <div class="text-sm">
            <div class="flex items-center mb-1">
                <div class="w-3 h-3 bg-green-500 rounded-full mr-2"></div>
                <span>Inicio: {startNode || 'No seleccionado'}</span>
            </div>
            <div class="flex items-center">
                <div class="w-3 h-3 bg-red-500 rounded-full mr-2"></div>
                <span>Fin: {endNode || 'No seleccionado'}</span>
            </div>
        </div>
    </div>
    
    <!-- Botón de ejecución -->
    <button 
        on:click={handleRunAlgorithm}
        disabled={isRunning || !startNode || (selectedAlgorithm !== 'dijkstra' && !endNode)}
        class="w-full bg-blue-500 hover:bg-blue-600 text-white p-2 rounded disabled:bg-gray-400 mb-2 transition-colors font-semibold"
    >
        {buttonText}
    </button>
    
    <!-- Botón de reinicio -->
    <button 
        on:click={handleReset}
        disabled={isRunning}
        class="w-full bg-gray-500 hover:bg-gray-600 text-white p-2 rounded disabled:bg-gray-400 transition-colors"
    >
        Reiniciar
    </button>
    
    <!-- Resultados -->
    {#if pathResult.length > 0}
    <div class="mt-4 p-3 bg-green-50 rounded border border-green-200">
        <h3 class="font-semibold text-green-800">{selectedAlgorithm.toUpperCase()} Completado:</h3>
        <p class="text-green-700">Coste total: <strong>{totalCost}</strong></p>
        <p class="text-green-700">Longitud: <strong>{pathResult.length}</strong> nodos</p>
        <p class="text-green-700">Explorados: <strong>{visitedNodes.length}</strong> nodos</p>
    </div>
    {:else if !isRunning && visitedNodes.length > 0}
    <!-- Mensaje cuando no se encontró camino -->
    <div class="mt-4 p-3 bg-yellow-50 rounded border border-yellow-200">
        <h3 class="font-semibold text-yellow-800">{selectedAlgorithm.toUpperCase()} Completado:</h3>
        <p class="text-yellow-700">No se encontró un camino</p>
        <p class="text-yellow-700">Nodos explorados: <strong>{visitedNodes.length}</strong></p>
    </div>
    {/if}
    
    <!-- Estadísticas adicionales -->
    {#if visitedNodes.length > 0}
    <div class="mt-4 p-3 bg-blue-50 rounded border border-blue-200">
        <h3 class="font-semibold text-blue-800 text-sm">Estadísticas:</h3>
        <div class="text-xs text-blue-700 space-y-1 mt-1">
            <div class="flex justify-between">
                <span>Nodos explorados:</span>
                <span class="font-mono">{visitedNodes.length}</span>
            </div>
            <div class="flex justify-between">
                <span>Longitud del camino:</span>
                <span class="font-mono">{pathResult.length}</span>
            </div>
            <div class="flex justify-between">
                <span>Eficiencia:</span>
                <span class="font-mono">
                    {visitedNodes.length > 0 ? ((pathResult.length / visitedNodes.length) * 100).toFixed(1) : 0}%
                </span>
            </div>
        </div>
    </div>
    {/if}
</div>