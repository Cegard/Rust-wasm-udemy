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

function drawGameStatus(label: HTMLElement, world: World) {
  label.textContent = world.game_status_text();
}

function setPlay(
  width: number,
  height: number,
  context: CanvasRenderingContext2D,
  label: HTMLElement,
  world: World,
  draw: () => void
) {
  const SPEED = 2; // tiles per second
  
  draw();

  return function updateDelayed() {
    setTimeout(() => {
      context.clearRect(0, 0, width, height);
      world.step();
      drawGameStatus(label, world);
      draw();
      requestAnimationFrame(updateDelayed); // the callback will be invoked before the next browser re-paint
    }, 1000 / SPEED);
  }
}

async function start() {
  const CELL_SIZE = 35;
  const WORLD_WIDTH = 7;
  const SNAKE_LENGTH = 4;

  let direction = Direction.Right;
  const snakeSpawnIdx = randomInt(Math.pow(WORLD_WIDTH, 2));

  const wasm = await init();

  const world = World.new(WORLD_WIDTH, snakeSpawnIdx, direction, SNAKE_LENGTH);

  const canvas = <HTMLCanvasElement>document.getElementById("game-canvas");
  const context = canvas.getContext("2d");
  const gameStatus = document.getElementById("game-status");
  const controlGameBtn = document.getElementById("control-game-btn");

  if (canvas === null || context === null || gameStatus === null || controlGameBtn === null ) return;

  canvas.height = canvas.width = world.length() * CELL_SIZE;
  drawGameStatus(gameStatus, world);

  const draw = setDrawer(
    context,
    world.length(),
    CELL_SIZE,
    wasm,
    world
  );
  const play = setPlay(canvas.height, canvas.width, context, gameStatus, world, draw);
  
  controlGameBtn.addEventListener("click", _ => {
    const initStatus = world.game_status();

    if (initStatus === undefined) {
      controlGameBtn.textContent = "Start again";
      play();
      world.start_game();
    }
    else {
      location.reload();
    }
  });

  document.addEventListener("keydown", (e) => {
    direction = directions[e.code] ?? direction;
    world.change_snake_direction(direction);
  });
}

start();