import { World } from "snake_game_v1";

function randomInt (max: number): number {
  return Math.floor(Math.random() * max);
}

function buildSnake(buffer: ArrayBuffer, world: World): Uint32Array {
  return new Uint32Array(
    buffer,
    world.get_snake_cells(),
    world.get_snake_length()
  );
}

export {
  randomInt,
  buildSnake
}

