import { animationWebgl, setupComputeProgram, setupInitProgram, setupWebgl } from 'wasm-game-of-life';

let lastCall = 0;
let sum = 0;

let canvas = document.getElementById('game-of-life-canvas');
let brect = canvas.getBoundingClientRect();

canvas.setAttribute('width', brect.width);
canvas.setAttribute('height', brect.height);

let state = setupWebgl();
let initProgram = setupInitProgram();
let computeProgram = setupComputeProgram();

const renderLoop = (timestamp) => {
  const delta = timestamp - lastCall;

  lastCall = timestamp;
  sum += delta;

  let fps = document.getElementById('frames-per-second').value;

  if (sum > 1000 / fps) {
    animationWebgl(initProgram, computeProgram, state);
    sum = 0;
  }

  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
