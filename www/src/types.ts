export type DrawWorldParams = readonly [
  CanvasRenderingContext2D,
  number,
  number
];

export type DrawSnakeParams = readonly [
  ...DrawWorldParams,
  number
];
