extern crate ranger;

use ranger::rendering::image;

#[test]
fn image_point_create() {
    let p = image::ImgPoint::new();
    assert_eq!(p.x, 0);
    assert_eq!(p.y, 0);
}

#[test]
fn image_point_set_xy() {
    let mut p = image::ImgPoint::new();
    p.set_xy(5, 5);
    assert_eq!(p.x, 5);
    assert_eq!(p.y, 5);

    p.x = 6;
    assert_eq!(p.x, 6);
    assert_eq!(p.y, 5);
}

#[test]
fn image_buffer_create() {
    let mut buf: Vec<u8> = vec![];
    buf.push(9);
    assert_eq!(buf.len(), 1);
}

#[test]
fn image_size() {
    let img = image::RGBA::new(2, 2);
    assert_eq!(img.bounds().size(), 4);
}

#[test]
fn image_buf_size() {
    let mut img = image::RGBA::new(2, 2);
    assert_eq!(img.size(), 4 * 4);
}

#[test]
fn image_set_pixel_direct() {
    let mut img = image::RGBA::new(2, 2);
    img.pix[0] = 128;
    assert_eq!(img.pix[0], 128);
    img.pix[1] = 255;
    assert_eq!(img.pix[1], 255);
}

#[test]
fn image_set_pixel_by_buf() {
    let mut img = image::RGBA::new(2, 2);
    img.buf()[0] = 120;
    assert_eq!(img.buf()[0], 120);
}

#[test]
fn image_set_pixel_index_op() {
    let mut img = image::RGBA::new(2, 2);

    img[0] = 120;
    assert_eq!(img[0], 120);
}

#[test]
fn image_at_pixel() {
    // We need mut because we are using img[...] to set blue
    // component of pixel.
    let mut img = image::RGBA::new(2, 2);
    // Sets 'B' of pixel at 0,0
    img[2] = 120; // <-- requires mutability.

    //                        R  G   B   A
    assert_eq!(img.at(0, 0), [0, 0, 120, 0]);
}

#[test]
fn image_at_pixel2() {
    // We need mut because we are using img[...] to set red
    // component of pixel.
    let mut img = image::RGBA::new(2, 2);
    // Sets 'R' of pixel at 1,0
    img[1 * 4] = 127; // <-- requires mutability.

    //                        R  G   B   A
    assert_eq!(img.at(1, 0), [127, 0, 0, 0]);
}

#[test]
fn image_set_pixel() {
    // We need mut because we are using img.set() to set color.
    let mut img = image::RGBA::new(2, 2);
    // Sets pixel 1,0 = Orange
    img.set(1, 0, [255, 127, 0, 0]);

    assert_eq!(img.at(1, 0), [255, 127, 0, 0]);
}

#[test]
fn image_set_pixel_by_components() {
    let mut img = image::RGBA::new(2, 2);
    // Sets pixel 1,0 = Orange
    img.set_components(1, 0, 255, 127, 0, 0);

    assert_eq!(img.at(1, 0), [255, 127, 0, 0]);
}
