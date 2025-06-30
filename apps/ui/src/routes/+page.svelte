<script lang="ts">
    import {onMount} from 'svelte';
    import {initWasm, random_start} from '$lib/wasm';

    // State variables for the component
    let seed = $state('');
    let board: any[] = $state([]);
    let useChinese = $state(false);

    onMount(async () => {
        // Initialize the WebAssembly module when the component mounts
        await initWasm();
        generate();
    });

    // Function to generate a new board layout using the WASM module
    function generate() {
        // Use the provided seed or a random number to generate the board state
        const val = seed ? BigInt(seed) : BigInt(Math.floor(Math.random() * (1 << 30)));
        const state: any = random_start(val);
        board = state.board;
        console.log(board);
    }

    // English and Chinese names for the game pieces
    const pieceNamesEn = [
        'Field Marshal', 'General', 'Major General', 'Brigadier General', 'Colonel', 'Major',
        'Captain', 'Lieutenant', 'Engineer', 'Bomb', 'Landmine', 'Flag'
    ];
    const pieceNamesZh = [
        '司令', '军长', '师长', '旅长', '团长', '营长',
        '连长', '排长', '工兵', '炸弹', '地雷', '军旗'
    ];

    // Maps the piece type string from WASM to the correct name array index
    const typeMap: Record<string, number> = {
        Overall: 0, Army: 1, Division: 2, Brigade: 3, Regiment: 4, Battalion: 5,
        Company: 6, Platoon: 7, Engineer: 8, Bomb: 9, Landmine: 10, Flag: 11
    };

    // --- Logic to identify special squares ---

    // Define the indices for each type of special square on the 13x5 board.
    const headquartersIndices = new Set([1, 3, 61, 63]);
    const campsiteIndices = new Set([11, 13, 17, 21, 23, 41, 43, 47, 51, 53]);
    const frontlineIndices = new Set([30, 32, 34]);
    const mountainIndices = new Set([31, 33]);
    // New set for all railroad squares, including frontlines which act as railroads
    const railroadIndices = new Set([5, 6, 7, 8, 9, 10, 14, 15, 19, 20, 24, 25, 26, 27, 28, 29, 30, 32, 34, 35, 36, 37, 38, 39, 40, 44, 45, 49, 50, 54, 55, 56, 57, 58, 59]);


    /**
     * Determines the primary type of a square based on its index.
     * @param {number} index - The index of the square in the flat board array.
     * @returns {string} - A class name representing the square type.
     */
    function getSquareType(index: number): string {
        if (headquartersIndices.has(index)) return 'headquarters';
        if (campsiteIndices.has(index)) return 'campsite';
        if (frontlineIndices.has(index)) return 'frontline';
        if (mountainIndices.has(index)) return 'mountain';
        if (railroadIndices.has(index)) return 'railroad';
        return 'normal';
    }

    /**
     * Determines the railroad connection classes for a square.
     * @param {number} index - The index of the square.
     * @returns {string} - A string of classes for railroad connections.
     */
    function getRailroadClasses(index: number): string {
        if (!railroadIndices.has(index) && !frontlineIndices.has(index)) return '';

        let classes = '';
        const col = index % 5;

        // Check for vertical connections
        if (railroadIndices.has(index - 5) || frontlineIndices.has(index - 5)) classes += ' rail-top';
        if (railroadIndices.has(index + 5) || frontlineIndices.has(index + 5)) classes += ' rail-bottom';

        // Check for horizontal connections (only if not on the edge)
        if (col > 0 && (railroadIndices.has(index - 1) || frontlineIndices.has(index - 1))) classes += ' rail-left';
        if (col < 4 && (railroadIndices.has(index + 1) || frontlineIndices.has(index + 1))) classes += ' rail-right';

        return classes;
    }

</script>

<svelte:head>
    <!--
      Replaced Google Fonts with Bunny Fonts, a privacy-friendly alternative
      that has better accessibility in regions like China.
    -->
    <link rel="preconnect" href="https://fonts.bunny.net">
    <link href="https://fonts.bunny.net/css?family=poppins:400,500,600" rel="stylesheet"/>
</svelte:head>

<!-- Main container to center the content -->
<main>
    <div class="controls">
        <input placeholder="Enter seed..." type="text" bind:value={seed}/>

        <button onclick={generate}>
            New Board
        </button>

        <!-- The language toggle is now a custom-styled switch -->
        <label class="lang-toggle">
            <input type="checkbox" bind:checked={useChinese}/>
            <span class="toggle-slider"></span>
            <span class="lang-text">{useChinese ? '中文' : 'English'}</span>
        </label>
    </div>

    <div class="board">
        {#each board as sq, i}
            <!-- Add railroad classes alongside the primary square type class -->
            <div class="cell {getSquareType(i)} {getRailroadClasses(i)}">
                {#if sq}
                    <div class="piece {sq.color === 'Red' ? 'red' : 'blue'}">
                        <span>
                            {#if useChinese}
                                {pieceNamesZh[typeMap[sq.ty]] ?? sq.ty}
                            {:else}
                                {pieceNamesEn[typeMap[sq.ty]] ?? sq.ty}
                            {/if}
                        </span>
                    </div>
                {/if}
            </div>
        {/each}
    </div>
</main>

<style>
    /*
    ** Global Styles & Arctic Theme
    */
    :global(body) {
        font-family: 'Poppins', sans-serif;
        background: linear-gradient(170deg, #f0f8ff, #e6f2ff);
        color: #2c3e50;
        margin: 0;
    }

    /*
    ** Main Layout Container
    */
    main {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        min-height: 100vh;
        padding: 2rem;
        box-sizing: border-box;
    }

    /*
    ** Controls Section
    */
    .controls {
        margin-bottom: 2rem;
        display: flex;
        gap: 1rem;
        align-items: center;
        padding: 1rem 1.5rem;
        background: rgba(255, 255, 255, 0.5);
        backdrop-filter: blur(8px);
        -webkit-backdrop-filter: blur(8px);
        border-radius: 16px;
        box-shadow: 0 8px 32px 0 rgba(118, 163, 208, 0.3);
        border: 1px solid rgba(255, 255, 255, 0.18);
    }

    .controls input[type="text"] {
        padding: 0.75rem;
        border: 1px solid #cddde9;
        border-radius: 10px;
        background-color: #fdfeff;
        color: #2c3e50;
        font-size: 0.9rem;
        font-family: 'Poppins', sans-serif;
        transition: all 0.2s ease-in-out;
        width: 120px;
    }

    .controls input[type="text"]:focus {
        outline: none;
        border-color: #6c9bc2;
        box-shadow: 0 0 0 4px rgba(93, 138, 168, 0.15);
    }

    .controls button {
        background: linear-gradient(145deg, #7ba9d1, #5d8aa8);
        color: white;
        font-weight: 500;
        font-size: 0.9rem;
        border: none;
        padding: 0.75rem 1.5rem;
        border-radius: 10px;
        cursor: pointer;
        transition: all 0.2s ease-in-out;
        box-shadow: 0 4px 6px rgba(44, 62, 80, 0.1);
    }

    .controls button:hover {
        transform: translateY(-2px);
        box-shadow: 0 6px 12px rgba(44, 62, 80, 0.2);
    }

    .controls button:active {
        transform: translateY(0);
    }

    /*
    ** Custom Language Toggle Switch
    */
    .lang-toggle {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        cursor: pointer;
    }

    .lang-toggle input[type="checkbox"] {
        opacity: 0;
        width: 0;
        height: 0;
    }

    .lang-text {
        font-weight: 500;
        font-size: 0.9rem;
        color: #34495e;
        user-select: none;
    }

    .toggle-slider {
        width: 48px;
        height: 26px;
        background-color: #cddde9;
        border-radius: 13px;
        position: relative;
        transition: background-color 0.3s ease;
    }

    .toggle-slider::before {
        content: '';
        position: absolute;
        width: 20px;
        height: 20px;
        border-radius: 50%;
        background-color: white;
        top: 3px;
        left: 4px;
        transition: transform 0.3s ease;
        box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
    }

    .lang-toggle input:checked + .toggle-slider {
        background-color: #5d8aa8;
    }

    .lang-toggle input:checked + .toggle-slider::before {
        transform: translateX(20px);
    }

    /*
    ** Game Board
    */
    .board {
        display: grid;
        grid-template-columns: repeat(5, 90px);
        grid-template-rows: repeat(13, 45px); /* 2:1 aspect ratio */
        gap: 5px;
        background: #a9cce3; /* A medium ice blue */
        border: 2px solid #5d8aa8;
        border-radius: 16px;
        padding: 10px;
        box-shadow: 0 10px 30px rgba(0, 0, 0, 0.1);
        width: max-content;
    }

    /*
    ** Board Cells
    */
    .cell {
        position: relative;
        display: flex;
        align-items: center;
        justify-content: center;
        background: #e4eef7;
        border-radius: 8px;
        transition: background-color 0.2s ease;
        overflow: hidden;
    }

    .cell:hover {
        background: #f0f8ff;
    }

    /* --- Styles for Special Squares --- */

    /* Campsite (行营): A safe zone. */
    .cell.campsite::after {
        content: '';
        position: absolute;
        width: 25px;
        height: 25px;
        border: 3px solid #82a8d1;
        border-radius: 50%;
        box-sizing: border-box;
    }

    /* Headquarters (大本营): The base. */
    .cell.headquarters {
        background-color: #fdf8e1; /* Pale gold background */
    }

    .cell.headquarters::after {
        content: '★';
        position: absolute;
        font-size: 24px;
        color: #e0d2a8;
        z-index: 0;
    }

    /* Mountain (山界): Impassable terrain. */
    .cell.mountain {
        background: repeating-linear-gradient(
                45deg,
                #b8c8d8,
                #b8c8d8 5px,
                #c4d3e1 5px,
                #c4d3e1 10px
        );
        cursor: not-allowed;
    }

    /* --- New Styles for Railroads --- */

    /* Base style for the track element */
    .railroad::before, .frontline::before {
        content: '';
        position: absolute;
        background-color: #9eb8d3; /* Cool, icy-grey track color */
        z-index: 0;
    }

    /* Horizontal track segment */
    .rail-left.rail-right::before {
        width: 100%;
        height: 5px;
        top: 50%;
        left: 0;
        transform: translateY(-50%);
    }

    /* Vertical track segment */
    .rail-top.rail-bottom::before {
        width: 5px;
        height: 100%;
        left: 50%;
        top: 0;
        transform: translateX(-50%);
    }

    /* Corner and intersection track pieces */
    .railroad::after, .frontline::after {
        content: '';
        position: absolute;
        background-color: #9eb8d3;
        z-index: 0;
    }

    /* Handling corners and T-junctions */
    .rail-left:not(.rail-right)::before,
    .rail-right:not(.rail-left)::before {
        width: 50%;
        height: 5px;
        top: 50%;
        transform: translateY(-50%);
    }

    .rail-left:not(.rail-right)::before {
        left: 0;
    }

    .rail-right:not(.rail-left)::before {
        right: 0;
    }

    .rail-top:not(.rail-bottom)::before,
    .rail-bottom:not(.rail-top)::before {
        width: 5px;
        height: 50%;
        left: 50%;
        transform: translateX(-50%);
    }

    .rail-top:not(.rail-bottom)::before {
        top: 0;
    }

    .rail-bottom:not(.rail-top)::before {
        bottom: 0;
    }

    /* For intersections, create the cross shape using both pseudo-elements */
    .rail-left.rail-right.rail-top.rail-bottom::before {
        width: 100%;
        height: 5px;
        top: 50%;
        left: 0;
        transform: translateY(-50%);
    }

    .rail-left.rail-right.rail-top.rail-bottom::after {
        width: 5px;
        height: 100%;
        top: 0;
        left: 50%;
        transform: translateX(-50%);
    }

    /* For T-junctions */
    .rail-left.rail-right.rail-top:not(.rail-bottom)::after {
        width: 5px;
        height: 50%;
        top: 0;
        left: 50%;
        transform: translateX(-50%);
    }

    .rail-left.rail-right.rail-bottom:not(.rail-top)::after {
        width: 5px;
        height: 50%;
        bottom: 0;
        left: 50%;
        transform: translateX(-50%);
    }

    .rail-top.rail-bottom.rail-left:not(.rail-right)::after {
        width: 50%;
        height: 5px;
        top: 50%;
        left: 0;
        transform: translateY(-50%);
    }

    .rail-top.rail-bottom.rail-right:not(.rail-left)::after {
        width: 50%;
        height: 5px;
        top: 50%;
        right: 0;
        transform: translateY(-50%);
    }

    /* Frontline (前线): Has a different background but still acts as a railroad. */
    .cell.frontline {
        background-color: #c5d5e5;
    }

    /* Ensure Campsite icons appear over railroad tracks */
    .cell.campsite.railroad::after, .cell.campsite.frontline::after {
        content: '';
        position: absolute;
        width: 25px;
        height: 25px;
        border: 3px solid #82a8d1;
        border-radius: 50%;
        box-sizing: border-box;
        background-color: #e4eef7; /* Match cell bg to "cover" the track */
        z-index: 1; /* Higher z-index for campsite icon */
    }

    /*
    ** Game Pieces
    */
    .piece {
        font-weight: 600;
        font-size: 0.8rem;
        display: flex;
        align-items: center;
        justify-content: center;
        width: 80%;
        height: 80%;
        border-radius: 6px;
        cursor: grab;
        transition: transform 0.2s ease, box-shadow 0.2s ease;
        user-select: none;
        box-shadow: inset 0 1px 1px rgba(255, 255, 255, 0.5), 0 2px 3px rgba(0, 0, 0, 0.1);
        position: relative;
        z-index: 2; /* Highest z-index to be on top of everything */
    }

    .piece > span {
        text-align: center;
        line-height: 1.2;
    }

    .piece:hover {
        transform: scale(1.05);
        box-shadow: inset 0 1px 1px rgba(255, 255, 255, 0.5), 0 4px 8px rgba(0, 0, 0, 0.15);
    }

    .piece:active {
        cursor: grabbing;
        transform: scale(1.02);
    }

    .piece.red {
        background: #f8d7da;
        color: #721c24;
        border: 1px solid #f5c6cb;
    }

    .piece.blue {
        background: #cfe2ff;
        color: #084298;
        border: 1px solid #b6d4fe;
    }
</style>
