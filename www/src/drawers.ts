import { World } from 'snake_game_v1';
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

export function drawSnake(...[context, headIdx, worldSize, cellSize]: DrawSnakeParams) {
  const headCol = headIdx % worldSize;
  const headRow = Math.floor(headIdx/worldSize);

  context.beginPath();
  context.fillRect(
    headCol * cellSize,
    headRow * cellSize,
    cellSize,
    cellSize
  );
  context.stroke();
}

export function setDrawer(context: CanvasRenderingContext2D, world: World, cellSize: number): () => void {
  const worldSize = world.size()
  const drawWorld = setDrawWorld(context, worldSize, cellSize);

  return () => {
    drawWorld();
    drawSnake(context, world.get_snake_head(), worldSize, cellSize);
  };
}
