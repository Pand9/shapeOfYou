#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
extern crate image;

use image::GenericImage;
use image::Rgba;

type C = Rgba<u8>;
type P = (i32, i32);
type Img = image::DynamicImage;

fn width(img: &Img) -> i32 {
    img.width() as i32
}

fn height(img: &Img) -> i32 {
    img.height() as i32
}

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

fn big(v: u8) -> bool {
    v >= 208
}

fn small(v: u8) -> bool {
    v <= 48
}

fn mild(v: u8) -> bool {
    v >= 64 && v <= 192
}

fn is_kinda_red(p: C) -> bool {
    let [r, g, b, _] = p.data;
    big(r) && mild(g) && mild(b)
}

fn is_circle_body(p: C) -> bool {
    is_basically_white(p[0], p[1], p[2]) || is_kinda_red(p)
}


struct CircleFound {
    l: i32,
    r: i32,
    u: i32,
    d: i32,
    // todo: add color of circle: red, blue, white
}

fn get_pixel(img: &Img, x: i32, y: i32) -> C {
    if y >= height(img) || x >= width(img) {
        return C { data: [0, 0, 0, 0] };
    }
    img.get_pixel(x as u32, y as u32)
}

fn check_if_edge<Iter: Iterator<Item = P>>(img: &Img, ps: Iter) -> Option<P> {
    // vector should start from the center of the circle to the outside
    let mut stage = 0;
    for p in ps {
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

fn check_circle(img: &Img, c: &CircleFound) -> i32 {
    // the least score the better
    if !is_circle_body(get_pixel(img, (c.l + c.r) / 2, (c.u + c.d) / 2)) {
        1000
    } else {
        (c.u - c.d) - (c.l - c.r)
    }
}

fn find_circle(img: &Img, p: P) -> Option<(CircleFound, i32)> {
    if !is_circle_edge(get_pixel(img, p.0, p.1)) {
        return None
    }
    let (x_center, y_up) = p;
    let (x_center, y_up) = (x_center as i32, y_up as i32);
    let y_down = {
        let mut stage = 0;
        let mut y_result = y_up;
        for y in y_up..(img.height() as i32) {
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

    let x_left = match check_if_edge(img, (-7..8).map(|e| (x_center - radius - e, y_center))) {
        None => return None,
        Some((x, _)) => x
    };

    let x_right = match check_if_edge(img, (-7..8).map(|e| (x_center + radius + e, y_center))) {
        None => return None,
        Some((x, _)) => x
    };

    let res = CircleFound {
        l: x_left,
        r: x_right,
        u: y_up,
        d: y_down,
    };
    let score = check_circle(img, & res);

    Some((res, score))
}

fn main() {
    let img = image::open("resources/example.png").unwrap();

    let x_white = 1050..1150;
    let y_white = 250..350;

    let x_lg = 875..900;
    let y_lg = 400..450;

    let x_full = 0..width(&img);
    let y_full = 0..height(&img);

    for x in x_full.clone() {
        for y in y_full.clone() {
            if let Some((CircleFound{l, r, u, d}, score)) = find_circle(&img, (x, y)) {
                println!("Circle found: {:?}, {:?}, {:?}, {:?}, score: {}", l, r, u, d, score)
            }
        }
    }
}
