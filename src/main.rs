#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
extern crate image;

use image::Rgba;
use image::GenericImage;

type C = Rgba<u8>;
type P = (u32, u32);
type Img = image::DynamicImage;

fn is_basically_white(r: u8, g: u8, b: u8) -> bool {
    r >= 240 && g >= 240 && b >= 240
}

fn is_light_green(p: &C) -> bool {
    p[0] >= 160 && p[1] >= 208 && p[2] >= 160 && p[0] < 208 && p[2] < 208
}

fn is_circle_edge(p: C) -> bool {
    let (r, g, b) = (p[0], p[1], p[2]);
    r <= 16 && g <= 16 && b <= 16
}

fn is_circle_body(p: C) -> bool {
    is_basically_white(p[0], p[1], p[2])
}


struct CircleFound {
    l: P,
    r: P,
    u: P,
    d: P,
    // todo: add color of circle: red, blue, white
}

fn get_pixel(img: &Img, x: u32, y: u32) -> C {
    if y >= img.height() || x >= img.width() {
        return C { data: [0, 0, 0, 0] };
    }
    img.get_pixel(x, y)
}

fn check_if_edge(img: &Img, ps: [P; 15]) -> Option<P> {
    // vector should start from the center of the circle to the outside
    let mut stage = 0;
    for p in ps.iter() {
        let px = get_pixel(img, p.0, p.1);
        let body = is_circle_body(px);
        if stage == 0 {
            if !body {
                return None
            }
            stage = 1
        } else if stage == 1 {
            if !body {
                stage = 2
            }
        }
        if stage == 2 {
            return Some(p.clone())
        }
    }
    None
}

fn find_circle(img: &Img, p: P) -> Option<CircleFound> {
    if !is_circle_edge(get_pixel(img, p.0, p.1)) {
        return None
    }
    let (x_center, y_up) = p;
    let y_down = {
        let mut stage = 0;
        let mut y_result = y_up;
        for y in y_up..img.height() {
            let px = get_pixel(img, x_center, y);
            let edge = is_circle_edge(px);
            let body = is_circle_body(px);
            if stage == 0 {
                if !edge {
                    stage = 1
                }
            } else if stage == 1 {
                if body {
                    stage = 2
                }
            } else if stage == 2 {
                if !body {
                    stage = 3
                }
            }
            if stage == 3 {
                y_result = y;
                break
            }
        }
        y_result
    };

    let y_center = (y_up + y_down) / 2;
    let radius = y_center - y_up;
    let y_left_res = check_if_edge(img, [
        (x_center - radius + 7, y_center),
        (x_center - radius + 6, y_center),
        (x_center - radius + 5, y_center),
        (x_center - radius + 4, y_center),
        (x_center - radius + 3, y_center),
        (x_center - radius + 2, y_center),
        (x_center - radius + 1, y_center),
        (x_center - radius + 0, y_center),
        (x_center - radius - 1, y_center),
        (x_center - radius - 2, y_center),
        (x_center - radius - 3, y_center),
        (x_center - radius - 4, y_center),
        (x_center - radius - 5, y_center),
        (x_center - radius - 6, y_center),
        (x_center - radius - 7, y_center)
    ]);
    match y_left_res {
        None => None,
        Some((x_left, _)) => Some(CircleFound {
            l: (x_left, y_center),
            r: (x_left + 2 * radius, y_center),
            u: (x_center, y_up),
            d: (x_center, y_down)
        })
    }
}

fn main() {
    let img = image::open("resources/example.png").unwrap();

    let x_white = 1050..1150;
    let y_white = 250..350;

    let x_lg = 875..900;
    let y_lg = 400..450;

    let x_full = 0..img.width();
    let y_full = 0..img.height();

    for x in x_full.clone() {
        for y in y_full.clone() {
            if let Some(CircleFound{l, r, u, d}) = find_circle(&img, (x, y)) {
                println!("Circle found: {:?}, {:?}, {:?}, {:?}", l, r, u, d)
            }
        }
    }
}
