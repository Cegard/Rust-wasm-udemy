import init, { World, Direction } from 'snake_game_v1';
import { setDrawer } from '../src/drawers';
import { randomInt } from '../src/helpers';
import { ARROW_KEYS_CODES } from '../src/consts';

const directionModifiers = {
  [ARROW_KEYS_CODES.up]: (world: World) => { world.change_snake_direction(Direction.Up) },
  [ARROW_KEYS_CODES.right]: (world: World) => { world.change_snake_direction(Direction.Right) },
  [ARROW_KEYS_CODES.down]: (world: World) => { world.change_snake_direction(Direction.Down) },
  [ARROW_KEYS_CODES.left]: (world: World) => { world.change_snake_direction(Direction.Left) },
}

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
  const DIRECTION = Direction.Right;
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
  console.log("init ", world.get_snake_head())
  update(canvas.height, canvas.width, context, world, draw);

  document.addEventListener('keydown', e => {
    let move_snake = directionModifiers[e.code] ?? false;

    if (move_snake)
      move_snake(world);
  });
}

start();
