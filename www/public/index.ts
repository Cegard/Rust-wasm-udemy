import init, { World } from 'snake_game_v1';

async function start() {
  await init();
  const world = World.new();
  const canvas = document.getElementById("game-canvas");
  const context = canvas.getContext("2d");
}

start();
