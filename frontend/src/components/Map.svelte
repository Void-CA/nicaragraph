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

// Paleta de colores profesional
const colors = {
    background: "#f8fafc",
    default: "#e2e8f0",
    defaultHover: "#cbd5e1",
    border: "#cbd5e1",
    start: "#059669",      // Verde esmeralda
    end: "#dc2626",        // Rojo vibrante
    path: "#d97706",       // Ámbar
    visited: "#2563eb",    // Azul
    visitedLight: "#3b82f6",
    text: "#1e293b",
    textLight: "#64748b"
};

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

    // Limpiar SVG existente
    d3.select(svg).selectAll("*").remove();
    
    // Crear contenedor principal con fondo
    const svgEl = d3.select(svg)
        .attr("viewBox", `0 0 900 800`)
        .attr("preserveAspectRatio", "xMidYMid meet")
        .style("background", colors.background);

    // Grupo principal para el mapa
    const g = svgEl.append("g")
        .attr("class", "map-container");

    // Configurar proyección
    projection = d3.geoMercator().fitSize([width * 0.95, height * 0.95], geojson);
    pathGen = d3.geoPath().projection(projection);

    // Dibujar municipios
    const municipalities = g.selectAll("path")
        .data(geojson.features)
        .join("path")
        .attr("d", pathGen)
        .attr("stroke", colors.border)
        .attr("stroke-width", 0.8)
        .attr("stroke-linejoin", "round")
        .attr("fill", d => {
            const id = d.properties.GID_2;
            
            // Jerarquía de colores mejorada
            if (id === startNode) return colors.start;
            if (id === endNode) return colors.end;
            if (pathResult.includes(id)) return colors.path;
            if (partial.includes(id)) return colors.visited;
            return colors.default;
        })
        .attr("opacity", d => {
            const id = d.properties.GID_2;
            if (pathResult.includes(id) || id === startNode || id === endNode) return 1;
            if (partial.includes(id)) return 0.9;
            return 0.7;
        })
        .style("cursor", "pointer")
        .style("transition", "all 0.2s ease")
        .on("click", (event, d) => {
            const clickedId = d.properties.GID_2;
            console.log("Municipio seleccionado:", clickedId, d.properties.NAME_2);
            dispatch('click', clickedId);
        })
        .on("mouseover", function(event, d) {
            const id = d.properties.GID_2;
            let status = "No visitado";
            let statusColor = "#64748b";
            
            if (id === startNode) {
                status = "Nodo de inicio";
                statusColor = "#059669";
            } else if (id === endNode) {
                status = "Nodo destino";
                statusColor = "#dc2626";
            } else if (pathResult.includes(id)) {
                status = "En camino óptimo";
                statusColor = "#d97706";
            } else if (visitedNodes.includes(id)) {
                status = "Nodo explorado";
                statusColor = "#2563eb";
            }
            
            // Efecto hover
            d3.select(this)
                .attr("stroke-width", 2)
                .attr("stroke", statusColor)
                .attr("filter", "url(#glow)");
            
            tooltipContent = `
                <div class="font-semibold text-gray-800 mb-1">${d.properties.NAME_2}</div>
                <div class="text-sm mb-2" style="color: ${statusColor}; font-weight: 500">${status}</div>
                <div class="text-xs text-gray-500 border-t border-gray-100 pt-2">
                    <div>ID: <span class="font-mono">${d.properties.GID_2}</span></div>
                    <div class="mt-1">Haz clic para seleccionar</div>
                </div>
            `;
            tooltipVisible = true;
        })
        .on("mousemove", event => {
            tooltipX = event.pageX + 15;
            tooltipY = event.pageY + 15;
        })
        .on("mouseout", function() {
            // Restaurar estilo original
            d3.select(this)
                .attr("stroke-width", 0.8)
                .attr("stroke", colors.border)
                .attr("filter", null);
            
            tooltipVisible = false;
        });

    // Agregar efectos de filtro para hover
    const defs = svgEl.append("defs");
    const filter = defs.append("filter")
        .attr("id", "glow")
        .attr("height", "300%")
        .attr("width", "300%")
        .attr("x", "-100%")
        .attr("y", "-100%");

    filter.append("feGaussianBlur")
        .attr("stdDeviation", "2")
        .attr("result", "coloredBlur");

    const feMerge = filter.append("feMerge");
    feMerge.append("feMergeNode")
        .attr("in", "coloredBlur");
    feMerge.append("feMergeNode")
        .attr("in", "SourceGraphic");

    // Agregar leyenda
    addLegend(g, width, height);

    // Configurar zoom con controles mejorados
    const zoom = d3.zoom()
        .scaleExtent([1, 12])
        .translateExtent([[0, 0], [width, height]])
        .on("zoom", (event) => {
            g.attr("transform", event.transform);
        });

    svgEl.call(zoom)
        .call(zoom.transform, d3.zoomIdentity);

    // Agregar controles de zoom
    addZoomControls(svgEl, zoom, width, height);
}

function addLegend(g, width, height) {
    const legend = g.append("g")
        .attr("class", "legend")
        .attr("transform", `translate(20, 20)`);

    const legendItems = [
        { color: colors.start, text: "Nodo Inicio" },
        { color: colors.end, text: "Nodo Destino" },
        { color: colors.path, text: "Camino Óptimo" },
        { color: colors.visited, text: "Nodos Explorados" },
        { color: colors.default, text: "No Visitado" }
    ];

    legendItems.forEach((item, i) => {
        const itemGroup = legend.append("g")
            .attr("transform", `translate(0, ${i * 25})`);

        itemGroup.append("rect")
            .attr("width", 16)
            .attr("height", 16)
            .attr("fill", item.color)
            .attr("rx", 3)
            .attr("stroke", colors.border)
            .attr("stroke-width", 0.5);

        itemGroup.append("text")
            .attr("x", 24)
            .attr("y", 12)
            .attr("fill", colors.text)
            .attr("font-size", "12px")
            .attr("font-family", "system-ui, sans-serif")
            .text(item.text);
    });

    // Fondo de la leyenda
    legend.insert("rect", ":first-child")
        .attr("width", 180)
        .attr("transform", `translate(-30, -5)`)
        .attr("text-anchor", "middle")
        .attr("height", legendItems.length * 25)
        .attr("fill", "white")
        .attr("opacity", 0.9)
        .attr("rx", 8)
        .attr("stroke", colors.border)
        .attr("stroke-width", 1);
}

function addZoomControls(svgEl, zoom, width, height) {
    const controls = svgEl.append("g")
        .attr("class", "zoom-controls")
        .attr("transform", `translate(${width - 80}, 20)`);

    // Fondo de controles
    controls.append("rect")
        .attr("width", 50)
        .attr("height", 80)
        .attr("fill", "white")
        .attr("opacity", 0.9)
        .attr("rx", 8)
        .attr("stroke", colors.border)
        .attr("stroke-width", 1);

    // Botón zoom in
    const zoomIn = controls.append("g")
        .attr("transform", "translate(10, 15)")
        .style("cursor", "pointer")
        .on("click", () => {
            svgEl.transition().duration(250).call(zoom.scaleBy, 1.5);
        });

    zoomIn.append("rect")
        .attr("width", 30)
        .attr("height", 25)
        .attr("fill", "transparent");

    zoomIn.append("path")
        .attr("d", "M5,12 L25,12 M15,5 L15,19")
        .attr("stroke", colors.text)
        .attr("stroke-width", 2)
        .attr("stroke-linecap", "round");

    // Botón zoom out
    const zoomOut = controls.append("g")
        .attr("transform", "translate(10, 45)")
        .style("cursor", "pointer")
        .on("click", () => {
            svgEl.transition().duration(250).call(zoom.scaleBy, 0.75);
        });

    zoomOut.append("rect")
        .attr("width", 30)
        .attr("height", 25)
        .attr("fill", "transparent");

    zoomOut.append("path")
        .attr("d", "M5,12 L25,12")
        .attr("stroke", colors.text)
        .attr("stroke-width", 2)
        .attr("stroke-linecap", "round");

    // Botón reset
    const reset = controls.append("g")
        .attr("transform", "translate(10, 75)")
        .style("cursor", "pointer")
        .on("click", () => {
            svgEl.transition().duration(500).call(zoom.transform, d3.zoomIdentity);
        });

    reset.append("rect")
        .attr("width", 30)
        .attr("height", 25)
        .attr("fill", "transparent");

    reset.append("path")
        .attr("d", "M8,8 L22,22 M22,8 L8,22")
        .attr("stroke", colors.text)
        .attr("stroke-width", 2)
        .attr("stroke-linecap", "round");

    // Efectos hover para controles
    controls.selectAll("g")
        .on("mouseover", function() {
            d3.select(this).select("path")
                .attr("stroke", colors.visited);
        })
        .on("mouseout", function() {
            d3.select(this).select("path")
                .attr("stroke", colors.text);
        });
}

onMount(() => {
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

<svg 
    bind:this={svg} 
    class="w-full h-full block rounded-xl shadow-sm border border-gray-200 bg-white"
></svg>

<Tooltip 
    content={tooltipContent} 
    visible={tooltipVisible} 
    x={tooltipX} 
    y={tooltipY} 
/>

<style>
:global(.map-container path) {
    transition: all 0.2s ease;
}

:global(.zoom-controls g:hover) {
    transform: scale(1.05);
    transition: transform 0.2s ease;
}

:global(.legend text) {
    font-family: system-ui, -apple-system, sans-serif;
    user-select: none;
}
</style>