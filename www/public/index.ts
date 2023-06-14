import init, { World, Direction, SnakeStatus } from "snake_game_v1";
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
      const step_result: SnakeStatus = world.step();
      draw();
      requestAnimationFrame(step_result == SnakeStatus.Playing ? updateDelayed : () => {alert("Stop!")}); // the callback will be invoked before the next browser re-paint
    }, 1000 / SPEED);
  }

  updateDelayed();
}

async function start() {
  const CELL_SIZE = 35;
  const WORLD_WIDTH = 8;
  const SNAKE_LENGTH = 4;

  let direction = Direction.Right;
  const snakeSpawnIdx = randomInt(Math.pow(WORLD_WIDTH, 2));

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
    wasm,
    world
  );

  draw();
  update(canvas.height, canvas.width, context, world, draw);

  document.addEventListener("keydown", (e) => {
    direction = directions[e.code] ?? direction;
    world.change_snake_direction(direction);
  });
}

start();