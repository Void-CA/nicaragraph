<!-- Tooltip.svelte -->
<script>
export let content = "";
export let visible = false;
export let x = 0;
export let y = 0;
</script>

{#if visible}
<div class="fixed z-50 bg-white text-gray-800 p-2 rounded shadow-lg text-sm pointer-events-none"
     style="left: {x}px; top: {y}px;">
    {content}
</div>
{/if}
