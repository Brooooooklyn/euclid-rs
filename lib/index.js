const { Transform2D } = require("../index");

const transform = new Transform2D(1, 2, 3, 4, 5, 6);
const transform2 = new Transform2D(2, 2, 2, 2, 2, 2);
const transform3 = new Transform2D(3, 3, 3, 3, 3, 3);
transform2.postMul(transform3);
transform.postMul(transform2);

console.log("getValue", transform.getValue());
console.log("transformVector", transform.transformVector(4, 4));
