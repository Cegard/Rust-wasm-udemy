export interface IDrawWorldParams {
  context: CanvasRenderingContext2D,
  worldSize: number,
  cellSize: number,
}

export interface IDrawSnakeParams extends IDrawWorldParams {
  headIdx: number
}