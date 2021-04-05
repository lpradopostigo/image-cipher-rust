mod bytestream;
mod chaoticmap;
use bytestream::ByteStream;
use chaoticmap::ChaoticMap;
use image::{io::Reader, DynamicImage, ImageFormat, RgbaImage};
use std::io::Cursor;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use wasm_bindgen::prelude::*;

#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"),wasm_bindgen)]
pub fn cipher_image(input_data: &[u8], key: &str) -> Vec<u8> {
    let input_img = image_from_raw(input_data).into_rgba8();
    let (width, height) = input_img.dimensions();
    let mut input_img_vector = input_img.into_raw();
    let key_vector = generate_key_vector(key, input_img_vector.len());

    for i in 0..input_img_vector.len() {
        input_img_vector[i] ^= key_vector[i];
    }

    let ouput_img =
        DynamicImage::ImageRgba8(RgbaImage::from_raw(width, height, input_img_vector).unwrap());
    let mut ouput_data: Vec<u8> = Vec::new();
    ouput_img
        .write_to(&mut ouput_data, ImageFormat::Png)
        .unwrap();

    ouput_data
}

fn image_from_raw(raw_data: &[u8]) -> DynamicImage {
    match Reader::with_format(Cursor::new(raw_data), ImageFormat::Png).decode() {
        Ok(image) => image,
        Err(e) => {
            println!("Error while decoding raw data {:?}", e);
            DynamicImage::new_rgba8(0, 0)
        }
    }
}

fn generate_key_vector(key: &str, size: usize) -> Vec<u8> {
    let mut map = ChaoticMap::new(generate_seed(key), 4.);
    let mut key_vector: Vec<u8> = vec![0; size];
    for i in 0..size {
        key_vector[i] = (map.get_term(i) * 256.) as u8;
    }
    key_vector
}

/// For a empty string always return 1
fn generate_seed(key: &str) -> f64 {
    let bytes = key.bytes();
    let mut even = 0.;
    let mut odd = 0.;
    let mut k = 1. / (1u64 << 8 * key.len() / 2) as f64;

    for byte in bytes.rev() {
        let mut byte_stream = ByteStream::new(byte);
        let mut flag = false;
        while !byte_stream.is_consumed() {
            match flag {
                false => even += byte_stream.consume_bit() as f64 * k,
                true => {
                    odd += byte_stream.consume_bit() as f64 * k;
                    k *= 2.;
                }
            }
            flag = !flag;
        }
    }

    let seed = (even + odd).fract();
    if seed == 0. {
        return k + ((even + odd) / 10.);
    }
    seed
}
