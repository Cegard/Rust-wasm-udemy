import { World } from 'snake_game_v1';
import {DrawSnakeParams, DrawWorldParams } from './types'

function setWorldDrawer(...[context, worldSize, cellSize]: DrawWorldParams): () => void {
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

function drawSnake(...[context, worldSize, cellSize, cells]: DrawSnakeParams) {
  
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

function drawReward(
  world: World,
  worldLength: number,
  context: CanvasRenderingContext2D,
  cellSize: number
) {
  
  const rewardIdx = world.get_reward_idx();
  const col = rewardIdx % worldLength;
  const row = Math.floor(rewardIdx/worldLength);

  context.fillStyle = "#ffaa00";

  context.beginPath();
  context.fillRect(
    col * cellSize,
    row * cellSize,
    cellSize,
    cellSize
  );
  context.stroke();
}

export function setDrawer(
  context: CanvasRenderingContext2D,
  worldLength: number,
  cellSize: number,
  cells: Uint32Array,
  world: World
): () => void {
  const drawWorld = setWorldDrawer(context, worldLength, cellSize);

  return () => {
    drawWorld();
    drawSnake(context, worldLength, cellSize, cells);
    drawReward(world, worldLength, context, cellSize);
  };
}
