import {
  animationWebgl,
  setupComputeProgram,
  setupCopyProgram,
  setupDisplayMonochrome,
  setupInitProgram,
  setupWebgl,
} from 'wasm-game-of-life';

let lastCall = 0;
let sum = 0;

let canvas = document.getElementById('game-of-life-canvas');
let rect = canvas.getBoundingClientRect();

let bWidth = rect.width;
let bHeight = rect.height;

let mWidth = Math.floor(bWidth / 8);
let mHeight = Math.floor(bHeight / 8);

canvas.setAttribute('width', bWidth);
canvas.setAttribute('height', bHeight);

let state = setupWebgl(mWidth, mHeight);
let initProgram = setupInitProgram();
let monochrome = setupDisplayMonochrome();
let computeProgram = setupComputeProgram();
let copyProgram = setupCopyProgram();

let drawProgram = initProgram;

document.getElementById('swap_colors').addEventListener('change', () => {
  if (drawProgram == initProgram) {
    drawProgram = monochrome;
  } else {
    drawProgram = initProgram;
  }
});

for (let i = 0; i < 100; i++) {
  animationWebgl(drawProgram, computeProgram, copyProgram, mWidth, mHeight, state);
}

const renderLoop = (timestamp) => {
  const delta = timestamp - lastCall;

  lastCall = timestamp;
  sum += delta;

  let fps = document.getElementById('frames-per-second').value;

  if (sum > 1000 / fps) {
    animationWebgl(drawProgram, computeProgram, copyProgram, mWidth, mHeight, state);
    sum = 0;
  }

  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
