<script>
export let content = "";
export let visible = false;
export let x = 0;
export let y = 0;

// Estado para animación suave
let isMounted = false;

$: if (visible) {
    isMounted = true;
}

function handleTransitionEnd() {
    if (!visible) {
        isMounted = false;
    }
}
</script>

{#if isMounted}
<div 
    class:opacity-100={visible}
    class:opacity-0={!visible}
    class:translate-y-0={visible}
    class:translate-y-2={!visible}
    class="tooltip-container"
    style="left: {x}px; top: {y}px;"
    on:transitionend={handleTransitionEnd}
>
    {@html content}
    
</div>
{/if}

<style>
.tooltip-container {
    position: fixed;
    z-index: 50;
    background: linear-gradient(135deg, #ffffff 0%, #f8fafc 100%);
    color: #1e293b;
    padding: 12px 16px;
    border-radius: 12px;
    box-shadow: 
        0 10px 25px -5px rgba(0, 0, 0, 0.1),
        0 8px 10px -6px rgba(0, 0, 0, 0.05),
        0 0 0 1px rgba(0, 0, 0, 0.05);
    pointer-events: none;
    font-family: system-ui, -apple-system, sans-serif;
    font-size: 14px;
    line-height: 1.5;
    max-width: 280px;
    backdrop-filter: blur(8px);
    border: 1px solid rgba(255, 255, 255, 0.8);
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    transform-origin: bottom left;
}

.tooltip-arrow {
    position: absolute;
    width: 12px;
    height: 12px;
    background: linear-gradient(135deg, #ffffff 0%, #f8fafc 100%);
    border: 1px solid rgba(0, 0, 0, 0.05);
    border-top: none;
    border-left: none;
    transform: rotate(45deg);
    bottom: -6px;
    left: 20px;
    box-shadow: 2px 2px 3px -1px rgba(0, 0, 0, 0.05);
}

.opacity-100 {
    opacity: 1;
}

.opacity-0 {
    opacity: 0;
}

.translate-y-0 {
    transform: translateY(0) scale(1);
}

.translate-y-2 {
    transform: translateY(2px) scale(0.98);
}

/* Mejoras específicas para el contenido HTML */
:global(.tooltip-container strong) {
    font-weight: 600;
    color: #0f172a;
}

:global(.tooltip-container .text-sm) {
    font-size: 13px;
}

:global(.tooltip-container .text-xs) {
    font-size: 12px;
}

:global(.tooltip-container .font-semibold) {
    font-weight: 600;
}

:global(.tooltip-container .mt-1) {
    margin-top: 4px;
}

:global(.tooltip-container .text-gray-500) {
    color: #64748b;
}

:global(.tooltip-container .text-gray-800) {
    color: #1e293b;
}

/* Efecto de glassmorphism en navegadores modernos */
@supports (backdrop-filter: blur(8px)) {
    .tooltip-container {
        background: linear-gradient(135deg, rgba(255, 255, 255, 0.95) 0%, rgba(248, 250, 252, 0.95) 100%);
        backdrop-filter: blur(12px);
    }
    
    .tooltip-arrow {
        background: linear-gradient(135deg, rgba(255, 255, 255, 0.95) 0%, rgba(248, 250, 252, 0.95) 100%);
        backdrop-filter: blur(12px);
    }
}

/* Responsive para móviles */
@media (max-width: 768px) {
    .tooltip-container {
        max-width: 240px;
        font-size: 13px;
        padding: 10px 14px;
    }
}

/* Dark mode support */
@media (prefers-color-scheme: dark) {
    .tooltip-container {
        background: linear-gradient(135deg, #1e293b 0%, #334155 100%);
        color: #f1f5f9;
        border: 1px solid rgba(255, 255, 255, 0.1);
        box-shadow: 
            0 10px 25px -5px rgba(0, 0, 0, 0.3),
            0 8px 10px -6px rgba(0, 0, 0, 0.2);
    }
    
    .tooltip-arrow {
        background: linear-gradient(135deg, #1e293b 0%, #334155 100%);
        border: 1px solid rgba(255, 255, 255, 0.1);
        border-top: none;
        border-left: none;
    }
    
    :global(.tooltip-container strong) {
        color: #f8fafc;
    }
    
    :global(.tooltip-container .text-gray-500) {
        color: #94a3b8;
    }
    
    :global(.tooltip-container .text-gray-800) {
        color: #f1f5f9;
    }
}
</style>