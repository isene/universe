//! The photographs, and how they are laid into a scene.
//!
//! Eight rungs are things nobody can draw better than a camera did:
//! the Earth, Jupiter, the Sun, the Moon, a city at night, a coastline,
//! a mountain range, and the shape of our own galaxy. Those are carried
//! in the binary and painted pixel by pixel into the same canvas as
//! everything else, so a scene is still one picture.
//!
//! Every one is public domain, from NASA. The Moon is the same height
//! map the `moon` app uses.
//!
//! A picture is decoded once and kept, so stepping back to a rung costs
//! nothing the second time.

use crate::paint::{mix, shade, Rgb};
use glow::Canvas;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// Which picture. The order is the order they appear on the ladder.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pic {
    Ant,
    Hand,
    Body,
    Whale,
    Everest,
    City,
    Coast,
    Earth,
    Jupiter,
    Sun,
    Galaxy,
}

const ANT: &[u8] = include_bytes!("../img/ant.jpg");
const HAND: &[u8] = include_bytes!("../img/hand.jpg");
const BODY: &[u8] = include_bytes!("../img/body.jpg");
const WHALE: &[u8] = include_bytes!("../img/whale.jpg");
const EVEREST: &[u8] = include_bytes!("../img/everest.jpg");
const CITY: &[u8] = include_bytes!("../img/city.jpg");
const COAST: &[u8] = include_bytes!("../img/coast.jpg");
const EARTH: &[u8] = include_bytes!("../img/earth.jpg");
const JUPITER: &[u8] = include_bytes!("../img/jupiter.jpg");
const SUN: &[u8] = include_bytes!("../img/sun.jpg");
const GALAXY: &[u8] = include_bytes!("../img/galaxy.jpg");

/// The Moon's near side as a height map, one byte per point, the same
/// file the `moon` app draws from.
const MOON_MAP: &[u8] = include_bytes!("../img/moon-512.gray");
const MOON_N: usize = 512;

pub struct Image {
    pub w: usize,
    pub h: usize,
    pub rgb: Vec<u8>,
}

impl Image {
    /// The colour at a point, with the edges held rather than wrapped.
    pub fn at(&self, x: usize, y: usize) -> Rgb {
        let x = x.min(self.w - 1);
        let y = y.min(self.h - 1);
        let o = (y * self.w + x) * 3;
        (self.rgb[o], self.rgb[o + 1], self.rgb[o + 2])
    }
}

fn store() -> &'static Mutex<HashMap<usize, &'static Image>> {
    static S: OnceLock<Mutex<HashMap<usize, &'static Image>>> = OnceLock::new();
    S.get_or_init(|| Mutex::new(HashMap::new()))
}

fn bytes(p: Pic) -> &'static [u8] {
    match p {
        Pic::Ant => ANT,
        Pic::Hand => HAND,
        Pic::Body => BODY,
        Pic::Whale => WHALE,
        Pic::Everest => EVEREST,
        Pic::City => CITY,
        Pic::Coast => COAST,
        Pic::Earth => EARTH,
        Pic::Jupiter => JUPITER,
        Pic::Sun => SUN,
        Pic::Galaxy => GALAXY,
    }
}

/// The picture, decoded on first use and kept for the rest of the run.
pub fn get(p: Pic) -> &'static Image {
    let key = p as usize;
    let mut s = store().lock().unwrap_or_else(|e| e.into_inner());
    if let Some(img) = s.get(&key) {
        return img;
    }
    let img: &'static Image = Box::leak(Box::new(decode(bytes(p))));
    s.insert(key, img);
    img
}

fn decode(data: &[u8]) -> Image {
    match image::load_from_memory_with_format(data, image::ImageFormat::Jpeg) {
        Ok(d) => {
            let rgb = d.to_rgb8();
            Image { w: rgb.width() as usize, h: rgb.height() as usize, rgb: rgb.into_raw() }
        }
        // A picture that will not decode should not take the app down:
        // the scene simply comes out dark.
        Err(_) => Image { w: 1, h: 1, rgb: vec![0, 0, 0] },
    }
}

/// Lay the picture over the whole canvas, cropping whichever way keeps
/// its shape. `dim` darkens it, for a scene that wants text over it.
pub fn cover(c: &mut Canvas, p: Pic, dim: f64) {
    let img = get(p);
    let (cw, ch) = (c.w as f64, c.h as f64);
    let scale = (cw / img.w as f64).max(ch / img.h as f64);
    let (ox, oy) = ((img.w as f64 * scale - cw) / 2.0, (img.h as f64 * scale - ch) / 2.0);
    for y in 0..c.h {
        for x in 0..c.w {
            let sx = ((x as f64 + ox) / scale) as usize;
            let sy = ((y as f64 + oy) / scale) as usize;
            c.put(x, y, shade(img.at(sx, sy), dim));
        }
    }
}

/// Lay the whole picture inside the canvas, as large as it goes, and
/// leave what is already there around it. Anything near black in the
/// picture is let through, so a drawing on black sits on the scene's
/// own background.
pub fn inset(c: &mut Canvas, p: Pic, fill: f64) {
    let img = get(p);
    let (cw, ch) = (c.w as f64, c.h as f64);
    let scale = (cw / img.w as f64).min(ch / img.h as f64) * fill;
    let (dw, dh) = (img.w as f64 * scale, img.h as f64 * scale);
    let (ox, oy) = ((cw - dw) / 2.0, (ch - dh) / 2.0);
    for y in 0..(dh as usize) {
        for x in 0..(dw as usize) {
            let rgb = img.at((x as f64 / scale) as usize, (y as f64 / scale) as usize);
            let lit = (rgb.0 as u32 + rgb.1 as u32 + rgb.2 as u32) as f64 / 765.0;
            if lit < 0.02 {
                continue;
            }
            let a = (lit / 0.12).min(1.0);
            c.blend((ox + x as f64) as i64, (oy + y as f64) as i64, rgb, a);
        }
    }
}

/// Lay a round thing, a planet or a star, into the middle of the canvas
/// at the given radius. The picture is a disc on black, so anything
/// outside the disc is left as it was and the sky shows through.
pub fn globe(c: &mut Canvas, p: Pic, cx: f64, cy: f64, r: f64) {
    let img = get(p);
    // The source disc is centred and nearly fills its frame.
    let src_r = img.w.min(img.h) as f64 / 2.0 * 0.995;
    let (icx, icy) = (img.w as f64 / 2.0, img.h as f64 / 2.0);
    let x0 = (cx - r - 1.0).max(0.0) as usize;
    let x1 = (cx + r + 1.0).min(c.w as f64) as usize;
    let y0 = (cy - r - 1.0).max(0.0) as usize;
    let y1 = (cy + r + 1.0).min(c.h as f64) as usize;
    for y in y0..y1 {
        for x in x0..x1 {
            let (dx, dy) = ((x as f64 + 0.5 - cx) / r, (y as f64 + 0.5 - cy) / r);
            let d = (dx * dx + dy * dy).sqrt();
            if d > 1.0 {
                continue;
            }
            let sx = (icx + dx * src_r) as usize;
            let sy = (icy + dy * src_r) as usize;
            let rgb = img.at(sx, sy);
            // The rim is feathered, so the disc does not end in a stair.
            let a = ((1.0 - d) * r).clamp(0.0, 1.0);
            c.blend(x as i64, y as i64, rgb, a);
        }
    }
}

/// The Moon, lit from one side, with its near side mapped onto the ball
/// from the height map. The terminator is where the light runs out, so
/// the craters along it throw the shadows that make it read as a world.
pub fn moon_globe(c: &mut Canvas, cx: f64, cy: f64, r: f64, light: (f64, f64)) {
    let (lx, ly) = light;
    let n = (lx * lx + ly * ly + 1.0).sqrt();
    let (lx, ly, lz) = (lx / n, ly / n, 1.0 / n);
    let x0 = (cx - r - 1.0).max(0.0) as usize;
    let x1 = (cx + r + 1.0).min(c.w as f64) as usize;
    let y0 = (cy - r - 1.0).max(0.0) as usize;
    let y1 = (cy + r + 1.0).min(c.h as f64) as usize;
    for y in y0..y1 {
        for x in x0..x1 {
            let (dx, dy) = ((x as f64 + 0.5 - cx) / r, (y as f64 + 0.5 - cy) / r);
            let d2 = dx * dx + dy * dy;
            if d2 > 1.0 {
                continue;
            }
            let dz = (1.0 - d2).sqrt();
            // Straight down the middle of the near side: the map is a
            // picture of the disc, so the same coordinates serve.
            let sx = ((dx * 0.5 + 0.5) * (MOON_N - 1) as f64) as usize;
            let sy = ((dy * 0.5 + 0.5) * (MOON_N - 1) as f64) as usize;
            let g = MOON_MAP[sy.min(MOON_N - 1) * MOON_N + sx.min(MOON_N - 1)] as f64 / 255.0;
            let lam = (dx * lx + dy * ly + dz * lz).max(0.0);
            let f = 0.06 + 1.05 * lam;
            let grey = mix((58, 56, 58), (226, 223, 216), g);
            let a = ((1.0 - d2.sqrt()) * r).clamp(0.0, 1.0);
            c.blend(x as i64, y as i64, shade(grey, f), a);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_picture_decodes() {
        for p in [Pic::Ant, Pic::Hand, Pic::Body, Pic::Whale, Pic::Everest, Pic::City, Pic::Coast, Pic::Earth, Pic::Jupiter, Pic::Sun, Pic::Galaxy] {
            let img = get(p);
            assert!(img.w > 200 && img.h > 200, "a picture came back empty");
        }
    }

    #[test]
    fn a_picture_decodes_once_and_is_kept() {
        let a = get(Pic::Earth) as *const Image;
        let b = get(Pic::Earth) as *const Image;
        assert_eq!(a, b, "the second call gives back the same picture");
    }

    #[test]
    fn a_globe_fills_its_circle_and_leaves_the_sky_alone() {
        let mut c = Canvas::with_cell(40, 20, (10, 20));
        globe(&mut c, Pic::Earth, 200.0, 200.0, 120.0);
        let at = |x: usize, y: usize| c.rgba[(y * c.w + x) * 4 + 2] as u32;
        assert!(at(200, 200) > 20, "the middle of the globe is painted");
        assert_eq!(at(5, 5), 0, "the corner is untouched");
    }

    #[test]
    fn the_moon_is_lit_on_one_side() {
        let mut c = Canvas::with_cell(40, 20, (10, 20));
        moon_globe(&mut c, 200.0, 200.0, 120.0, (-0.6, -0.5));
        let at = |x: usize, y: usize| c.rgba[(y * c.w + x) * 4] as u32;
        assert!(at(150, 150) > at(250, 250), "the lit side is the brighter one");
    }
}
