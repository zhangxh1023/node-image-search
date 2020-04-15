mod image;
mod utils;

fn main() {
  let main_image = image::Image::new(String::from("../img/big.jpeg"));
  let min_image = image::Image::new(String::from("../img/small.jpeg"));

  main_image.search_child_image_point_from_parent_image(&min_image, 2);
}