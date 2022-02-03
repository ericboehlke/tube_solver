use image;
use imageproc::template_matching;
use std::{cmp::Ordering, path::Path};
use tubes::{LiquidColor, Tube, TubeState};

fn find_local_minima<T>(img: &image::ImageBuffer<image::Luma<T>, Vec<T>>) -> Vec<(u32, u32)>
where
    T: image::Primitive + 'static,
{
    let mut minima = Vec::new();
    for width_idx in 1..img.width() - 1 {
        for height_idx in 1..img.height() - 1 {
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

fn lonely_minima<T>(
    img: &image::ImageBuffer<image::Luma<T>, Vec<T>>,
    minima: Vec<(u32, u32)>,
) -> Vec<(u32, u32)>
where
    T: image::Primitive + 'static,
{
    let how_local = 20.0;
    let mut local_minima = Vec::new();
    fn distance(p1: (u32, u32), p2: (u32, u32)) -> f32 {
        return f32::sqrt(
            (p2.0 as f32 - p1.0 as f32) * (p2.0 as f32 - p1.0 as f32)
                + (p2.1 as f32 - p1.1 as f32) * (p2.1 as f32 - p1.1 as f32),
        );
    }
    for m in 0..minima.len() {
        let mut is_local_minimum = true;
        for n in 0..minima.len() {
            if distance(minima[m], minima[n]) < how_local {
                if img.get_pixel(minima[m].0, minima[m].1)[0]
                    > img.get_pixel(minima[n].0, minima[n].1)[0]
                {
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

fn crop_level(level_img: &image::DynamicImage) -> image::RgbImage {
    let level_img = level_img.to_rgb8();
    let top_crop_ratio = 0.25;
    let bot_crop_ratio = 0.15;
    let top_crop_pixels = (level_img.height() as f32 * top_crop_ratio) as u32;
    let bot_crop_pixels =
        ((1.0 - top_crop_ratio - bot_crop_ratio) * level_img.height() as f32) as u32;
    let level_img = image::imageops::crop_imm(
        &level_img,
        0,
        top_crop_pixels,
        level_img.width(),
        bot_crop_pixels,
    )
    .to_image();
    let level_img = image::imageops::resize(
        &level_img,
        level_img.width() / 10,
        level_img.height() / 10,
        image::imageops::FilterType::Triangle,
    );
    return level_img;
}

fn preprocess_tube(tube_img: &image::DynamicImage) -> image::GrayImage {
    let tube_img = tube_img.to_luma8();
    let tube_img = image::imageops::resize(
        &tube_img,
        tube_img.width() / 10,
        tube_img.height() / 10,
        image::imageops::FilterType::Triangle,
    );
    return tube_img;
}

fn find_tubes(level_img: &image::RgbImage) -> Vec<(u32, u32)> {
    let level_img = image::DynamicImage::ImageRgb8(level_img.clone()).to_luma8();
    let full_tube_img = image::open(&Path::new("screenshots/full_tube.png")).unwrap();
    let full_tube_img = preprocess_tube(&full_tube_img);
    // let _ = full_tube_img.save("full_tube.png");
    let empty_tube_img = image::open(&Path::new("screenshots/empty_tube.png")).unwrap();
    let empty_tube_img = preprocess_tube(&empty_tube_img);
    // let _ = empty_tube_img.save("empty_tube.png");
    let full_matched = template_matching::match_template(
        &level_img,
        &full_tube_img,
        template_matching::MatchTemplateMethod::SumOfSquaredErrorsNormalized,
    );
    let empty_matched = template_matching::match_template(
        &level_img,
        &empty_tube_img,
        template_matching::MatchTemplateMethod::SumOfSquaredErrorsNormalized,
    );
    let mut combined_matched: image::GrayImage =
        image::GrayImage::new(full_matched.width(), empty_matched.height());
    for width_index in 0..full_matched.width() {
        for height_index in 0..full_matched.height() {
            let full_pixel = *full_matched.get_pixel(width_index, height_index);
            let empty_pixel = *empty_matched.get_pixel(width_index, height_index);
            let combined_pixel_value = full_pixel[0] * empty_pixel[0];
            let combined_pixel_value = (combined_pixel_value * 255.0) as u8;
            combined_matched.put_pixel(
                width_index,
                height_index,
                image::Luma([combined_pixel_value]),
            );
        }
    }
    let minima = find_local_minima(&combined_matched);
    let mut thresholded_minima = Vec::new();
    for (x, y) in minima {
        let pixel = combined_matched.get_pixel(x, y);
        if pixel[0] < 100 {
            thresholded_minima.push((x, y));
        }
    }
    let minima = lonely_minima(&combined_matched, thresholded_minima);
    let mut adjusted_minima = Vec::new();
    for (x, y) in minima {
        adjusted_minima.push((x + 15, y + 30));
    }
    return adjusted_minima;
}

fn point_cmp(a: &(u32, u32), b: &(u32, u32)) -> Ordering {
    let a = (a.0 as i32, a.1 as i32);
    let b = (b.0 as i32, b.1 as i32);
    if (a.1 - b.1).abs() <= 3 {
        if a.0 > b.0 {
            return Ordering::Greater;
        } else if a.0 < b.0 {
            return Ordering::Less;
        } else {
            return Ordering::Equal;
        }
    } else if a.1 > b.1 {
        return Ordering::Greater;
    } else {
        return Ordering::Less;
    }
}

fn extract_tube_colors(level_img: &image::RgbImage, tube_centers: Vec<(u32, u32)>) -> TubeState {
    let mut tube_centers = tube_centers.clone();
    tube_centers.sort_by(|a, b| point_cmp(a, b));
    let color_spacing = 11;
    let mut tubes = Vec::new();
    let mut new_level_img = level_img.clone();
    for (x, y) in tube_centers {
        let mut tube_colors = Vec::new();
        for layer in [2, 1, 0, -1] {
            let dy = layer * color_spacing as i32;
            println!("{}, {}", x, y as i32 + dy);
            let color = level_img.get_pixel(x, (y as i32 + dy) as u32);
            new_level_img.put_pixel(x, (y as i32 + dy) as u32, image::Rgb([255, 0, 0]));
            let liquid_color = LiquidColor::new(color[0], color[1], color[2]);
            tube_colors.push(liquid_color);
        }
        tubes.push(Tube::from_vec(tube_colors));
    }
    let _ = new_level_img.save("tube_color_locations.png");
    return TubeState { tubes };
}

#[cfg(test)]
mod matching_tests {
    use tubes::EMPTY_TUBE;

    use super::*;

    #[test]
    fn test_match_tube_level_5() {
        let level_img = image::open(&Path::new("screenshots/level5.png")).unwrap();
        let mut level_img = crop_level(&level_img);
        // let _ = level_img.save("level.png");
        let minima = find_tubes(&level_img);
        for c in minima {
            level_img.put_pixel(c.0, c.1, image::Rgb([255, 0, 0]));
            println!("{:?}", c);
        }
        let _ = level_img.save("result_level_5.png");
    }

    #[test]
    fn test_match_tube_level_8() {
        let level_img = image::open(&Path::new("screenshots/level8.png")).unwrap();
        let level_img = crop_level(&level_img);
        // let _ = level_img.save("level.png");
        let tube_centers = find_tubes(&level_img);
        let mut display_level_img = level_img.clone();
        // let _ = level_img.save("level.png");
        for c in &tube_centers {
            display_level_img.put_pixel(c.0, c.1, image::Rgb([255, 0, 0]));
            println!("{:?}", c);
        }
        let _ = display_level_img.save("result_level_8.png");
        let tubes = extract_tube_colors(&level_img, tube_centers.clone());
        println!("{}", tubes);
        for i in &tubes.tubes {
            for c in i.layers {
                println!("{:?}", c);
            }
        }
        let green = LiquidColor::WhoKnows(129, 211, 133);
        let orange = LiquidColor::WhoKnows(219, 144, 81);
        let blue = LiquidColor::WhoKnows(56, 46, 187);
        let pink = LiquidColor::WhoKnows(217, 103, 124);
        let red = LiquidColor::WhoKnows(181, 57, 45);
        let expected_tubes = TubeState { tubes: vec![
            Tube::new(green, orange, green, blue),
            Tube::new(orange, pink, pink, orange),
            Tube::new(pink, red, blue, red),
            Tube::new(blue, red, green, pink),
            Tube::new(blue, green, red, orange),
            EMPTY_TUBE,
            EMPTY_TUBE,
        ] };
        assert_eq!(tubes, expected_tubes);
    }
}
