import {
  setupCopyProgram,
  setupDisplayMonochromeProgram,
  setupDisplayProgram,
  setupComputeProgram,
  animationWebgl,
  setupWebgl,
} from 'wasm-game-of-life';

import ZingTouch from 'zingtouch';

const canvas = document.getElementById('game-of-life-canvas');
const brect = canvas.getBoundingClientRect();
canvas.setAttribute('width', brect.width);
canvas.setAttribute('height', brect.height);

const colorProgram = setupDisplayProgram();
const monochromeProgram = setupDisplayMonochromeProgram();
const computeProgram = setupComputeProgram();
const copyProgram = setupCopyProgram();

let drawProgram = colorProgram;

let activeRegion = ZingTouch.Region(canvas);

activeRegion.bind(canvas, 'tap', () => {
  if (drawProgram == colorProgram) {
    drawProgram = monochromeProgram;
  } else {
    drawProgram = colorProgram;
  }
});

let main = (cellsPerInch) => {
  const ppi = window.devicePixelRatio * 96;
  const mWidth = Math.floor((brect.width / ppi) * cellsPerInch);
  const mHeight = Math.floor((brect.height / ppi) * cellsPerInch);

  const state = setupWebgl(mWidth, mHeight);

  let lastCall = 0;
  let cum = 0;

  // skip first 100 iterations
  for (let i = 0; i < 100; i += 1) {
    animationWebgl(drawProgram, computeProgram, copyProgram, mWidth, mHeight, state);
  }

  const renderLoop = (timestamp) => {
    const delta = timestamp - lastCall;
    lastCall = timestamp;
    cum += delta;

    let fps = 60;
    if (cum > 1000 / fps) {
      animationWebgl(drawProgram, computeProgram, copyProgram, mWidth, mHeight, state);
      cum = 0;
    }

    requestAnimationFrame(renderLoop);
  };

  requestAnimationFrame(renderLoop);
};

main(80);
