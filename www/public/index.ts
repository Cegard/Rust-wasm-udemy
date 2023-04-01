import init, { World } from 'snake_game_v1';

async function start() {
  await init();
  const world = World.new();
  console.log(world.size());
}

start();
