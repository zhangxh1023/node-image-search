extern crate image;

use self::image::{imageops, GenericImageView, ImageBuffer, Pixel};
use std::path::Path;

use crate::utils;

pub struct ResultPoint {
  pub x: u32,
  pub y: u32,
  pub hash_string: String,
  pub hamming_distance: u32,
}

pub struct Image {
  pub path: String,
  pub image: image::DynamicImage,
}

impl Image {
  pub fn new(path: String) -> Self {
    let image = match image::open(&Path::new(&path)) {
      Ok(img) => img,
      Err(e) => panic!(e),
    };
    let mut path_slice: Vec<&str> = path.split(".").collect();
    let image_type = path_slice.pop().unwrap();
    if image_type != "jpeg" && image_type != "jpg" && image_type != "png" {
      panic!("Unexpected image type!");
    }
    Image { path, image }
  }

  fn get_size(&self) -> (u32, u32) {
    let width = self.image.width();
    let height = self.image.height();
    return (width, height);
  }

  pub fn get_d_hash(&self) -> String {
    let resize_width = 9;
    let resize_height = 8;
    // resize
    let resized_img =
      self
        .image
        .resize_exact(resize_width, resize_height, imageops::FilterType::Nearest);
    // 灰度化
    let resized_img = imageops::colorops::grayscale(&resized_img);
    // calculate difference
    let mut difference: Vec<u8> = vec![];
    for height in 0..resize_height {
      for width in 0..(resize_width - 1) {
        let v_before = match resized_img.get_pixel(width, height) {
          &image::Luma(v) => v,
        };
        let v_later = match resized_img.get_pixel(width + 1, height) {
          &image::Luma(v) => v,
        };
        if v_before > v_later {
          difference.push(1);
        } else {
          difference.push(0);
        }
      }
    }
    let mut decimal_value: i32 = 0;
    let mut img_hash_string = String::new();
    for (index, value) in difference.iter().enumerate() {
      if *value == 1 {
        decimal_value += *value as i32 * (2_i32.pow(index as u32 % 8)) as i32;
      }
      if index as u32 % 8 == 7 {
        let hex_str = format!("{:X}", decimal_value);
        let mut hash = hex_str.to_string();
        while hash.len() < 2 {
          hash = String::from("0") + &hash;
        }
        img_hash_string = format!("{}{}", img_hash_string, hash);
        decimal_value = 0;
      }
    }

    return img_hash_string;
  }

  pub fn sort_result_point_vector(&self, result_point_vec: &mut Vec<Vec<ResultPoint>>) {
    result_point_vec.sort_by(|a, b| {
      let a = &a[0];
      let b = &b[0];
      let a = a.hamming_distance;
      let b = b.hamming_distance;
      a.cmp(&b)
    });
  }

  pub fn try_to_push_result_point(
    &self,
    result_point_vec: &mut Vec<Vec<ResultPoint>>,
    max_length: usize,
    result_point: ResultPoint,
  ) {
    if result_point_vec.len() < max_length {
      let mut exist_same_hamming_distance_index: i32 = -1;
      for (index, item) in result_point_vec.iter().enumerate() {
        if item[0].hamming_distance == result_point.hamming_distance {
          exist_same_hamming_distance_index = index as i32;
          break;
        }
      }
      if exist_same_hamming_distance_index > -1 {
        result_point_vec[exist_same_hamming_distance_index as usize].push(result_point);
      } else {
        let new_vec = vec![result_point];
        result_point_vec.push(new_vec);
      }
    } else {
      let last = &result_point_vec[result_point_vec.len() - 1];
      if last[0].hamming_distance >= result_point.hamming_distance {
        let mut exist_same_hamming_distance_index: i32 = -1;
        for (index, item) in result_point_vec.iter().enumerate() {
          if item[0].hamming_distance == result_point.hamming_distance {
            exist_same_hamming_distance_index = index as i32;
            break;
          }
        }
        if exist_same_hamming_distance_index > -1 {
          result_point_vec[exist_same_hamming_distance_index as usize].push(result_point);
        } else {
          result_point_vec.pop();
          let new_vec = vec![result_point];
          result_point_vec.push(new_vec);
        }
      }
    }

    self.sort_result_point_vector(result_point_vec);
  }

  pub fn search_child_image_point_from_parent_image(
    &self,
    child_image: &Image,
    result_level: u32,
  ) -> Vec<Vec<ResultPoint>> {
    let child_image_d_hash = child_image.get_d_hash();
    let mut min_hamming_distance_for_point: Vec<Vec<ResultPoint>> = vec![];
    let (child_image_width, child_image_height) = child_image.get_size();
    let iterate_width = self.image.width() - child_image_width;
    let iterate_height = self.image.height() - child_image_height;

    for width in 0..iterate_width {
      for height in 0..iterate_height {
        let mut temp_image: image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> =
          ImageBuffer::new(child_image_width, child_image_height);
        for x in width..width + child_image_width {
          for y in height..height + child_image_height {
            let p = self.image.get_pixel(x, y);
            let p = p.to_rgb();
            temp_image.put_pixel(x - width, y - height, p);
          }
        }
        let temp_image = Image {
          path: String::new(),
          image: image::DynamicImage::ImageRgb8(temp_image),
        };
        let temp_image_d_hash = temp_image.get_d_hash();
        let hamming_distance =
          utils::get_hamming_distance_by_hex_hash(&temp_image_d_hash, &child_image_d_hash);

        let result_point = ResultPoint {
          x: width,
          y: height,
          hamming_distance,
          hash_string: temp_image_d_hash,
        };
        self.try_to_push_result_point(
          &mut min_hamming_distance_for_point,
          result_level as usize,
          result_point,
        );
      }
    }
    return min_hamming_distance_for_point;
  }

  pub fn mark_child_image_border_with_new_image(
    &self,
    child_image: &Image,
    path: &str,
    point: &Vec<Vec<ResultPoint>>,
  ) {
    let new_image = self.image.clone();
    let mut image_type: Vec<&str> = path.split(".").collect();
    let image_type = image_type.pop().unwrap();
    match new_image {
      image::DynamicImage::ImageRgb8(mut img) => {
        let (child_image_width, child_image_height) = child_image.get_size();
        let (parent_image_width, parent_image_height) = self.get_size();

        for v in point {
          for p in v {
            let ResultPoint {
              x: start_x,
              y: start_y,
              hash_string: _,
              hamming_distance: _,
            } = p;

            for x in 0..child_image_width {
              let point_x = x + start_x;
              if point_x < parent_image_width - 1 {
                let pixel = self.image.get_pixel(point_x, *start_y);
                let pixel = [255 - pixel[0], 255 - pixel[1], 255 - pixel[2]];
                img.put_pixel(point_x, *start_y, image::Rgb(pixel));
              } else {
                break;
              }
            }

            for y in 0..child_image_height {
              let point_y = y + start_y;
              if point_y < parent_image_height - 1 {
                let pixel = self.image.get_pixel(*start_x, point_y);
                let pixel = [255 - pixel[0], 255 - pixel[1], 255 - pixel[2]];
                img.put_pixel(*start_x, point_y, image::Rgb(pixel));
              } else {
                break;
              }
            }

            for x in 0..child_image_width {
              let point_x = x + start_x;
              let point_y = start_y + child_image_height;
              if point_y >= parent_image_height {
                break;
              }
              if point_x < parent_image_width - 1 {
                let pixel = self.image.get_pixel(point_x, point_y);
                let pixel = [255 - pixel[0], 255 - pixel[1], 255 - pixel[2]];
                img.put_pixel(point_x, point_y as u32, image::Rgb(pixel));
              } else {
                break;
              }
            }

            for y in 0..child_image_height {
              let point_y = y + start_y;
              let point_x = start_x + child_image_width;
              if point_x >= parent_image_width {
                break;
              }
              if point_y < parent_image_height - 1 {
                let pixel = self.image.get_pixel(point_x, point_y);
                let pixel = [255 - pixel[0], 255 - pixel[1], 255 - pixel[2]];
                img.put_pixel(point_x as u32, point_y, image::Rgb(pixel));
              } else {
                break;
              }
            }
          }
        }
        if image_type == "png" {
          img
            .save_with_format(Path::new(path), image::ImageFormat::Png)
            .expect("save image error");
        } else {
          img
            .save_with_format(Path::new(path), image::ImageFormat::Jpeg)
            .expect("save image error");
        }
      }
      image::DynamicImage::ImageRgba8(mut img) => {
        let (child_image_width, child_image_height) = child_image.get_size();
        let (parent_image_width, parent_image_height) = self.get_size();

        for v in point {
          for p in v {
            let ResultPoint {
              x: start_x,
              y: start_y,
              hash_string: _,
              hamming_distance: _,
            } = p;

            for x in 0..child_image_width {
              let point_x = x + start_x;
              if point_x < parent_image_width - 1 {
                let pixel = self.image.get_pixel(point_x, *start_y);
                let pixel = [255 - pixel[0], 255 - pixel[1], 255 - pixel[2], pixel[3]];
                img.put_pixel(point_x, *start_y, image::Rgba(pixel));
              } else {
                break;
              }
            }

            for y in 0..child_image_height {
              let point_y = y + start_y;
              if point_y < parent_image_height - 1 {
                let pixel = self.image.get_pixel(*start_x, point_y);
                let pixel = [255 - pixel[0], 255 - pixel[1], 255 - pixel[2], pixel[3]];
                img.put_pixel(*start_x, point_y, image::Rgba(pixel));
              }
            }

            for x in 0..child_image_width {
              let point_x = x + start_x;
              let point_y = start_y + child_image_height;
              if point_y >= parent_image_height {
                break;
              }
              if point_x < parent_image_width - 1 {
                let pixel = self.image.get_pixel(point_x, point_y);
                let pixel = [255 - pixel[0], 255 - pixel[1], 255 - pixel[2], pixel[3]];
                img.put_pixel(point_x, point_y as u32, image::Rgba(pixel));
              } else {
                break;
              }
            }

            for y in 0..child_image_height {
              let point_y = y + start_y;
              let point_x = start_x + child_image_width;
              if point_x >= parent_image_width {
                break;
              }
              if point_y < parent_image_height - 1 {
                let pixel = self.image.get_pixel(point_x, point_y);
                let pixel = [255 - pixel[0], 255 - pixel[1], 255 - pixel[2], pixel[3]];
                img.put_pixel(point_x as u32, point_y, image::Rgba(pixel));
              } else {
                break;
              }
            }
          }
        }
        if image_type == "png" {
          img
            .save_with_format(Path::new(path), image::ImageFormat::Png)
            .expect("save image error");
        } else {
          img
            .save_with_format(Path::new(path), image::ImageFormat::Jpeg)
            .expect("save image error");
        }
      }
      _ => (),
    };
  }
}
