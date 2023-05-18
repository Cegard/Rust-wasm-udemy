import init, { World } from 'snake_game_v1';
import { setDrawer } from '../src/drawers';
import { randomInt } from '../src/helpers';

function update(
  width: number,
  height: number,
  context: CanvasRenderingContext2D,
  world: World,
  draw: () => void
) {
  const SPEED = 2; // tiles per second

  function updateDelayed() {
    setTimeout(() => {
      context.clearRect(0, 0, width, height);
      world.update();
      draw();
      requestAnimationFrame(updateDelayed); // the callback will be invoked before the next browser re-paint
    }, 1000 / SPEED);
  }
  
  updateDelayed();
}

async function start() {
  const CELL_SIZE = 35;
  const WORLD_WIDTH = 8;
  const DIRECTION = 3;
  const snakeSpawnIdx = randomInt(0, Math.pow(WORLD_WIDTH, 2));

  await init();

  const world = World.new(WORLD_WIDTH, snakeSpawnIdx, DIRECTION);
  
  const canvas = <HTMLCanvasElement> document.getElementById("game-canvas");
  if (canvas === null) return;
  
  canvas.height = canvas.width = world.size() * CELL_SIZE;

  const context = canvas.getContext("2d");
  if (context === null) return;
  
  const draw = setDrawer(context, world, CELL_SIZE);

  draw();
  update(canvas.height, canvas.width, context, world, draw);
}

start();
