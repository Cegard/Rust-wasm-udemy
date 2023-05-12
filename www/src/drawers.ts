import { World } from 'snake_game_v1';
import {IDrawSnakeParams, IDrawWorldParams } from './models'

const CELL_SIZE = 30;

export function drawWorld(context: CanvasRenderingContext2D, world: World) {
  const dimensionSize = world.size() * CELL_SIZE;
  
  context.beginPath();
  Array.from(Array(world.size() + 1).keys()).map((position) => {
    const pos = CELL_SIZE * position;
    context.moveTo(0, pos);
    context.lineTo(dimensionSize, pos);
    context.moveTo(pos, 0);
    context.lineTo(pos, dimensionSize);
    context.stroke();
  });
}

export function drawSnake({context, headIdx, worldSize}: IDrawSnakeParams) {
  const headCol = headIdx % worldSize;
  const headRow = Math.floor(headIdx/worldSize);

  context.beginPath();
  context.fillRect(
    headCol * CELL_SIZE,
    headRow * CELL_SIZE,
    CELL_SIZE,
    CELL_SIZE
  );
  context.stroke();
}

export function setDrawer(context: CanvasRenderingContext2D, world: World): () => void {
  
  return () => {
    drawWorld(context, world);
    drawSnake({context, worldSize: world.size(), headIdx: world.get_snake_head()});
  };
}
