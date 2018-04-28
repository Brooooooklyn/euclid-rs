const { EuclidTransform2D } = require("./native");

const postMul = EuclidTransform2D.prototype.postMul;

EuclidTransform2D.prototype.postMul = function(transform) {
  const args = transform.getValue();
  return postMul.apply(this, args);
};

module.exports = EuclidTransform2D;
module.exports.Transform2D = EuclidTransform2D;
