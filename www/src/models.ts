export type DrawWorldParams = [
  context: CanvasRenderingContext2D,
  worldSize: number,
  cellSize: number
];

export type DrawSnakeParams = [
  ...drawWorldParams: DrawWorldParams,
  headIdx: number
]