extern crate neon;
use neon::prelude::*;
mod image;

struct ImageSearchTask {
  main_image_path: String,
  min_image_path: String,
}

impl Task for ImageSearchTask {
  type Output = Vec<image::ChildImg>;
  type Error = String;
  type JsEvent = JsArray;
  fn perform(&self) -> Result<Self::Output, Self::Error> {
    let result = image::search(self.main_image_path.clone(), self.min_image_path.clone());
    Ok(result)
  }
  fn complete(self, mut cx: TaskContext, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
    let result = result.unwrap();
    let result_array = JsArray::new(&mut cx, result.len() as u32);
    for (index, object) in result.iter().enumerate() {
      let result_object = JsObject::new(&mut cx);
      let (x, y) = object.point;
      let x = cx.number(x);
      let y = cx.number(y);
      let hash_string = cx.string(object.hash_string.clone());
      let hamming_distance = cx.number(object.hamming_distance);
      result_object.set(&mut cx, "x", x).unwrap();
      result_object.set(&mut cx, "y", y).unwrap();
      result_object.set(&mut cx, "hash_string", hash_string).unwrap();
      result_object.set(&mut cx, "hamming_distance", hamming_distance).unwrap();
      result_array.set(&mut cx, index as u32, result_object).unwrap();
    }

    Ok(result_array)
  }
}

fn image_search(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let main_image_path = cx.argument::<JsString>(0)?.value();
  let min_image_path = cx.argument::<JsString>(1)?.value();
  let f = cx.argument::<JsFunction>(2)?;
  let image_search_task = ImageSearchTask {
    main_image_path,
    min_image_path,
  };
  image_search_task.schedule(f);
  Ok(cx.undefined())
}

register_module!(mut m, {
  m.export_function("image_search", image_search)?;
  Ok(())
});