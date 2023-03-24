import init, { greet } from 'snake_game_v1';

async function start() {
  await init();
  greet("Eddie");
}

start();
