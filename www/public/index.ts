import init, {World} from 'snake_game_v1';
import {setDrawer} from '../src/drawers';

function update(
  width: number,
  height: number,
  context: CanvasRenderingContext2D,
  world: World,
  draw: () => void
) {
  const TIMEOUT = 100;

  function updateDelayed() {
    setTimeout(() => {
      context.clearRect(0, 0, width, height);
      world.update();
      draw();
      requestAnimationFrame(updateDelayed); // the callback will be invoked before the next browser re-paint
    }, TIMEOUT);
  }
  
  updateDelayed();
}

async function start() {
  const CELL_SIZE = 35;

  await init();

  const world = World.new();
  
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
