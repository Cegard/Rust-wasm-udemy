
function randomInt (min: number, max: number): number {
  return Math.ceil(Math.random() * (max - min) + min);
}

const now = Date.now;

export {
  randomInt,
  now
}

