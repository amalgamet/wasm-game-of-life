import { useEffect, useRef, useState } from 'react';
// import ZingTouch from 'zingtouch';
// import {
//   setupCopyProgram,
//   setupDisplayMonochromeProgram,
//   setupDisplayProgram,
//   setupComputeProgram,
//   animationWebgl,
//   setupWebgl,
// } from 'wasm-game-of-life';

function updateCanvasSize(canvas, ctx) {
  const { width, height } = canvas.getBoundingClientRect();

  if (canvas.width !== width || canvas.height !== height) {
    const { devicePixelRatio: ratio = 1 } = window;

    canvas.width = width * ratio;
    canvas.height = height * ratio;
    ctx.scale(ratio, ratio);

    return true;
  }

  return false;
}

function useCanvas(draw, options = {}) {
  const canvasRef = useRef(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    const ctx = canvas.getContext(options.context || '2d');
    let frameCount = 0;
    let animationFrameId;

    function render() {
      frameCount++;
      updateCanvasSize(canvas, ctx);
      draw(ctx, frameCount);
      animationFrameId = requestAnimationFrame(render);
    }

    requestAnimationFrame(render);

    return () => {
      cancelAnimationFrame(animationFrameId);
    };
  }, [draw]);

  return canvasRef;
}

export default function Canvas({ draw, options, ...rest }) {
  const canvasRef = useCanvas(draw, options);

  return <canvas ref={canvasRef} {...rest} />;
}
