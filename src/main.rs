use rand::Rng;

type Result<T> = core::result::Result<T, image::error::ImageError>;
use rand::thread_rng;
fn fifty_fifty() -> bool {
    thread_rng().gen()
}
fn main() -> Result<()> {
    const IMGX: u32 = 800;
    const IMGY: u32 = 800;
    const BLOCK_X: u32 = 100;
    const BLOCK_Y: u32 = 100;

    let colors = [
        [248u8, 235, 243, 255],
        [19, 7, 16, 255],
        [218, 153, 195, 255],
        [43, 116, 84, 255],
        [84, 114, 193, 255],
    ];

    let mut imgbuf = image::ImageBuffer::new(IMGX, IMGY);
    for pixel in imgbuf.pixels_mut() {
        *pixel = image::Rgba(colors[1]);
    }

    let mut x = 0;
    let mut y = 0;
    while y <= IMGY {
        while x <= IMGX {
            let a = thread_rng().gen();
            let color = match thread_rng().gen_range(0..=99) {
                0..=9 => image::Rgba([colors[0][0], colors[0][1], colors[0][2], a]),
                10..=49 => image::Rgba([colors[2][0], colors[2][1], colors[2][2], a]),
                50..=90 => image::Rgba([colors[3][0], colors[3][1], colors[3][2], a]),
                _ => image::Rgba([colors[4][0], colors[4][1], colors[4][2], a]),
            };
            'X: for i in x..BLOCK_X + x {
                'Y: for j in y..BLOCK_Y + y {
                    if let Some(pixel) = imgbuf.get_pixel_mut_checked(i, j) {
                        *pixel = color;
                    } else {
                        continue 'X;
                    }
                }
            }
            x += BLOCK_X;
        }
        y += BLOCK_Y;
        x = 0;
    }
    imgbuf.save("profile.png").unwrap();
    Ok(())
}
