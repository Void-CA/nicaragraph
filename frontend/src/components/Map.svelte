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
            if (id === startNode) return "#4CAF50";
            if (id === endNode) return "#F44336";
            if (pathResult.includes(id)) return "#FFC107";
            if (partial.includes(id)) return "#42A5F5";
            return "#E0F7FA";
        })
        .style("cursor", "pointer")
        .on("click", (event, d) => {
            const clickedId = d.properties.GID_2;
            console.log("Municipio clickeado:", clickedId, d.properties.NAME_2);
            // ✅ CORREGIDO: Pasar solo el ID, no el evento
            dispatch('click', clickedId);
        })
        .on("mouseover", (event, d) => {
            tooltipContent = `${d.properties.NAME_2} (ID: ${d.properties.GID_2})`;
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