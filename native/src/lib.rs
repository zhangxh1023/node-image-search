extern crate neon;
use neon::prelude::*;
mod image;
mod utils;

struct ImageSearchTask {
  parent_image_path: String,
  child_image_path: String,
  out: String,
  result_level: u32,
}

impl Task for ImageSearchTask {
  type Output = Vec<Vec<image::ResultPoint>>;
  type Error = String;
  type JsEvent = JsArray;
  fn perform(&self) -> Result<Self::Output, Self::Error> {
    let parent_image = image::Image::new(self.parent_image_path.clone());
    let child_image = image::Image::new(self.child_image_path.clone());
    let result = parent_image.search_child_image_point_from_parent_image(&child_image, self.result_level);

    Ok(result)
  }
  fn complete(
    self,
    mut cx: TaskContext,
    result: Result<Self::Output, Self::Error>,
  ) -> JsResult<Self::JsEvent> {
    let result = result.unwrap();
    let result_array = JsArray::new(&mut cx, result.len() as u32);
    for v in result.iter() {
      for (index, object) in v.iter().enumerate() {
        let result_object = JsObject::new(&mut cx);
        let x = object.x;
        let y = object.y;
        let x = cx.number(x);
        let y = cx.number(y);
        let hash_string = cx.string(object.hash_string.clone());
        let hamming_distance = cx.number(object.hamming_distance);
        result_object.set(&mut cx, "x", x).unwrap();
        result_object.set(&mut cx, "y", y).unwrap();
        result_object
          .set(&mut cx, "hash_string", hash_string)
          .unwrap();
        result_object
          .set(&mut cx, "hamming_distance", hamming_distance)
          .unwrap();
        result_array
          .set(&mut cx, index as u32, result_object)
          .unwrap();
      }
    }

    Ok(result_array)
  }
}

fn image_search(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let parent_image_path = cx.argument::<JsString>(0)?.value();
  let child_image_path = cx.argument::<JsString>(1)?.value();
  let options = cx.argument::<JsObject>(2)?;
  let out = options
    .get(&mut cx, "out")?
    .downcast::<JsString>()
    .or_throw(&mut cx)?
    .value();

  let result_level = options
    .get(&mut cx, "result_level")?
    .downcast::<JsNumber>()
    .or_throw(&mut cx)?
    .value();
  let f = cx.argument::<JsFunction>(3)?;
  let image_search_task = ImageSearchTask {
    parent_image_path,
    child_image_path,
    out,
    result_level: result_level as u32,
  };
  image_search_task.schedule(f);
  Ok(cx.undefined())
}

struct GetDHashTask {
  image_path: String,
}

impl Task for GetDHashTask {
  type Output = String;
  type Error = String;
  type JsEvent = JsString;
  fn perform(&self) -> Result<Self::Output, Self::Error> {
    let image = image::Image::new(self.image_path.clone());
    let result = image.get_d_hash();
    Ok(result)
  }

  fn complete(
    self,
    mut cx: TaskContext,
    result: Result<Self::Output, Self::Error>,
  ) -> JsResult<Self::JsEvent> {
    let result = result.unwrap();
    Ok(cx.string(result))
  }
}

fn get_d_hash(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let image_path = cx.argument::<JsString>(0)?.value();
  let f = cx.argument::<JsFunction>(1)?;
  let get_d_hash_task = GetDHashTask { image_path };
  get_d_hash_task.schedule(f);
  Ok(cx.undefined())
}

fn get_hamming_distance_by_hex_hash(mut cx: FunctionContext) -> JsResult<JsNumber> {
  let hash_1 = cx.argument::<JsString>(0)?.value();
  let hash_2 = cx.argument::<JsString>(1)?.value();
  let result = utils::get_hamming_distance_by_hex_hash(&hash_1, &hash_2);
  Ok(cx.number(result))
}

register_module!(mut m, {
  m.export_function("image_search", image_search)?;
  m.export_function("get_d_hash", get_d_hash)?;
  m.export_function(
    "get_hamming_distance_by_hex_hash",
    get_hamming_distance_by_hex_hash,
  )?;
  Ok(())
});
