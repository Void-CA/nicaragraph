<script>
import { onMount, onDestroy } from "svelte";
import * as d3 from "d3";
import Tooltip from "./Tooltip.svelte";

export let graph;
export let geojson;
export let startNode = null;
export let endNode = null;
export let pathResult = [];
export let visitedNodes = [];

// ✅ Importar dispatcher para eventos
import { createEventDispatcher } from 'svelte';
const dispatch = createEventDispatcher();

export function animateVisited(delay = 50) {
    if (!visitedNodes.length) return;
    
    const partial = [];
    let index = 0;
    
    const animateStep = async () => {
        if (index < visitedNodes.length) {
            partial.push(visitedNodes[index]);
            drawMap(partial);
            index++;
            setTimeout(animateStep, delay);
        } else if (pathResult.length > 0) {
            drawMap(visitedNodes);
        }
    };
    
    animateStep();
}

let svg;
let projection;
let pathGen;

let tooltipContent = "";
let tooltipX = 0;
let tooltipY = 0;
let tooltipVisible = false;
let resizeObserver;

// ✅ Reactividad para redibujar cuando cambian las props
$: if (graph && geojson && svg) {
    drawMap();
}

$: startNode, redrawOnNodeChange();
$: endNode, redrawOnNodeChange();
$: pathResult, redrawOnNodeChange();

function redrawOnNodeChange() {
    if (graph && geojson && svg) {
        drawMap(visitedNodes);
    }
}

function drawMap(partial = []) {
    if (!graph || !geojson || !svg) return;

    const { width, height } = svg.getBoundingClientRect();
    if (width === 0 || height === 0) return;

    d3.select(svg).selectAll("*").remove();
    const svgEl = d3.select(svg)
        .attr("viewBox", `0 0 900 800`)
        .attr("preserveAspectRatio", "xMidYMid meet");

    const g = svgEl.append("g");

    projection = d3.geoMercator().fitSize([width, height], geojson);
    pathGen = d3.geoPath().projection(projection);

    g.selectAll("path")
        .data(geojson.features)
        .join("path")
        .attr("d", pathGen)
        .attr("stroke", "#333")
        .attr("stroke-width", 0.5)
        .attr("fill", d => {
            const id = d.properties.GID_2;
            
            // ✅ MEJOR JERARQUÍA DE COLORES PARA LA ANIMACIÓN
            if (id === startNode) return "#10B981"; // Verde brillante - inicio
            if (id === endNode) return "#EF4444";   // Rojo brillante - fin
            if (pathResult.includes(id)) return "#F59E0B"; // Amarillo/naranja - camino final
            if (partial.includes(id)) return "#3B82F6";    // Azul brillante - nodos visitados
            return "#E5E7EB"; // Gris claro - no visitado
        })
        .attr("opacity", d => {
            const id = d.properties.GID_2;
            // ✅ Dar más opacidad a los nodos importantes
            if (pathResult.includes(id) || id === startNode || id === endNode) return 1;
            if (partial.includes(id)) return 0.8;
            return 0.6;
        })
        .style("cursor", "pointer")
        .on("click", (event, d) => {
            const clickedId = d.properties.GID_2;
            console.log("Municipio clickeado:", clickedId, d.properties.NAME_2);
            dispatch('click', clickedId);
        })
        .on("mouseover", (event, d) => {
            const id = d.properties.GID_2;
            let status = "No visitado";
            if (id === startNode) status = "Nodo de inicio";
            else if (id === endNode) status = "Nodo final";
            else if (pathResult.includes(id)) status = "En el camino";
            else if (visitedNodes.includes(id)) status = "Visitado";
            
            tooltipContent = `${d.properties.NAME_2} (${status})`;
            tooltipVisible = true;
        })
        .on("mousemove", event => {
            tooltipX = event.pageX + 10;
            tooltipY = event.pageY + 10;
        })
        .on("mouseout", () => {
            tooltipVisible = false;
        });

    const zoom = d3.zoom().scaleExtent([1, 8]).on("zoom", (event) => g.attr("transform", event.transform));
    svgEl.call(zoom);
}

onMount(() => {
    // ✅ Manejo más robusto del ResizeObserver
    if (svg) {
        resizeObserver = new ResizeObserver(() => {
            if (graph && geojson) {
                drawMap(visitedNodes);
            }
        });
        resizeObserver.observe(svg);
    }
});

onDestroy(() => {
    if (resizeObserver) {
        resizeObserver.disconnect();
    }
});
</script>

<svg bind:this={svg} class="w-full h-full block rounded-lg shadow-lg"></svg>
<Tooltip content={tooltipContent} visible={tooltipVisible} x={tooltipX} y={tooltipY} />