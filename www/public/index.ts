import init, {World} from 'snake_game_v1';
import {setDrawer} from '../src/drawers';

const TIMEOUT = 200;

async function start() {

  await init();

  const world = World.new();
  const canvas = <HTMLCanvasElement> document.getElementById("game-canvas");
  const context = canvas.getContext("2d");

  if (context === null) return

  const width = canvas.width;
  const height = canvas.height;
  const draw = setDrawer(context, world);
  draw();

  function update(context: CanvasRenderingContext2D, canvas: HTMLCanvasElement, world: World, ) {
    setTimeout(() => {
    context.clearRect(0, 0, width, height);
      world.update();
      draw();
      requestAnimationFrame(() => update(context, canvas, world)); // the callback will be invoked before the nezt browser re-paint
   }, TIMEOUT);
  }

  update(context, canvas, world);
}

start();
