import { World } from 'snake_game_v1';
import {IDrawSnakeParams, IDrawWorldParams } from './models'

interface IDrawParams extends IDrawSnakeParams, IDrawWorldParams {}

const CELL_SIZE = 30;

function drawWorld({context, worldSize}: IDrawWorldParams) {
  const dimensionSize = worldSize * CELL_SIZE;
  
  context.beginPath();
  Array.from(Array(worldSize + 1).keys()).map((position) => {
    context.moveTo(CELL_SIZE * position, 0);
    context.lineTo(CELL_SIZE * position, dimensionSize);
    context.moveTo(0, CELL_SIZE * position);
    context.lineTo(dimensionSize, CELL_SIZE * position);
  });
  context.stroke();
}

function drawSnake({context, headIdx, worldSize}: IDrawSnakeParams) {
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

export default function setDrawer(context: CanvasRenderingContext2D, world: World): () => void {
  
  return () => {
    drawWorld({context, worldSize: world.size()});
    drawSnake({context, worldSize: world.size(), headIdx: world.get_snake_head()});
  }
}