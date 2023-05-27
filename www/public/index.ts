import init, { World, Direction } from "snake_game_v1";
import { setDrawer } from "../src/drawers";
import { randomInt } from "../src/helpers";
import { DirectionsType } from "../src/types";

const directions: DirectionsType = {
  ArrowUp: Direction.Up,
  ArrowRight: Direction.Right,
  ArrowDown: Direction.Down,
  ArrowLeft: Direction.Left,
};

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
      world.step();
      draw();
      requestAnimationFrame(updateDelayed); // the callback will be invoked before the next browser re-paint
    }, 1000 / SPEED);
  }

  updateDelayed();
}

function build_snake(buffer: ArrayBuffer, world: World): Uint32Array {
  return new Uint32Array(
    buffer,
    world.get_snake_cells(),
    world.get_snake_length()
  );
}

async function start() {
  const CELL_SIZE = 35;
  const WORLD_WIDTH = 8;
  const SNAKE_LENGTH = 5;

  let direction = Direction.Right;
  const snakeSpawnIdx = randomInt(0, Math.pow(WORLD_WIDTH, 2));

  const wasm = await init();

  const world = World.new(WORLD_WIDTH, snakeSpawnIdx, direction, SNAKE_LENGTH);

  const canvas = <HTMLCanvasElement>document.getElementById("game-canvas");
  if (canvas === null) return;

  canvas.height = canvas.width = world.length() * CELL_SIZE;

  const context = canvas.getContext("2d");
  if (context === null) return;

  const draw = setDrawer(
    context,
    world.length(),
    CELL_SIZE,
    build_snake(wasm.memory.buffer, world)
  );

  draw();
  update(canvas.height, canvas.width, context, world, draw);

  document.addEventListener("keydown", (e) => {
    direction = directions[e.code] ?? direction;
    let new_dir = world.change_snake_direction(direction);
    console.log(new_dir)
  });
}

start();