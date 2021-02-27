import { Universe, setupCanvas, getCellSize, animationWebgl, setupWebgl } from 'wasm-game-of-life';

const universe = Universe.new(64);

let lastCall = 0;
let sum = 0;

const renderLoop = (timestamp) => {
  const delta = timestamp - lastCall;

  lastCall = timestamp;
  sum += delta;

  let fps = document.getElementById('frames-per-second').value;

  if (sum > 1000 / fps) {
    const tpf = document.getElementById('ticks-per-frame').value;

    animationWebgl(universe, tpf);
    sum = 0;
  }

  requestAnimationFrame(renderLoop);
};

setupCanvas(universe);
setupWebgl();
requestAnimationFrame(renderLoop);

const width = universe.width();
const height = universe.height();

const canvas = document.getElementById('game-of-life-canvas');
const CELL_SIZE = getCellSize();

canvas.addEventListener('click', (e) => {
  const boundingRect = canvas.getBoundingClientRect();
  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;
  const canvasLeft = (e.clientX - boundingRect.left) * scaleX;
  const canvasTop = (e.clientY - boundingRect.top) * scaleY;
  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

  universe.toggle_cell(row, col);
});
