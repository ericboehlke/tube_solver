use imageproc::template_matching::{match_template, MatchTemplateMethod};
use std::path::Path;

#[cfg(test)]
mod matching_tests {
    use super::*;

    #[test]
    fn test_match_tube_level_5() {
        let mut level_img = image::open(&Path::new("screenshots/level5.png")).unwrap();
        let mut gray_level_img = level_img.as_mut_luma8().unwrap();
        image::imageops::dither(&mut gray_level_img, &image::imageops::BiLevel);
        gray_level_img.save("cat.png").unwrap();
        for i in 0..3 {
            println!("hi");
        }
        let gray_level_img = level_img.grayscale();
        let tube_img = image::open(&Path::new("screenshots/tube.png")).unwrap();
        let gray_tube_img = tube_img.grayscale();
        let gray_tube_img = tube_img.as_luma8().unwrap();
        // match_template(gray_level_img, gray_tube_img, MatchTemplateMethod::CrossCorrelation);
    }
}