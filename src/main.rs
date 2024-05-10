use fantoccini::ClientBuilder;
use image::Rgba;
use rand::Rng;

type Result<T> = core::result::Result<T, image::error::ImageError>;
use rand::thread_rng;



async fn get_color() -> String {
    let client = ClientBuilder::rustls()
        .connect("http://localhost:4444")
        .await
        .unwrap();
    client
        .goto("https://www.realtimecolors.com/")
        .await
        .unwrap();
    if thread_rng().gen() {
        client
            .find(fantoccini::Locator::Id("theme"))
            .await
            .unwrap()
            .click()
            .await
            .unwrap();
    }
    for _ in 0..thread_rng().gen_range(1..=5) {
        client
            .find(fantoccini::Locator::Id("randomize"))
            .await
            .unwrap()
            .click()
            .await
            .unwrap();
    }
    let url = client.current_url().await.unwrap().to_string();
    client.close_window().await.unwrap();
    client.close().await.unwrap();
    url
}

#[tokio::main]
async fn main() -> Result<()> {
    const IMGX: u32 = 1000;
    const IMGY: u32 = 1000;
    const BLOCK_X: u32 = 60;
    const BLOCK_Y: u32 = 60;
    //run geckodirver first on port 4444
    opener::open("./geckodriver.exe").unwrap();
    let colors: Vec<[u8; 4]> = get_color()
        .await
        .split('=')
        .nth(1)
        .unwrap()
        .split('&')
        .next()
        .unwrap()
        .split('-')
        .map(|str| {
            [
                u8::from_str_radix(&str[0..=1], 16).unwrap(),
                u8::from_str_radix(&str[2..=3], 16).unwrap(),
                u8::from_str_radix(&str[4..=5], 16).unwrap(),
                255,
            ]
        })
        .collect();

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
                10..=64 => image::Rgba([colors[2][0], colors[2][1], colors[2][2], a]),
                65..=92 => image::Rgba([colors[3][0], colors[3][1], colors[3][2], a]),
                _ => image::Rgba([colors[4][0], colors[4][1], colors[4][2], a]),
            };
            'X: for i in x..BLOCK_X + x {
                for j in y..BLOCK_Y + y {
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

    //let mut bound: std::ops::RangeInclusive<u32> = IMGX / 2..=IMGX / 2;
    //dimond shape
    // for j in 0..IMGY/2{
    //     for i in 0..IMGX {
    //         if !bound.contains(&i){
    //             let pixel = imgbuf.get_pixel_mut(i, j);
    //             *pixel = Rgba([0,0,0,0]);
    //         }
    //     }
    //     bound = bound.start()-1..=bound.end()+1;
    // }
    // for j in IMGY/2..IMGY{
    //     for i in 0..IMGX {
    //         if !bound.contains(&i){
    //             let pixel = imgbuf.get_pixel_mut(i, j);
    //             *pixel = Rgba([0,0,0,0]);
    //         }
    //     }
    //     bound = bound.start()+1..=bound.end()-1;
    // }
    let center_x = IMGX / 2;
    let center_y = IMGY / 2;
    let radius = IMGY / 2; 

    for y in 0..IMGY {
        for x in 0..IMGX {
            let dx = x as i32 - center_x as i32;
            let dy = y as i32 - center_y as i32;
            let dist_squared = dx * dx + dy * dy;

            if dist_squared > radius as i32 * radius as i32 {
                if let Some(pixel) = imgbuf.get_pixel_mut_checked(x, y) {
                    *pixel = Rgba([0, 0, 0, 0]);
                }
            }
        }
    }

    imgbuf.save("profile.png").unwrap();
    opener::open(std::path::Path::new("./profile.png")).unwrap();
    Ok(())
}
