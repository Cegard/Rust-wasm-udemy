
export interface IDrawWorldParams {
  context: CanvasRenderingContext2D,
  worldSize: number
}

export interface IDrawSnakeParams extends IDrawWorldParams {
  headIdx: number
}