import { Direction } from "snake_game_v1";

export type DrawWorldParams = readonly [
  CanvasRenderingContext2D,
  number,
  number
];

export type DrawSnakeParams = readonly [
  ...DrawWorldParams,
  Uint32Array
];

export type DirectionsType = {
  [key: string]: Direction
};
