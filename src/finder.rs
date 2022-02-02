use image::{
    imageops::crop_imm, imageops::resize, imageops::FilterType, DynamicImage, GenericImage,
    GrayImage, Luma, Rgb,
};
use image::{ImageBuffer, Primitive, GenericImageView};
use imageproc::drawing::Canvas;
use imageproc::template_matching::{self, match_template, MatchTemplateMethod};
use imageproc::{
    corners::corners_fast9, gradients::sobel_gradient_map, map::ChannelMap,
    template_matching::find_extremes,
};
use std::path::Path;
use std::cmp;

fn find_local_minima<T>(img: &ImageBuffer<Luma<T>, Vec<T>>) -> Vec<(u32, u32)>
where
    T: Primitive + 'static,
{
    let mut minima = Vec::new();
    for width_idx in 1..img.width()-1 {
        for height_idx in 1..img.height()-1 {
            let value = img.get_pixel(width_idx, height_idx)[0];
            let x = width_idx as i32;
            let y = height_idx as i32;
            let mut is_minimum = true;
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if value >= img.get_pixel((x + dx) as u32, (y + dy) as u32)[0] {
                    is_minimum = false;
                    break;
                }
            }
            if is_minimum {
                minima.push((width_idx, height_idx));
            }
        }
    }
    return minima;
}


fn lonely_minima<T>(img: &ImageBuffer<Luma<T>, Vec<T>>, minima: Vec<(u32, u32)>) -> Vec<(u32, u32)>
where
    T: Primitive + 'static,
{
    let how_local = 20.0;
    let mut local_minima = Vec::new();
    fn distance(p1: (u32, u32), p2: (u32, u32)) -> f32 {
        return f32::sqrt((p2.0 as f32 - p1.0 as f32) * (p2.0 as f32 - p1.0 as f32) + (p2.1 as f32 - p1.1 as f32) * (p2.1 as f32 - p1.1 as f32));
    }
    for m in 0..minima.len() {
        let mut is_local_minimum = true;
        for n in 0..minima.len() {
            if distance(minima[m], minima[n]) < how_local {
                if img.get_pixel(minima[m].0, minima[m].1)[0] > img.get_pixel(minima[n].0, minima[n].1)[0] {
                    is_local_minimum = false;
                }
            }
        }
        if is_local_minimum {
            local_minima.push(minima[m]);
        }
    } 
    return local_minima;
}

#[cfg(test)]
mod matching_tests {
    use super::*;

    #[test]
    fn test_match_tube_level_5() {
        let level_img = image::open(&Path::new("screenshots/level8.png")).unwrap();
        let gray_level_img = level_img.to_luma8();
        let top_crop_ratio = 0.25;
        let bot_crop_ratio = 0.15;
        let top_crop_pixels = (gray_level_img.height() as f32 * top_crop_ratio) as u32;
        let bot_crop_pixels =
            ((1.0 - top_crop_ratio - bot_crop_ratio) * gray_level_img.height() as f32) as u32;
        let cropped_level_img = crop_imm(
            &gray_level_img,
            0,
            top_crop_pixels,
            gray_level_img.width(),
            bot_crop_pixels,
        )
        .to_image();
        let smol_gray_level_img = resize(
            &cropped_level_img,
            cropped_level_img.width() / 10,
            cropped_level_img.height() / 10,
            FilterType::Triangle,
        );
        let _ = smol_gray_level_img.save("smol_level.png");
        let tube_img = image::open(&Path::new("screenshots/full_tube.png")).unwrap();
        let etube_img = image::open(&Path::new("screenshots/empty_tube.png")).unwrap();
        let gray_tube_img = tube_img.to_luma8();
        let gray_etube_img = etube_img.to_luma8();
        let smol_gray_tube_img = resize(
            &gray_tube_img,
            gray_tube_img.width() / 10,
            gray_tube_img.height() / 10,
            FilterType::Triangle,
        );
        let smol_gray_etube_img = resize(
            &gray_etube_img,
            gray_etube_img.width() / 10,
            gray_etube_img.height() / 10,
            FilterType::Triangle,
        );
        let _ = smol_gray_tube_img.save("smol_tube.png");
        let _ = smol_gray_etube_img.save("smol_empty_tube.png");
        let matched = match_template(
            &smol_gray_level_img,
            &smol_gray_tube_img,
            MatchTemplateMethod::SumOfSquaredErrorsNormalized,
        );
        let ematched = match_template(
            &smol_gray_level_img,
            &smol_gray_etube_img,
            MatchTemplateMethod::SumOfSquaredErrorsNormalized,
        );
        let mut new_matched: GrayImage = GrayImage::new(matched.width(), matched.height());
        let extremes = find_extremes(&smol_gray_level_img);
        for idx_width in 0..matched.width() {
            for idx_height in 0..matched.height() {
                let pixel = *matched.get_pixel(idx_width, idx_height);
                let epixel = *ematched.get_pixel(idx_width, idx_height);
                let new_pix_value = pixel[0] * epixel[0];
                if new_pix_value < 0.1 {
                    println!("pix1: {} at {}, {}", new_pix_value, idx_width, idx_height);
                }
                let p_val = (new_pix_value * 255.0) as u8;
                new_matched.put_pixel(idx_width, idx_height, Luma([p_val]));
            }
        }
        fn brighten(l: ChannelMap<Luma<u16>, u16>) -> Luma<u16> {
            return Luma([l.0[0] * 100]);
        }
        // let matched_prime = sobel_gradient_map(&new_matched, brighten);
        let matched_corners = corners_fast9(&new_matched, 30);
        let minima = find_local_minima(&new_matched);
        let mut matched_corners_img = DynamicImage::ImageLuma8(new_matched.clone()).to_rgb8();
        // for c in matched_corners {
            // matched_corners_img.put_pixel(c.x, c.y, Rgb([255, 0, 0]));
            // println!("{:?}", c);
        // }
        let mut new_minima = Vec::new();
        for c in minima {
            let pixel = new_matched.get_pixel(c.0, c.1);
            if pixel[0] < 100 {
                new_minima.push(c);
                // matched_corners_img.put_pixel(c.0, c.1, Rgb([255, 0, 0]));
            }
        }
        let minima = lonely_minima(&new_matched, new_minima);
        for c in minima {
            let pixel = new_matched.get_pixel(c.0, c.1);
            println!("pix: {} at {}, {}", pixel[0], c.0, c.1);
            matched_corners_img.put_pixel(c.0, c.1, Rgb([255, 0, 0]));
            println!("{:?}", c);
        }
        let _ = matched_corners_img.save("matched_corners.png");
        let _ = new_matched.save("matched.png");
        println!("{}, {:?}", extremes.max_value, extremes.max_value_location)
        //let result = matched.save(Path::new("matched.png"));
    }
}
