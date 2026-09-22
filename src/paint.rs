//! The drawing kit: the few shapes every scene is built from.
//!
//! Every picture is painted into one `glow::Canvas`, whatever the
//! terminal can show. A terminal with real pixels gets the canvas as it
//! is; one without gets the same pixels as half-blocks, two rows of
//! colour per character cell. So a scene is written once.
//!
//! Nothing here moves by itself. A scene is painted when the step
//! changes or the window is resized, never on a timer.

use glow::Canvas;

pub type Rgb = (u8, u8, u8);

/// Numbers that look random and never change between two runs, so a
/// starfield stays put when the window is resized.
pub struct Dice(u64);

impl Dice {
    pub fn new(seed: u64) -> Dice {
        Dice(seed.wrapping_mul(0x9E3779B97F4A7C15) | 1)
    }
    /// The next number, from 0 up to but not including 1.
    pub fn next(&mut self) -> f64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        (x >> 11) as f64 / (1u64 << 53) as f64
    }
    /// The next number between `lo` and `hi`.
    pub fn span(&mut self, lo: f64, hi: f64) -> f64 {
        lo + self.next() * (hi - lo)
    }
    /// True with the given chance.
    pub fn odds(&mut self, p: f64) -> bool {
        self.next() < p
    }
}

/// Paint the whole canvas one colour.
pub fn fill(c: &mut Canvas, rgb: Rgb) {
    for y in 0..c.h {
        for x in 0..c.w {
            c.put(x, y, rgb);
        }
    }
}

/// A top-to-bottom wash, for a sky or a deep field.
pub fn wash(c: &mut Canvas, top: Rgb, bottom: Rgb) {
    let h = c.h.max(1) as f64;
    for y in 0..c.h {
        let t = y as f64 / h;
        let rgb = mix(top, bottom, t);
        for x in 0..c.w {
            c.put(x, y, rgb);
        }
    }
}

/// Two colours mixed, `t` from 0 (all `a`) to 1 (all `b`).
pub fn mix(a: Rgb, b: Rgb, t: f64) -> Rgb {
    let t = t.clamp(0.0, 1.0);
    (
        (a.0 as f64 + (b.0 as f64 - a.0 as f64) * t) as u8,
        (a.1 as f64 + (b.1 as f64 - a.1 as f64) * t) as u8,
        (a.2 as f64 + (b.2 as f64 - a.2 as f64) * t) as u8,
    )
}

/// The same colour, brighter or darker. 1.0 leaves it alone.
pub fn shade(rgb: Rgb, f: f64) -> Rgb {
    let g = |v: u8| (v as f64 * f).clamp(0.0, 255.0) as u8;
    (g(rgb.0), g(rgb.1), g(rgb.2))
}

/// A lit ball: bright where the light falls, dark on the far side, with
/// a soft edge so it does not look cut out. The light comes from the
/// upper left unless `light` says otherwise.
pub fn ball(c: &mut Canvas, cx: f64, cy: f64, r: f64, rgb: Rgb, light: (f64, f64)) {
    if r <= 0.0 {
        return;
    }
    let (lx, ly) = light;
    let n = (lx * lx + ly * ly + 1.0).sqrt();
    let (lx, ly, lz) = (lx / n, ly / n, 1.0 / n);
    let x0 = (cx - r - 1.0).floor().max(0.0) as usize;
    let x1 = (cx + r + 1.0).ceil().min(c.w as f64) as usize;
    let y0 = (cy - r - 1.0).floor().max(0.0) as usize;
    let y1 = (cy + r + 1.0).ceil().min(c.h as f64) as usize;
    for y in y0..y1 {
        for x in x0..x1 {
            let (dx, dy) = ((x as f64 + 0.5 - cx) / r, (y as f64 + 0.5 - cy) / r);
            let d2 = dx * dx + dy * dy;
            if d2 > 1.0 {
                continue;
            }
            let dz = (1.0 - d2).sqrt();
            // Lambert, lifted off the floor so the night side is not
            // pure black, plus a rim so the sphere reads as round.
            let lam = (dx * lx + dy * ly + dz * lz).max(0.0);
            let rim = (1.0 - dz).powf(3.0) * 0.25;
            let f = 0.12 + 0.95 * lam + rim;
            // The last pixel ring fades out, so the edge is not a stair.
            let edge = ((1.0 - d2.sqrt()) * r).clamp(0.0, 1.0);
            c.blend(x as i64, y as i64, shade(rgb, f), edge);
        }
    }
}

/// A soft glow around a point, falling off with distance. This is what
/// makes a star look like a star rather than a dot.
pub fn halo(c: &mut Canvas, cx: f64, cy: f64, r: f64, rgb: Rgb, strength: f64) {
    if r <= 0.0 {
        return;
    }
    let x0 = (cx - r).floor().max(0.0) as usize;
    let x1 = (cx + r).ceil().min(c.w as f64) as usize;
    let y0 = (cy - r).floor().max(0.0) as usize;
    let y1 = (cy + r).ceil().min(c.h as f64) as usize;
    for y in y0..y1 {
        for x in x0..x1 {
            let (dx, dy) = (x as f64 + 0.5 - cx, y as f64 + 0.5 - cy);
            let d = (dx * dx + dy * dy).sqrt() / r;
            if d >= 1.0 {
                continue;
            }
            let a = (1.0 - d).powf(2.4) * strength;
            c.blend(x as i64, y as i64, rgb, a);
        }
    }
}

/// A ring of given thickness, for an orbit or a shell.
pub fn orbit(c: &mut Canvas, cx: f64, cy: f64, rx: f64, ry: f64, rgb: Rgb, alpha: f64) {
    let steps = ((rx.max(ry)) * 7.0).clamp(64.0, 2000.0) as usize;
    let mut prev: Option<(f64, f64)> = None;
    for i in 0..=steps {
        let t = i as f64 / steps as f64 * std::f64::consts::TAU;
        let p = (cx + rx * t.cos(), cy + ry * t.sin());
        if let Some(q) = prev {
            c.line(q, p, 1.0, rgb, alpha);
        }
        prev = Some(p);
    }
}

/// Points scattered over the whole canvas, dimmer ones more common, the
/// way a sky looks. `warm` tints a few of them.
pub fn starfield(c: &mut Canvas, seed: u64, count: usize) {
    let mut d = Dice::new(seed);
    for _ in 0..count {
        let x = d.next() * c.w as f64;
        let y = d.next() * c.h as f64;
        let b = d.next().powf(2.6);
        let rgb = if d.odds(0.12) {
            mix((255, 210, 170), (255, 255, 255), d.next())
        } else if d.odds(0.1) {
            mix((180, 205, 255), (255, 255, 255), d.next())
        } else {
            (255, 255, 255)
        };
        let r = 0.5 + b * 1.6;
        halo(c, x, y, r * 2.4, rgb, b * 0.5);
        c.disc(x, y, r * 0.5, rgb, 0.35 + b * 0.65);
    }
}

/// A grainy cloud around a centre, for gas, dust or a nebula.
pub fn cloud(c: &mut Canvas, seed: u64, cx: f64, cy: f64, rx: f64, ry: f64, rgb: Rgb, puffs: usize) {
    let mut d = Dice::new(seed);
    for _ in 0..puffs {
        // Cluster towards the middle, so the edge frays instead of
        // stopping at a line.
        let t = d.next() * std::f64::consts::TAU;
        let k = d.next().powf(0.55);
        let x = cx + t.cos() * rx * k;
        let y = cy + t.sin() * ry * k;
        let r = d.span(0.02, 0.16) * rx.max(ry);
        let a = (1.0 - k) * d.span(0.05, 0.22);
        halo(c, x, y, r, rgb, a);
    }
}

/// A straight run of small dots, for a filament of galaxies.
pub fn strand(c: &mut Canvas, d: &mut Dice, a: (f64, f64), b: (f64, f64), n: usize, rgb: Rgb, spread: f64) {
    for _ in 0..n {
        let t = d.next();
        let x = a.0 + (b.0 - a.0) * t + d.span(-spread, spread);
        let y = a.1 + (b.1 - a.1) * t + d.span(-spread, spread);
        let w = d.span(0.4, 1.5);
        let br = d.span(0.3, 1.0);
        halo(c, x, y, w * 3.0, rgb, br * 0.35);
        c.disc(x, y, w * 0.6, mix(rgb, (255, 255, 255), 0.4), br);
    }
}

/// Render the canvas as half-blocks: each cell shows two pixels, the top
/// as the letter's colour and the bottom as its background. For a
/// terminal with no pixel support.
pub fn half_blocks(c: &Canvas, cols: u16, rows: u16) -> Vec<String> {
    use crust::style;
    let mut out = Vec::with_capacity(rows as usize);
    for row in 0..rows as usize {
        let mut line = String::new();
        let mut last: Option<(Rgb, Rgb)> = None;
        for col in 0..cols as usize {
            let top = sample(c, col, row * 2, cols, rows);
            let bot = sample(c, col, row * 2 + 1, cols, rows);
            if last != Some((top, bot)) {
                line.push_str(&style::set_fg_rgb(top.0, top.1, top.2));
                line.push_str(&style::set_bg_rgb(bot.0, bot.1, bot.2));
                last = Some((top, bot));
            }
            line.push('\u{2580}');
        }
        line.push_str(style::RESET);
        out.push(line);
    }
    out
}

/// The colour of one half-block, averaged over the pixels it covers.
fn sample(c: &Canvas, col: usize, half: usize, cols: u16, rows: u16) -> Rgb {
    let x0 = col * c.w / cols.max(1) as usize;
    let x1 = ((col + 1) * c.w / cols.max(1) as usize).max(x0 + 1).min(c.w);
    let y0 = half * c.h / (rows.max(1) as usize * 2);
    let y1 = ((half + 1) * c.h / (rows.max(1) as usize * 2)).max(y0 + 1).min(c.h);
    let (mut r, mut g, mut b, mut n) = (0u32, 0u32, 0u32, 0u32);
    for y in y0..y1 {
        for x in x0..x1 {
            let o = (y * c.w + x) * 4;
            if o + 2 < c.rgba.len() {
                r += c.rgba[o] as u32;
                g += c.rgba[o + 1] as u32;
                b += c.rgba[o + 2] as u32;
                n += 1;
            }
        }
    }
    if n == 0 {
        return (0, 0, 0);
    }
    ((r / n) as u8, (g / n) as u8, (b / n) as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_dice_are_the_same_every_run() {
        let a: Vec<f64> = (0..5).map(|_| Dice::new(7).next()).collect();
        assert!(a.iter().all(|v| (*v - a[0]).abs() < 1e-12), "same seed, same first number");
        let mut d = Dice::new(7);
        let run: Vec<f64> = (0..5).map(|_| d.next()).collect();
        assert!(run.iter().all(|v| *v >= 0.0 && *v < 1.0), "every number lands in range");
        assert!(run[0] != run[1], "and they do not repeat");
    }

    #[test]
    fn a_ball_is_lit_on_the_side_the_light_comes_from() {
        let mut c = Canvas::with_cell(20, 10, (10, 20));
        fill(&mut c, (0, 0, 0));
        ball(&mut c, 100.0, 100.0, 60.0, (200, 200, 200), (-0.6, -0.6));
        let at = |x: usize, y: usize| c.rgba[(y * c.w + x) * 4] as u32;
        assert!(at(70, 70) > at(130, 130), "the lit side is brighter than the far side");
        assert_eq!(at(10, 10), 0, "and the corner is untouched");
    }

    #[test]
    fn half_blocks_carry_the_colour_of_the_pixels() {
        let mut c = Canvas::with_cell(4, 2, (2, 4));
        fill(&mut c, (10, 20, 30));
        let rows = half_blocks(&c, 4, 2);
        assert_eq!(rows.len(), 2);
        assert!(rows[0].contains("10;20;30"), "the colour reaches the escape sequence");
        assert_eq!(rows[0].matches('\u{2580}').count(), 4, "one block per column");
    }
}
