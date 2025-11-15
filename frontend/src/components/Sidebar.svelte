<script>
// Props normales
export let pathResult = [];
export let totalCost = 0;
export let visitedNodes = [];
export let startNodeName = null;
export let endNodeName = null;
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
    `Ejecutando ${getAlgorithmDisplayName(selectedAlgorithm)} - ${visitedNodes.length} nodos procesados` : 
    pathResult.length > 0 ? `${getAlgorithmDisplayName(selectedAlgorithm)} completado - ${pathResult.length} pasos` : 
    'Selecciona algoritmo y nodos para comenzar';

// Información sobre algoritmos
$: algorithmInfo = {
    a_star: "Algoritmo A* - Encuentra el camino más corto usando heurística de distancia euclidiana",
    dijkstra: "Algoritmo de Dijkstra - Calcula el camino más corto considerando todos los pesos", 
    bfs: "Búsqueda en Amplitud - Explora sistemáticamente por niveles",
    dfs: "Búsqueda en Profundidad - Explora ramas completas antes de retroceder"
};

// Nombres display para algoritmos
function getAlgorithmDisplayName(algo) {
    const names = {
        a_star: "A* Search",
        dijkstra: "Dijkstra",
        bfs: "BFS",
        dfs: "DFS"
    };
    return names[algo] || algo;
}

// Texto del botón dinámico
$: buttonText = isRunning ? 
    'Procesando...' : 
    `Ejecutar ${getAlgorithmDisplayName(selectedAlgorithm)}`;

// Calcular eficiencia
$: efficiency = visitedNodes.length > 0 && pathResult.length > 0 ? 
    ((pathResult.length / visitedNodes.length) * 100).toFixed(1) : 0;
</script>

<div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100 h-full overflow-y-auto">
    <!-- Header -->
    <div class="mb-6 pb-4 border-b border-gray-200">
        <h2 class="text-xl font-semibold text-gray-800">Panel de Control de Algoritmos</h2>
        <p class="text-sm text-gray-500 mt-1">Visualización y ejecución de algoritmos de búsqueda en grafos</p>
    </div>
    
    <!-- Indicador de Estado -->
    <div class="mb-6 p-4 bg-gradient-to-r from-blue-50 to-indigo-50 rounded-lg border border-blue-100">
        <div class="flex justify-between items-center text-sm text-blue-800 mb-2">
            <span class="font-medium">{statusText}</span>
            <span class="font-semibold">{Math.round(progressPercentage)}%</span>
        </div>
        <div class="w-full bg-blue-100 rounded-full h-2.5">
            <div 
                class="bg-gradient-to-r from-blue-500 to-indigo-600 h-2.5 rounded-full transition-all duration-500 ease-out" 
                style="width: {progressPercentage}%"
            ></div>
        </div>
        {#if isRunning}
        <div class="text-xs text-blue-600 mt-2 flex items-center">
            <div class="w-2 h-2 bg-blue-500 rounded-full mr-2 animate-pulse"></div>
            <span>Algoritmo activo: <strong>{getAlgorithmDisplayName(selectedAlgorithm)}</strong></span>
        </div>
        {/if}
    </div>
    
    <!-- Configuración del Algoritmo -->
    <div class="mb-6">
        <label class="block text-sm font-medium text-gray-700 mb-3">Selección de Algoritmo</label>
        <select 
            bind:value={selectedAlgorithm} 
            class="w-full px-3 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors disabled:bg-gray-100 disabled:cursor-not-allowed"
            disabled={isRunning}
        >
            <option value="a_star">A* Search Algorithm</option>
            <option value="dijkstra">Dijkstra Algorithm</option>
            <option value="bfs">Breadth-First Search</option>
            <option value="dfs">Depth-First Search</option>
        </select>
        <p class="text-xs text-gray-500 mt-2 leading-relaxed">
            {algorithmInfo[selectedAlgorithm]}
        </p>
    </div>
    
    <!-- Nodos Seleccionados -->
    <div class="mb-6 p-4 bg-gray-50 rounded-lg border border-gray-200">
        <h3 class="font-medium text-gray-700 text-sm mb-3">Configuración de Nodos</h3>
        <div class="space-y-2.5">
            <div class="flex items-center justify-between">
                <div class="flex items-center">
                    <div class="w-3 h-3 bg-green-500 rounded-full mr-3"></div>
                    <span class="text-sm text-gray-600">Nodo inicial</span>
                </div>
                <span class="text-sm font-mono text-gray-800 bg-white px-2 py-1 rounded border">
                    {startNodeName || 'No seleccionado'}
                </span>
            </div>
            <div class="flex items-center justify-between">
                <div class="flex items-center">
                    <div class="w-3 h-3 bg-red-500 rounded-full mr-3"></div>
                    <span class="text-sm text-gray-600">Nodo destino</span>
                </div>
                <span class="text-sm font-mono text-gray-800 bg-white px-2 py-1 rounded border">
                    {endNodeName || 'No seleccionado'}
                </span>
            </div>
        </div>
    </div>
    
    <!-- Controles de Ejecución -->
    <div class="mb-6 space-y-3">
        <button 
            on:click={handleRunAlgorithm}
            disabled={isRunning || !startNodeName || (selectedAlgorithm !== 'dijkstra' && !endNodeName)}
            class="w-full bg-gradient-to-r from-blue-600 to-blue-700 hover:from-blue-700 hover:to-blue-800 text-white py-3 px-4 rounded-lg font-medium disabled:from-gray-400 disabled:to-gray-500 disabled:cursor-not-allowed transition-all duration-200 shadow-sm hover:shadow-md"
        >
            {#if isRunning}
            <div class="flex items-center justify-center">
                <div class="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin mr-2"></div>
                {buttonText}
            </div>
            {:else}
            <div class="flex items-center justify-center">
                <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/>
                </svg>
                {buttonText}
            </div>
            {/if}
        </button>
        
        <button 
            on:click={handleReset}
            disabled={isRunning}
            class="w-full bg-white border border-gray-300 hover:bg-gray-50 text-gray-700 py-2.5 px-4 rounded-lg font-medium disabled:bg-gray-100 disabled:text-gray-400 disabled:cursor-not-allowed transition-colors duration-200"
        >
            <div class="flex items-center justify-center">
                <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
                </svg>
                Reiniciar Configuración
            </div>
        </button>
    </div>
    
    <!-- Resultados y Estadísticas -->
    {#if pathResult.length > 0}
    <div class="mb-4 p-4 bg-green-50 rounded-lg border border-green-200">
        <div class="flex items-center mb-3">
            <svg class="w-5 h-5 text-green-600 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
            </svg>
            <h3 class="font-semibold text-green-800">{getAlgorithmDisplayName(selectedAlgorithm)} - Resultados</h3>
        </div>
        <div class="grid grid-cols-2 gap-3 text-sm">
            <div class="bg-white rounded p-2 border border-green-100">
                <div class="text-green-600 font-medium">Coste Total</div>
                <div class="text-green-800 font-mono text-lg">{totalCost.toFixed(2)}</div>
            </div>
            <div class="bg-white rounded p-2 border border-green-100">
                <div class="text-green-600 font-medium">Longitud</div>
                <div class="text-green-800 font-mono text-lg">{pathResult.length} nodos</div>
            </div>
        </div>
    </div>
    {:else if !isRunning && visitedNodes.length > 0}
    <div class="mb-4 p-4 bg-yellow-50 rounded-lg border border-yellow-200">
        <div class="flex items-center mb-2">
            <svg class="w-5 h-5 text-yellow-600 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L4.082 16.5c-.77.833.192 2.5 1.732 2.5z"/>
            </svg>
            <h3 class="font-semibold text-yellow-800">Camino No Encontrado</h3>
        </div>
        <p class="text-yellow-700 text-sm">El algoritmo no pudo encontrar una ruta entre los nodos seleccionados</p>
    </div>
    {/if}
    
    <!-- Métricas de Rendimiento -->
    {#if visitedNodes.length > 0}
    <div class="p-4 bg-gradient-to-br from-gray-50 to-blue-50 rounded-lg border border-gray-200">
        <h3 class="font-medium text-gray-700 text-sm mb-3">Métricas de Rendimiento</h3>
        <div class="grid grid-cols-2 gap-4 text-xs">
            <div class="text-center">
                <div class="text-2xl font-bold text-blue-600 font-mono">{visitedNodes.length}</div>
                <div class="text-gray-500 mt-1">Nodos Explorados</div>
            </div>
            <div class="text-center">
                <div class="text-2xl font-bold text-green-600 font-mono">{pathResult.length}</div>
                <div class="text-gray-500 mt-1">Camino Encontrado</div>
            </div>
        </div>
        {#if pathResult.length > 0}
        <div class="mt-3 pt-3 border-t border-gray-200">
            <div class="flex justify-between text-xs text-gray-600 mb-1">
                <span>Eficiencia del Algoritmo</span>
                <span>{efficiency}%</span>
            </div>
            <div class="w-full bg-gray-200 rounded-full h-2">
                <div 
                    class="bg-gradient-to-r from-green-400 to-green-500 h-2 rounded-full transition-all duration-500" 
                    style="width: {efficiency}%"
                ></div>
            </div>
        </div>
        {/if}
    </div>
    {/if}
</div>

<style>
/* Animaciones suaves */
button {
    transition: all 0.2s ease-in-out;
}

/* Mejora la legibilidad */
.font-mono {
    font-feature-settings: "tnum";
    font-variant-numeric: tabular-nums;
}

/* Efectos hover mejorados */
.hover-lift:hover {
    transform: translateY(-1px);
}
</style>