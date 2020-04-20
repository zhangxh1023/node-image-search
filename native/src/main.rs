mod image;
mod utils;

fn main() {
  let main_image = image::Image::new(String::from("../examples/img/2/big.jpg"));
  let min_image = image::Image::new(String::from("../examples/img/2/small.png"));

  let result = main_image.search_child_image_point_from_parent_image(&min_image, 1);
  main_image.mark_child_image_border_with_new_image(&min_image, "./temp.png", &result);
}
