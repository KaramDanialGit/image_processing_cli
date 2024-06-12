use image::{buffer::ConvertBuffer, GrayImage, ImageBuffer, ImageFormat, Luma, Pixel};

fn convert_to_gray_png(image_name: &String) -> Result<(), &str> {
    let get_path: String = format!("images/{}", image_name);
    let set_path: String = format!("new_images/gray_{}", image_name);
    let img = manager::get_image(&get_path);

    if img.is_none() {
        return Err("Image not found");
    }

    let gray_image: GrayImage = img.unwrap().convert();

    ImageLuma8(gray_image).save(set_path).unwrap();

    Ok(())
}
