<script>
import { onMount, onDestroy } from "svelte";
import * as d3 from "d3";
import Tooltip from "./Tooltip.svelte";

export let graph;
export let geojson;
export let pathResult = [];

let svg;
let startNode = null;
let endNode = null;

let tooltipContent = "";
let tooltipX = 0;
let tooltipY = 0;
let tooltipVisible = false;

let resizeObserver;

function drawMap() {
    if (!graph || !geojson || !svg) return;

    const { width, height } = svg.getBoundingClientRect();
    if (width === 0 || height === 0) return;

    d3.select(svg).selectAll("*").remove();
    const svgEl = d3.select(svg).attr("viewBox", `0 0 900 800`).attr("preserveAspectRatio", "xMidYMid meet");
    const g = svgEl.append("g");

    const projection = d3.geoMercator().fitSize([width, height], geojson);
    const pathGen = d3.geoPath().projection(projection);

    g.selectAll("path")
        .data(geojson.features)
        .join("path")
        .attr("d", pathGen)
        .attr("stroke", "#333")
        .attr("stroke-width", 0.5)
        .attr("fill", d => {
            if (d.properties.GID_2 === startNode) return "green";
            if (d.properties.GID_2 === endNode) return "red";
            if (pathResult.includes(d.properties.GID_2)) return "yellow";
            return "#cce5df";
        })
        .style("cursor", "pointer")
        .on("mouseover", (event, d) => {
            tooltipContent = d.properties.NAME_2;
            tooltipVisible = true;
            d3.select(event.currentTarget).attr("fill", "lightblue");
        })
        .on("mousemove", (event) => {
            tooltipX = event.pageX + 10;
            tooltipY = event.pageY + 10;
        })
        .on("mouseout", (event, d) => {
            tooltipVisible = false;
            d3.select(event.currentTarget)
                .attr("fill", d => {
                    if (d.properties.GID_2 === startNode) return "green";
                    if (d.properties.GID_2 === endNode) return "red";
                    if (pathResult.includes(d.properties.GID_2)) return "yellow";
                    return "#cce5df";
                });
        })
        .on("click", (event, d) => {
            const clickedId = d.properties.GID_2;
            if (!startNode) startNode = clickedId;
            else if (!endNode) endNode = clickedId;
            else { startNode = clickedId; endNode = null; }
            pathResult = [startNode];
            if (endNode) pathResult.push(endNode);
            drawMap(); // actualizar colores
        });

    // Zoom
    const zoom = d3.zoom().scaleExtent([1, 8]).on("zoom", (event) => g.attr("transform", event.transform));
    svgEl.call(zoom);
}

onMount(() => {
    const tryDraw = setInterval(() => {
        if (graph && geojson && svg) {
            drawMap();
            clearInterval(tryDraw);
        }
    }, 50);

    resizeObserver = new ResizeObserver(() => drawMap());
    resizeObserver.observe(svg);
});

onDestroy(() => resizeObserver.disconnect());
</script>

<svg bind:this={svg} class="w-full h-full block rounded-lg shadow-lg"></svg>
<Tooltip content={tooltipContent} visible={tooltipVisible} x={tooltipX} y={tooltipY} />
