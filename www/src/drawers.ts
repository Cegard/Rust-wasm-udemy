import {DrawSnakeParams, DrawWorldParams } from './types'

export function setDrawWorld(...[context, worldSize, cellSize]: DrawWorldParams): () => void {
  const dimentionSize = cellSize * cellSize;
  
  return () => {
    context.beginPath();
    Array.from(Array(worldSize + 1).keys()).map((position) => {
      const pos = cellSize * position;
      context.moveTo(0, pos);
      context.lineTo(dimentionSize, pos);
      context.moveTo(pos, 0);
      context.lineTo(pos, dimentionSize);
    });
    context.stroke();
  };
}

export function drawSnake(...[context, worldSize, cellSize, cells]: DrawSnakeParams) {
  
  cells.forEach((cell, i) => {
    const headCol = cell % worldSize;
    const headRow = Math.floor(cell/worldSize);

    context.fillStyle = i === 0 ? "#7878db" : "#000000";

    context.beginPath();
    context.fillRect(
      headCol * cellSize,
      headRow * cellSize,
      cellSize,
      cellSize
    );
    context.stroke();
  });
}

export function setDrawer(
  context: CanvasRenderingContext2D,
  worldSize: number,
  cellSize: number,
  cells: Uint32Array
): () => void {
  const drawWorld = setDrawWorld(context, worldSize, cellSize);

  return () => {
    drawWorld();
    drawSnake(context, worldSize, cellSize, cells);
  };
}
