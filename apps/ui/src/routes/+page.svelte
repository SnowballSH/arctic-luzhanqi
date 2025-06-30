<script lang="ts">
import { onMount } from 'svelte';
import { initWasm, random_start } from '$lib/wasm';

let seed = '';
let board: any[] = [];
let useChinese = false;

onMount(async () => {
    await initWasm();
    generate();
});

function generate() {
    const val = seed ? BigInt(seed) : 0n;
    const state: any = random_start(val);
    board = state.board;
}

const pieceNamesEn = [
    'Field Marshal',
    'General',
    'Major General',
    'Brigadier General',
    'Colonel',
    'Major',
    'Captain',
    'Lieutenant',
    'Engineer',
    'Bomb',
    'Landmine',
    'Flag'
];
const pieceNamesZh = [
    '司令',
    '军长',
    '师长',
    '旅长',
    '团长',
    '营长',
    '连长',
    '排长',
    '工兵',
    '炸弹',
    '地雷',
    '军旗'
];
</script>

<div class="controls">
    <input placeholder="seed" bind:value={seed} />
    <label class="lang-toggle">
        <input type="checkbox" bind:checked={useChinese} />
        <span>{useChinese ? '中文' : 'English'}</span>
    </label>
    <button on:click={generate}>generate new random starting position</button>
</div>
<div class="board">
    {#each board as sq}
        <div class="cell">
            {#if sq}
                <span class="piece {sq.color === 0 ? 'red' : 'black'}">
                    {useChinese ? pieceNamesZh[sq.ty] : pieceNamesEn[sq.ty]}
                </span>
            {/if}
        </div>
    {/each}
</div>

<style>
.board {
    display: grid;
    grid-template-columns: repeat(5, 50px);
    grid-template-rows: repeat(13, 50px);
    gap: 2px;
    background: #f7f7f7;
    border: 2px solid #555;
    width: max-content;
}
.cell {
    border: 1px solid #999;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 12px;
    background: #fff;
    width: 50px;
    height: 50px;
}
.piece.red {
    color: #c00;
    font-weight: bold;
}
.piece.black {
    color: #000;
    font-weight: bold;
}
.controls {
    margin-bottom: 1rem;
    display: flex;
    gap: 1rem;
    align-items: center;
}
.lang-toggle {
    display: flex;
    align-items: center;
    gap: 0.25rem;
}
</style>
