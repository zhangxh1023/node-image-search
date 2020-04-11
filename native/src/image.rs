extern crate image;

use self::image::{imageops, GenericImageView, ImageBuffer};
use std::path::Path;

#[derive(Debug)]
pub struct ChildImg {
  pub point: (u32, u32),
  pub hash_string: String,
  pub hamming_distance: u32,
}

pub fn convert_to_binary_from_hex(hex: &str) -> String {
  hex.chars().map(to_binary).collect()
}

pub fn to_binary(c: char) -> &'static str {
  match c {
    '0' => "0000",
    '1' => "0001",
    '2' => "0010",
    '3' => "0011",
    '4' => "0100",
    '5' => "0101",
    '6' => "0110",
    '7' => "0111",
    '8' => "1000",
    '9' => "1001",
    'A' => "1010",
    'B' => "1011",
    'C' => "1100",
    'D' => "1101",
    'E' => "1110",
    'F' => "1111",
    _ => "",
  }
}

pub fn search(main_img_path: String, min_img_path: String) ->  Vec<ChildImg> {
  let main_path = &main_img_path;
  let min_path = &min_img_path;
  let main_img = image::open(&Path::new(main_path)).unwrap();
  let min_img = image::open(&Path::new(min_path)).unwrap();
  let main_img_width = main_img.width();
  let main_img_height = main_img.height();
  let min_img_width = min_img.width();
  let min_img_height = min_img.height();

  if main_img_width < min_img_width || main_img_height < min_img_height {
    panic!("img size error");
  }

  let min_img_hash_string = get_hash_string(&min_img);

  let iterate_width = main_img_width - min_img_width;
  let iterate_height = main_img_height - min_img_height;

  let mut minimal_hamming_distance_child_img: Vec<ChildImg> = vec![];
  for width in 0..iterate_width {
    for height in 0..iterate_height {
      let point = (width, height);
      let img = create_image_buffer_from_main_img(
        width,
        width + min_img_width,
        height,
        height + min_img_height,
        &main_img,
      );
      let hash_string = get_hash_string(&img);
      let mut hamming_distance = 0;
      let mut binary_hash_string = convert_to_binary_from_hex(&hash_string);
      let mut binary_min_img_hash_string = convert_to_binary_from_hex(&min_img_hash_string);
      while binary_hash_string.len() < 64 {
        binary_hash_string = String::from("0") + &binary_hash_string;
      }
      while binary_min_img_hash_string.len() < 64 {
        binary_min_img_hash_string = String::from("0") + &binary_min_img_hash_string;
      }
      for index in 0..64 {
        if &binary_hash_string.as_bytes()[index] != &binary_min_img_hash_string.as_bytes()[index] {
          hamming_distance += 1;
        }
      }

      if minimal_hamming_distance_child_img.len() == 0 {
        minimal_hamming_distance_child_img.push(ChildImg {
          point,
          hash_string,
          hamming_distance,
        })
      } else {
        let child_img = minimal_hamming_distance_child_img
          .get(0)
          .expect("minimal_hamming_distance_child_img index overflow");
        if child_img.hamming_distance == hamming_distance {
          minimal_hamming_distance_child_img.push(ChildImg {
            point,
            hash_string,
            hamming_distance,
          })
        } else if child_img.hamming_distance > hamming_distance {
          minimal_hamming_distance_child_img = vec![];
          minimal_hamming_distance_child_img.push(ChildImg {
            point,
            hash_string,
            hamming_distance,
          })
        }
      }
    }
  }

  println!("{:?}", minimal_hamming_distance_child_img);

  // let (x, y) = minimal_hamming_distance_child_img[0].point;
  // let mut temp_img: ImageBuffer<image::Rgba<u16>, std::vec::Vec<u16>> =
  // ImageBuffer::new(main_img_width, main_img_height);
  // let out_image = &image::DynamicImage::ImageRgba16(temp_img);
  // mark_square(x, y, &main_img, &min_img, out_image, &minimal_hamming_distance_child_img);

  // temp_img.save(&Path::new("./temp.jpeg")).unwrap();
  minimal_hamming_distance_child_img
}

pub fn mark(x: u32, y: u32, temp_img: &mut image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>) {
  let mut temp_pixel = match temp_img.get_pixel(x, y) {
    image::Rgb(v) => *v,
  };
  temp_pixel[0] = 255 - temp_pixel[0];
  temp_pixel[1] = 255 - temp_pixel[1];
  temp_pixel[2] = 255 - temp_pixel[2];
  temp_img.put_pixel(x, y, image::Rgb([temp_pixel[0], temp_pixel[1], temp_pixel[2]]));
  temp_img.put_pixel(x - 1, y, image::Rgb([temp_pixel[0], temp_pixel[1], temp_pixel[2]]));
  temp_img.put_pixel(x + 1, y, image::Rgb([temp_pixel[0], temp_pixel[1], temp_pixel[2]]));
  temp_img.put_pixel(x, y - 1, image::Rgb([temp_pixel[0], temp_pixel[1], temp_pixel[2]]));
  temp_img.put_pixel(x, y + 1, image::Rgb([temp_pixel[0], temp_pixel[1], temp_pixel[2]]));
  temp_img.put_pixel(x - 1, y - 1, image::Rgb([temp_pixel[0], temp_pixel[1], temp_pixel[2]]));
  temp_img.put_pixel(x + 1, y + 1, image::Rgb([temp_pixel[0], temp_pixel[1], temp_pixel[2]]));
  temp_img.put_pixel(x - 1, y + 1, image::Rgb([temp_pixel[0], temp_pixel[1], temp_pixel[2]]));
  temp_img.put_pixel(x + 1, y - 1, image::Rgb([temp_pixel[0], temp_pixel[1], temp_pixel[2]]));
}

pub fn mark_square(start_x: u32, start_y: u32, main_image: &image::DynamicImage, min_image: &image::DynamicImage, out_image: &image::DynamicImage, minimal_hamming_distance_child_img: &Vec<ChildImg>) {

}

pub fn get_hash_string(img: &image::DynamicImage) -> String {
  let resize_width = 9;
  let resize_height = 8;
  // resize
  let resized_img = img.resize_exact(resize_width, resize_height, imageops::FilterType::Nearest);
  // 灰度化
  let resized_img = imageops::colorops::grayscale(&resized_img);
  // 计算差异值
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

pub fn create_image_buffer_from_main_img(
  start_width: u32,
  end_width: u32,
  start_height: u32,
  end_height: u32,
  main_img: &image::DynamicImage,
) -> image::DynamicImage {
  let mut temp_img: ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>> =
    ImageBuffer::new(end_width - start_width, end_height - start_height);
  for width in start_width..end_width {
    for height in start_height..end_height {
      temp_img.put_pixel(
        width - start_width,
        height - start_height,
        main_img.get_pixel(width, height),
      );
    }
  }
  return image::DynamicImage::ImageRgba8(temp_img);
}
