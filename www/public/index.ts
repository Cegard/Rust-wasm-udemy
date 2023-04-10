import init, { World } from 'snake_game_v1';

async function start() {
  const CELL_SIZE = 30;

  await init();

  const world = World.new();
  const worldSize = world.size();
  const dimensionSize = CELL_SIZE * worldSize;
  const canvas = document.getElementById("game-canvas");
  const context = canvas.getContext("2d");
  
  canvas.width = canvas.height = dimensionSize;

  function drawWorld() {
    context.beginPath();
    Array.from(Array(worldSize + 1).keys()).map((position) => {
      context.moveTo(CELL_SIZE * position, 0);
      context.lineTo(CELL_SIZE * position, dimensionSize);
      context.moveTo(0, CELL_SIZE * position);
      context.lineTo(dimensionSize, CELL_SIZE * position);
    });
    context.stroke();
  }

  drawWorld();
}

start();
