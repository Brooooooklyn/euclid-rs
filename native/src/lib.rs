#[macro_use]
extern crate neon;
extern crate euclid;
extern crate neon_runtime;

use std::ops::Deref;

use euclid::{Transform2D, Vector2D};
use neon::js::class::{Class, JsClass};
use neon::js::{JsArray, JsFunction, JsNumber, JsObject, JsUndefined, Object, Value};
use neon::mem::Handle;
use neon::vm::{FunctionCall, JsResult, Lock, This};

trait CheckArgument<'a> {
  fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<'a, V>;
}

impl<'a, T: This> CheckArgument<'a> for FunctionCall<'a, T> {
  fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<'a, V> {
    self.arguments.require(self.scope, i)?.check::<V>()
  }
}

#[derive(Debug)]
pub struct Transfrom2DWrap(Transform2D<f64>);

impl std::ops::Deref for Transfrom2DWrap {
  type Target = Transform2D<f64>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

declare_types! {
  pub class EuclidTransform2D for Transfrom2DWrap {
    init(mut call) {
      let m11_raw = call
        .check_argument::<JsNumber>(0)
        .unwrap();
      let m11 = m11_raw.value();
      let m12_raw = call
        .check_argument::<JsNumber>(1)
        .unwrap();
      let m12 = m12_raw.value();
      let m21_raw = call
        .check_argument::<JsNumber>(2)
        .unwrap();
      let m21 = m21_raw.value();
      let m22_raw = call
        .check_argument::<JsNumber>(3)
        .unwrap();
      let m22 = m22_raw.value();
      let m31_raw = call
        .check_argument::<JsNumber>(4)
        .unwrap();
      let m31 = m31_raw.value();
      let m32_raw = call
        .check_argument::<JsNumber>(5)
        .unwrap();
      let m32 = m32_raw.value();
      Ok(Transfrom2DWrap(Transform2D::row_major(m11, m12, m21, m22, m31, m32)))
    }

    method postMul(mut call) {
      let m11_raw = call
        .check_argument::<JsNumber>(0)
        .unwrap();
      let m11 = m11_raw.value();
      let m12_raw = call
        .check_argument::<JsNumber>(1)
        .unwrap();
      let m12 = m12_raw.value();
      let m21_raw = call
        .check_argument::<JsNumber>(2)
        .unwrap();
      let m21 = m21_raw.value();
      let m22_raw = call
        .check_argument::<JsNumber>(3)
        .unwrap();
      let m22 = m22_raw.value();
      let m31_raw = call
        .check_argument::<JsNumber>(4)
        .unwrap();
      let m31 = m31_raw.value();
      let m32_raw = call
        .check_argument::<JsNumber>(5)
        .unwrap();
      let m32 = m32_raw.value();
      let other = Transform2D::row_major(m11, m12, m21, m22, m31, m32);
      let mut this = call.arguments.this(call.scope);
      this.grab(|c| c.0 = c.0.post_mul(&other));
      Ok(JsUndefined::new().as_value(call.scope))
    }

    method transformVector(mut call) {
      let x = call
        .check_argument::<JsNumber>(0)
        ?.value();
      let y = call
        .check_argument::<JsNumber>(1)
        ?.value();
      let vector = Vector2D::new(x, y);
      let mut this = call.arguments.this(call.scope);
      let raw = this.grab(|c| c.0);
      let vec = raw.transform_vector(&vector);
      let dist_arr = JsArray::new(call.scope, 2);
      dist_arr.set(0, JsNumber::new(call.scope, vec.x)).unwrap();
      dist_arr.set(1, JsNumber::new(call.scope, vec.y)).unwrap();
      Ok(dist_arr.as_value(call.scope))
    }

    method getValue(call) {
      let mut this = call.arguments.this(call.scope);
      let raw = this.grab(|c| c.0);
      let dist_arr = JsArray::new(call.scope, 6);
      dist_arr.set(0, JsNumber::new(call.scope, raw.m11)).unwrap();
      dist_arr.set(1, JsNumber::new(call.scope, raw.m12)).unwrap();
      dist_arr.set(2, JsNumber::new(call.scope, raw.m21)).unwrap();
      dist_arr.set(3, JsNumber::new(call.scope, raw.m22)).unwrap();
      dist_arr.set(4, JsNumber::new(call.scope, raw.m31)).unwrap();
      dist_arr.set(5, JsNumber::new(call.scope, raw.m32)).unwrap();
      Ok(dist_arr.as_value(call.scope))
    }
  }
}

register_module!(m, {
  let class: Handle<JsClass<EuclidTransform2D>> = try!(EuclidTransform2D::class(m.scope));
  let constructor: Handle<JsFunction<EuclidTransform2D>> = try!(class.constructor(m.scope));
  let exports = m.exports.check::<JsObject>().unwrap();
  let deref_module = exports.deref();
  try!(deref_module.set("EuclidTransform2D", constructor));
  Ok(())
});
