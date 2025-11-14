<script>
import { onMount } from "svelte";
import Map from "./components/Map.svelte";
  import Sidebar from "./components/Sidebar.svelte";

let graph = null;
let geojson = null;
let pathResult = []; // IDs seleccionados (inicio/fin)
let selectedNodes = [];

onMount(async () => {
    const graphResponse = await fetch("/src/data/nicaragua_graph.json");
    graph = await graphResponse.json();

    const geoResponse = await fetch("/src/data/nicaragua_municipios_simplificado.geojson");
    geojson = await geoResponse.json();
});

// Actualizar array de nodos seleccionados
$: if (graph && pathResult) {
    selectedNodes = pathResult.map(id => graph.nodes.find(n => n.id === id));
}

</script>

<div class="grid grid-cols-[70%_30%] gap-4 h-[90vh] mt-5 items-center">
    <Map {graph} {geojson} bind:pathResult />
    <Sidebar {selectedNodes} {pathResult} />
</div>