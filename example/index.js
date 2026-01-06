import init, { sort_numbers } from '../pkg/sort_wasm';

async function run() {
    await init();
    const nums = [5, 2, 9, 1, 5, 6];
    const sorted = sort_numbers(nums, true);
    document.getElementById('output').textContent = `Sorted: ${JSON.stringify(sorted)}`;
}

document.getElementById('sortBtn').onclick = run;

