import { World } from 'snake_game_v1';
import {IDrawSnakeParams, IDrawWorldParams } from './models'

export function setDrawWorld({context, worldSize, cellSize}: IDrawWorldParams): () => void {
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

export function drawSnake({context, headIdx, worldSize, cellSize}: IDrawSnakeParams) {
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
  const drawWorld = setDrawWorld({context, worldSize, cellSize});

  return () => {
    drawWorld();
    drawSnake({context, worldSize, cellSize, headIdx: world.get_snake_head()});
  };
}
