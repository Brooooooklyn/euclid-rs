export class Transform2D {
  constructor(
    m11: number,
    m12: number,
    m21: number,
    m22: number,
    m31: number,
    m32: number
  );

  transformVector(x: number, y: number);
  postMul(transform: Transform2D);
  getValue(): [number, number, number, number, number, number];
}
