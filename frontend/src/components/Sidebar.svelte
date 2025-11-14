<!-- src/components/Sidebar.svelte -->
<script>
export let selectedNodes = [];
export let pathResult = [];
</script>

<div class="flex flex-col h-full border rounded shadow-lg p-4 bg-white mr-10">
    <h3 class="text-xl font-semibold mb-3">Selección de municipios</h3>
    <div class="flex-1 overflow-y-auto">
        {#if pathResult.length === 0}
            <p class="text-gray-500">No hay municipios seleccionados</p>
        {:else}
            <ul class="space-y-2">
                {#each selectedNodes as node}
                    <li class="p-2 border rounded bg-gray-50 hover:bg-gray-100">
                        <strong>{node.name}</strong> ({node.dept})<br>
                        <span class="text-gray-600 text-sm">
                            Lat: {node.lat.toFixed(3)}, Lon: {node.lon.toFixed(3)}
                        </span>
                    </li>
                {/each}
            </ul>
        {/if}
    </div>
</div>
