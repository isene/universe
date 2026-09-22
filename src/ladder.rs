//! The rungs of the ladder, smallest first, and the picture for each.
//!
//! A rung says how wide the picture is in metres, not how big the thing
//! is. The thing sits inside that width with room around it, so one step
//! really does look like a step outwards.

use crate::paint::*;
use glow::Canvas;
use std::f64::consts::{PI, TAU};

pub struct Rung {
    pub name: &'static str,
    /// How wide the picture is, in metres.
    pub span: f64,
    pub blurb: &'static str,
}

/// Where the app opens: a person, in the middle of everything there is.
pub const HUMAN: usize = 14;

pub const RUNGS: &[Rung] = &[
    Rung { name: "Quantum foam", span: 1e-34,
        blurb: "The smallest length that means anything. Below this, distance itself stops making sense." },
    Rung { name: "Quark", span: 1e-17,
        blurb: "No experiment has ever found a quark to have any size at all. This is only the room it moves in." },
    Rung { name: "Proton", span: 5e-15,
        blurb: "Three quarks, and the force between them. Almost all the mass is that force, not the quarks." },
    Rung { name: "Carbon nucleus", span: 2e-14,
        blurb: "Six protons and six neutrons, packed as tight as matter goes. Every atom of you is built around one of these." },
    Rung { name: "Carbon atom", span: 3e-10,
        blurb: "The nucleus is the dot in the middle. The rest is electrons, and the rest is almost all of it." },
    Rung { name: "Water molecule", span: 1e-9,
        blurb: "One oxygen, two hydrogens, at an angle of 104.5 degrees. That angle is why water behaves as it does." },
    Rung { name: "DNA", span: 1e-8,
        blurb: "Two metres of this is coiled inside nearly every cell you have." },
    Rung { name: "Virus", span: 3e-7,
        blurb: "A shell of protein around a set of instructions. Too small to be alive on its own." },
    Rung { name: "Bacterium", span: 6e-6,
        blurb: "One cell, no nucleus, and more of these live on you than you have cells of your own." },
    Rung { name: "Red blood cell", span: 3e-5,
        blurb: "Dished on both sides to carry more oxygen. You make about two million of them every second." },
    Rung { name: "Human hair", span: 2e-4,
        blurb: "Thin enough to be a byword for thin, and still ten times a red blood cell." },
    Rung { name: "Grain of sand", span: 2e-3,
        blurb: "The smallest thing you can pick up and look at." },
    Rung { name: "Ant", span: 1.5e-2,
        blurb: "The smallest thing that walks about with a plan." },
    Rung { name: "Human hand", span: 0.5,
        blurb: "The part of you that made everything else on this ladder." },
    Rung { name: "Human body", span: 2.5,
        blurb: "You are here, almost exactly halfway between the smallest thing and the largest." },
    Rung { name: "Blue whale", span: 60.0,
        blurb: "The largest animal that has ever lived, with a person beside it." },
    Rung { name: "Football pitch", span: 250.0,
        blurb: "The size of a thing you can see all of, standing still." },
    Rung { name: "Skyscraper", span: 1.2e3,
        blurb: "The tallest things people build, still less than a thousandth of the air above them." },
    Rung { name: "Mount Everest", span: 2.4e4,
        blurb: "The highest ground on Earth, and a smaller bump than it feels from below." },
    Rung { name: "City at night", span: 6e4,
        blurb: "Millions of lives, and from here only the lights." },
    Rung { name: "Coastline from orbit", span: 2e6,
        blurb: "A country, seen the way weather sees it." },
    Rung { name: "The Moon", span: 8e6,
        blurb: "The only other ground anyone has stood on." },
    Rung { name: "The Earth", span: 3e7,
        blurb: "Everything anyone has ever done, apart from a few days, happened here." },
    Rung { name: "Jupiter", span: 4e8,
        blurb: "Eleven Earths across, and made of almost nothing but the two lightest gases." },
    Rung { name: "The Sun", span: 4e9,
        blurb: "A million Earths would fit inside. The dot is Earth, to scale." },
    Rung { name: "Earth's orbit", span: 6e11,
        blurb: "Light takes eight minutes to cross from the middle to here." },
    Rung { name: "The solar system", span: 2e13,
        blurb: "Out to Neptune. Everything you have ever seen with your own eyes sits inside this circle, bar the stars." },
    Rung { name: "The Oort cloud", span: 4e16,
        blurb: "A shell of ice around the Sun, a thousand times further out than the planets. The comets come from here." },
    Rung { name: "The nearest stars", span: 1.2e17,
        blurb: "The Sun is one of these. The next one along is four light years away, and that is close." },
    Rung { name: "The Orion Nebula", span: 5e17,
        blurb: "Stars being made, right now, and near enough to see with your eyes on a dark night." },
    Rung { name: "A globular cluster", span: 3e18,
        blurb: "A million old stars held together since the galaxy was young." },
    Rung { name: "The Milky Way", span: 3e21,
        blurb: "A few hundred billion stars. Every star you have ever seen by eye is one of them, and all of them are nearby." },
    Rung { name: "The Local Group", span: 3e23,
        blurb: "Our galaxy, Andromeda, and about eighty smaller ones, falling towards each other." },
    Rung { name: "The Virgo Cluster", span: 1e24,
        blurb: "A thousand galaxies, and the nearest place where gravity beats the expansion of space." },
    Rung { name: "Laniakea", span: 1e25,
        blurb: "A hundred thousand galaxies, all drifting the same way. The name means immeasurable heaven." },
    Rung { name: "The cosmic web", span: 1e26,
        blurb: "Galaxies string along filaments, and between them the voids are almost perfectly empty." },
    Rung { name: "The observable universe", span: 8.8e26,
        blurb: "As far as light has had time to reach us. The edge is the glow left over from the beginning." },
];

/// Paint the rung into the canvas. Coordinates are pixels, so every
/// scene works out its own sizes from the canvas it is given.
pub fn paint(i: usize, c: &mut Canvas) {
    let (w, h) = (c.w as f64, c.h as f64);
    let (cx, cy) = (w / 2.0, h / 2.0);
    // Two units. `u` is what a round thing can use and still fit top to
    // bottom. `flat` is for a scene squashed towards the horizontal, a
    // galaxy or a whale, which can take the width instead.
    let u = (h / 2.0).min(w / 2.0) * 0.94;
    let flat = (w / 2.0 * 0.94).min(h / 2.0 / 0.46);
    match i {
        0 => foam(c, w, h),
        1 => quark(c, cx, cy, u),
        2 => proton(c, cx, cy, u),
        3 => nucleus(c, cx, cy, u),
        4 => atom(c, cx, cy, u),
        5 => water(c, cx, cy, u),
        6 => dna(c, cx, cy, u, h),
        7 => virus(c, cx, cy, u),
        8 => bacterium(c, cx, cy, flat),
        9 => blood(c, cx, cy, u),
        10 => hair(c, cx, cy, u, h),
        11 => sand(c, cx, cy, u),
        12 => ant(c, cx, cy, flat),
        13 => hand(c, cx, cy, u),
        14 => body(c, cx, cy, u),
        15 => whale(c, cx, cy, flat, w),
        16 => pitch(c, cx, cy, flat),
        17 => tower(c, cx, cy, u, h),
        18 => everest(c, cx, cy, u, w, h),
        19 => city(c, w, h),
        20 => coast(c, w, h),
        21 => moon(c, cx, cy, u),
        22 => earth(c, cx, cy, u),
        23 => jupiter(c, cx, cy, u),
        24 => sun(c, cx, cy, u),
        25 => inner_system(c, cx, cy, flat),
        26 => solar_system(c, cx, cy, flat),
        27 => oort(c, cx, cy, u),
        28 => near_stars(c, w, h),
        29 => orion(c, w, h),
        30 => cluster(c, cx, cy, u),
        31 => galaxy(c, cx, cy, flat),
        32 => local_group(c, w, h),
        33 => virgo(c, w, h),
        34 => laniakea(c, w, h),
        35 => web(c, w, h),
        36 => whole(c, cx, cy, u),
        _ => fill(c, (0, 0, 0)),
    }
}

// ── the very small ──────────────────────────────────────────────────

fn foam(c: &mut Canvas, w: f64, h: f64) {
    fill(c, (6, 4, 14));
    let mut d = Dice::new(11);
    // Loops and kinks of space, forming and going again.
    for _ in 0..260 {
        let x = d.next() * w;
        let y = d.next() * h;
        let r = d.span(0.01, 0.05) * w;
        let hue = d.next();
        let rgb = mix((120, 80, 220), (60, 190, 210), hue);
        halo(c, x, y, r * 2.2, rgb, 0.10);
        c.ring(x, y, r, rgb, d.span(0.25, 0.7));
    }
    for _ in 0..140 {
        let x = d.next() * w;
        let y = d.next() * h;
        let a = d.next() * TAU;
        let l = d.span(0.01, 0.06) * w;
        let p = (x + a.cos() * l, y + a.sin() * l);
        c.line((x, y), p, 1.2, mix((200, 170, 255), (255, 255, 255), d.next()), d.span(0.2, 0.6));
    }
}

fn quark(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    fill(c, (5, 3, 12));
    let r = u * 0.62;
    // Three quarks at the corners, and the force between them. The
    // tubes are the point: they hold far more energy than the quarks.
    let cols = [(255, 90, 90), (90, 200, 255), (120, 255, 150)];
    let mut p = [(0.0, 0.0); 3];
    for k in 0..3 {
        let a = -PI / 2.0 + k as f64 * TAU / 3.0;
        p[k] = (cx + a.cos() * r, cy + a.sin() * r);
    }
    let mut d = Dice::new(5);
    for k in 0..3 {
        let q = p[(k + 1) % 3];
        // A tube that wanders, rather than a ruled line.
        let mut prev = p[k];
        for s in 1..=22 {
            let t = s as f64 / 22.0;
            let x = p[k].0 + (q.0 - p[k].0) * t + d.span(-1.0, 1.0) * u * 0.03;
            let y = p[k].1 + (q.1 - p[k].1) * t + d.span(-1.0, 1.0) * u * 0.03;
            c.line(prev, (x, y), u * 0.03, (255, 200, 120), 0.5);
            c.line(prev, (x, y), u * 0.012, (255, 255, 220), 0.9);
            prev = (x, y);
        }
    }
    for k in 0..3 {
        halo(c, p[k].0, p[k].1, u * 0.34, cols[k], 0.75);
        ball(c, p[k].0, p[k].1, u * 0.1, cols[k], (-0.5, -0.5));
    }
}

fn proton(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    fill(c, (5, 3, 12));
    // The gluon sea first, then the quarks inside it.
    let mut d = Dice::new(17);
    cloud(c, 29, cx, cy, u * 0.8, u * 0.8, (120, 100, 230), 220);
    for _ in 0..500 {
        let a = d.next() * TAU;
        let k = d.next().powf(0.5);
        let x = cx + a.cos() * u * 0.78 * k;
        let y = cy + a.sin() * u * 0.78 * k;
        c.disc(x, y, d.span(0.4, 1.4), (200, 190, 255), d.span(0.1, 0.45));
    }
    let cols = [(255, 90, 90), (90, 200, 255), (120, 255, 150)];
    for k in 0..3 {
        let a = -PI / 2.0 + k as f64 * TAU / 3.0;
        let x = cx + a.cos() * u * 0.3;
        let y = cy + a.sin() * u * 0.3;
        halo(c, x, y, u * 0.22, cols[k], 0.8);
        ball(c, x, y, u * 0.075, cols[k], (-0.5, -0.5));
    }
    c.ring(cx, cy, u * 0.82, (150, 140, 255), 0.35);
}

fn nucleus(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    fill(c, (5, 3, 12));
    // Twelve nucleons, packed. Protons warm, neutrons cool.
    let mut d = Dice::new(23);
    let rn = u * 0.15;
    let mut placed: Vec<(f64, f64)> = Vec::new();
    // Bounded, so a small window can never leave this spinning: after
    // enough tries the spacing gives way rather than the loop.
    for tries in 0..4000 {
        if placed.len() == 12 {
            break;
        }
        let a = d.next() * TAU;
        let k = d.next().powf(0.45);
        let p = (cx + a.cos() * u * 0.6 * k, cy + a.sin() * u * 0.6 * k);
        let want = rn * 1.5 * (1.0 - tries as f64 / 4000.0 * 0.6);
        if placed.iter().all(|q: &(f64, f64)| {
            let (dx, dy) = (p.0 - q.0, p.1 - q.1);
            (dx * dx + dy * dy).sqrt() > want
        }) {
            placed.push(p);
        }
    }
    for (n, p) in placed.iter().enumerate() {
        let rgb = if n % 2 == 0 { (240, 110, 90) } else { (110, 160, 240) };
        halo(c, p.0, p.1, rn * 1.7, rgb, 0.3);
        ball(c, p.0, p.1, rn, rgb, (-0.55, -0.6));
    }
}

fn atom(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    fill(c, (4, 4, 12));
    // The whole point of this picture is the emptiness, so the nucleus
    // is drawn at its true size against the electron cloud: a speck.
    cloud(c, 31, cx, cy, u * 0.9, u * 0.9, (70, 120, 230), 260);
    let mut d = Dice::new(41);
    for _ in 0..1400 {
        let a = d.next() * TAU;
        // Two shells, the way carbon holds its electrons.
        let shell = if d.odds(0.33) { 0.34 } else { 0.82 };
        let k = shell + d.span(-0.08, 0.08);
        let x = cx + a.cos() * u * k;
        let y = cy + a.sin() * u * k * 0.98;
        c.disc(x, y, d.span(0.4, 1.1), (150, 200, 255), d.span(0.15, 0.6));
    }
    halo(c, cx, cy, u * 0.14, (255, 220, 150), 0.7);
    c.disc(cx, cy, u * 0.012, (255, 245, 210), 1.0);
}

fn water(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    fill(c, (6, 8, 16));
    let ro = u * 0.34;
    let rh = u * 0.2;
    let d = u * 0.52;
    // 104.5 degrees, the angle that makes water water.
    let half = 104.5_f64.to_radians() / 2.0;
    let o = (cx, cy - u * 0.08);
    let h1 = (o.0 - half.sin() * d, o.1 + half.cos() * d);
    let h2 = (o.0 + half.sin() * d, o.1 + half.cos() * d);
    c.line(o, h1, u * 0.09, (120, 140, 170), 0.8);
    c.line(o, h2, u * 0.09, (120, 140, 170), 0.8);
    halo(c, o.0, o.1, ro * 2.0, (255, 80, 80), 0.28);
    ball(c, o.0, o.1, ro, (230, 70, 70), (-0.5, -0.6));
    for p in [h1, h2] {
        halo(c, p.0, p.1, rh * 2.0, (230, 230, 240), 0.22);
        ball(c, p.0, p.1, rh, (225, 228, 235), (-0.5, -0.6));
    }
}

fn dna(c: &mut Canvas, cx: f64, _cy: f64, u: f64, h: f64) {
    fill(c, (5, 7, 14));
    let amp = u * 0.34;
    let turns = 3.4;
    let n = 260;
    let base = [(255, 110, 110), (120, 200, 255), (130, 240, 150), (255, 210, 120)];
    let mut prev: Option<((f64, f64), (f64, f64))> = None;
    for i in 0..=n {
        let t = i as f64 / n as f64;
        let y = t * h;
        let a = t * turns * TAU;
        let p1 = (cx + a.cos() * amp, y);
        let p2 = (cx + (a + PI).cos() * amp, y);
        if let Some((q1, q2)) = prev {
            // The strand in front is drawn brighter, so the twist reads.
            let front = a.sin() > 0.0;
            c.line(q1, p1, u * 0.035, if front { (240, 240, 255) } else { (120, 130, 170) }, 0.95);
            c.line(q2, p2, u * 0.035, if front { (120, 130, 170) } else { (240, 240, 255) }, 0.95);
        }
        if i % 9 == 0 {
            let rgb = base[(i / 9) % 4];
            c.line(p1, p2, u * 0.028, rgb, 0.85);
        }
        prev = Some((p1, p2));
    }
}

fn virus(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    fill(c, (7, 6, 14));
    let r = u * 0.55;
    // A shell with twenty faces, seen corner on, and its spikes.
    let mut d = Dice::new(53);
    for k in 0..26 {
        let a = k as f64 / 26.0 * TAU;
        let p = (cx + a.cos() * r * 1.02, cy + a.sin() * r * 1.02);
        let q = (cx + a.cos() * r * 1.3, cy + a.sin() * r * 1.3);
        c.line(p, q, u * 0.03, (200, 160, 110), 0.9);
        ball(c, q.0, q.1, u * 0.055, (240, 200, 140), (-0.4, -0.5));
    }
    ball(c, cx, cy, r, (110, 150, 120), (-0.5, -0.6));
    // The facets, as faint creases across the shell.
    for k in 0..5 {
        let a = k as f64 / 5.0 * TAU + 0.3;
        let p = (cx + a.cos() * r, cy + a.sin() * r);
        let q = (cx + (a + TAU / 5.0).cos() * r, cy + (a + TAU / 5.0).sin() * r);
        c.line(p, q, u * 0.012, (60, 90, 70), 0.6);
        c.line((cx, cy), p, u * 0.01, (70, 100, 80), 0.35);
    }
    for _ in 0..40 {
        let a = d.next() * TAU;
        let k = d.next().powf(0.6) * r * 0.9;
        c.disc(cx + a.cos() * k, cy + a.sin() * k, d.span(0.6, 1.8), (150, 190, 160), 0.3);
    }
}

fn bacterium(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    fill(c, (6, 9, 12));
    let len = u * 0.62;
    let rad = u * 0.3;
    let mut d = Dice::new(67);
    // Tails first, so the body sits over them.
    for k in 0..4 {
        let y = cy + (k as f64 - 1.5) * rad * 0.5;
        let mut prev = (cx + len, y);
        for s in 1..=40 {
            let t = s as f64 / 40.0;
            let x = cx + len + t * u * 0.9;
            let yy = y + (t * 9.0 + k as f64).sin() * rad * 0.45 * t;
            c.line(prev, (x, yy), u * 0.016, (150, 210, 180), 0.75);
            prev = (x, yy);
        }
    }
    // A capsule: two ends and the middle.
    c.line((cx - len, cy), (cx + len, cy), rad * 2.0, (90, 190, 140), 1.0);
    ball(c, cx - len, cy, rad, (110, 210, 160), (-0.5, -0.6));
    ball(c, cx + len, cy, rad, (110, 210, 160), (-0.5, -0.6));
    for x in 0..((len * 2.0) as usize) {
        let xx = cx - len + x as f64;
        let t = (xx - (cx - len)) / (len * 2.0);
        let _ = t;
        for y in 0..(rad as usize * 2) {
            let yy = cy - rad + y as f64;
            let dy = (yy - cy) / rad;
            if dy.abs() > 1.0 {
                continue;
            }
            let f = 0.35 + 0.8 * (1.0 - dy * dy).sqrt() * (0.5 - dy * 0.5 + 0.5);
            c.blend(xx as i64, yy as i64, shade((110, 210, 160), f), 1.0);
        }
    }
    // The tangle of DNA inside.
    let mut prev = (cx - len * 0.5, cy);
    for _ in 0..70 {
        let p = (
            cx + d.span(-len * 0.75, len * 0.75),
            cy + d.span(-rad * 0.55, rad * 0.55),
        );
        c.line(prev, p, u * 0.012, (60, 130, 100), 0.5);
        prev = p;
    }
}

fn blood(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    fill(c, (24, 6, 10));
    let mut d = Dice::new(71);
    // A few cells, dished on both sides, at different depths.
    let spots = [
        (cx, cy, u * 0.46, 1.0),
        (cx - u * 0.72, cy + u * 0.3, u * 0.34, 0.75),
        (cx + u * 0.68, cy - u * 0.26, u * 0.3, 0.7),
        (cx + u * 0.3, cy + u * 0.62, u * 0.26, 0.6),
    ];
    for (x, y, r, k) in spots {
        halo(c, x, y, r * 1.6, (220, 40, 50), 0.25 * k);
        ball(c, x, y, r, shade((215, 55, 60), k), (-0.5, -0.6));
        // The dish: a darker well in the middle.
        for yy in ((y - r) as usize)..((y + r) as usize).min(c.h) {
            for xx in ((x - r) as usize)..((x + r) as usize).min(c.w) {
                let (dx, dy) = ((xx as f64 - x) / r, (yy as f64 - y) / r);
                let d2 = dx * dx + dy * dy;
                if d2 > 0.45 {
                    continue;
                }
                let a = (1.0 - d2 / 0.45).powf(0.7) * 0.6;
                c.blend(xx as i64, yy as i64, shade((120, 20, 30), k), a);
            }
        }
    }
    for _ in 0..200 {
        c.disc(d.next() * c.w as f64, d.next() * c.h as f64, d.span(0.4, 1.2), (255, 150, 150), 0.12);
    }
}

fn hair(c: &mut Canvas, cx: f64, _cy: f64, u: f64, h: f64) {
    fill(c, (16, 11, 9));
    let r = u * 0.78;
    let mut d = Dice::new(83);
    // The shaft: lit along one side, so it reads as round rather than
    // as a band.
    for x in ((cx - r) as usize)..((cx + r) as usize).min(c.w) {
        let dx = (x as f64 - cx) / r;
        if dx.abs() > 1.0 {
            continue;
        }
        let round = (1.0 - dx * dx).sqrt();
        let lit = (1.0 - (dx + 0.4).abs()).max(0.0).powf(1.4);
        let f = 0.22 + 0.55 * round + 0.7 * lit;
        for y in 0..c.h {
            c.blend(x as i64, y as i64, shade((118, 76, 44), f), 1.0);
        }
    }
    // Cuticle scales: shallow overlapping arcs, each one a shadow above
    // and a highlight below, the way a fish's scales catch light.
    let mut y = -h * 0.1;
    while y < h * 1.1 {
        let lean = u * 0.09;
        let n = 26;
        for k in 0..n {
            let t = k as f64 / (n - 1) as f64;
            let dx = -1.0 + 2.0 * t;
            let px = cx + dx * r;
            // The arc sags in the middle, since the scale wraps round.
            let sag = (1.0 - dx * dx).max(0.0).sqrt() * u * 0.05;
            let py = y + dx * lean + sag;
            c.disc(px, py, r * 0.035, (58, 34, 18), 0.55);
            c.disc(px, py + r * 0.05, r * 0.028, (196, 148, 96), 0.28);
        }
        y += u * d.span(0.24, 0.32);
    }
    // A few split ends of the cuticle, and the sheen down the shaft.
    for _ in 0..40 {
        let x = cx + d.span(-r, r);
        let y = d.next() * h;
        c.disc(x, y, d.span(0.5, 1.6), (220, 180, 130), d.span(0.1, 0.3));
    }
    halo(c, cx - r * 0.4, h * 0.45, r * 0.55, (255, 226, 190), 0.22);
}

fn sand(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    fill(c, (18, 16, 20));
    let mut d = Dice::new(97);
    let grains = [
        (cx, cy, u * 0.5, (215, 185, 130)),
        (cx - u * 0.8, cy + u * 0.4, u * 0.3, (190, 160, 140)),
        (cx + u * 0.75, cy + u * 0.2, u * 0.26, (225, 205, 170)),
        (cx + u * 0.2, cy - u * 0.7, u * 0.2, (170, 150, 120)),
    ];
    for (x, y, r, rgb) in grains {
        // An irregular lump: a ring of points at wandering radii, filled.
        let n = 11;
        let mut pts = Vec::new();
        for k in 0..n {
            let a = k as f64 / n as f64 * TAU;
            let rr = r * d.span(0.7, 1.15);
            pts.push((x + a.cos() * rr, y + a.sin() * rr));
        }
        for k in 0..n {
            let p = pts[k];
            let q = pts[(k + 1) % n];
            c.line(p, q, r * 0.1, shade(rgb, 0.7), 1.0);
            // Fill by fanning to the middle.
            for s in 0..24 {
                let t = s as f64 / 24.0;
                let a = (x + (p.0 - x) * t, y + (p.1 - y) * t);
                let b = (x + (q.0 - x) * t, y + (q.1 - y) * t);
                let f = 0.45 + 0.75 * (1.0 - t);
                c.line(a, b, r * 0.09, shade(rgb, f), 1.0);
            }
        }
        halo(c, x - r * 0.3, y - r * 0.3, r * 0.8, (255, 245, 220), 0.25);
    }
}

// ── things you can hold, and things you can see ─────────────────────

fn ant(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    wash(c, (44, 38, 30), (22, 19, 15));
    let body = (46, 26, 16);
    let leg = (38, 21, 13);
    let s = u * 0.19;
    // Head at the left, then thorax, a pinched waist, and the gaster.
    let head = (cx - s * 3.5, cy - s * 0.2);
    let thorax = (cx - s * 1.1, cy - s * 0.5);
    let waist = (cx + s * 0.5, cy);
    let gaster = (cx + s * 2.4, cy + s * 0.35);
    // Legs: three a side, each with a knee above the body and a foot
    // planted well out, which is what makes an ant read as an ant.
    let hips = [thorax.0 - s * 0.6, thorax.0, thorax.0 + s * 0.6];
    for (k, hx) in hips.iter().enumerate() {
        for sgn in [-1.0, 1.0] {
            let reach = s * (2.4 + k as f64 * 0.5);
            let knee = (hx - s * 0.5 + k as f64 * s * 0.5, cy + sgn * s * 1.6);
            let foot = (hx - s * 1.8 + k as f64 * s * 1.9, cy + sgn * reach);
            c.line((*hx, cy - s * 0.2), knee, s * 0.17, leg, 1.0);
            c.line(knee, foot, s * 0.13, leg, 1.0);
            c.disc(foot.0, foot.1, s * 0.09, leg, 1.0);
        }
    }
    // Antennae, elbowed halfway, as they really are.
    for sgn in [-1.0, 1.0] {
        let elbow = (head.0 - s * 1.1, cy + sgn * s * 0.9);
        let tip = (head.0 - s * 2.4, cy + sgn * s * 0.3);
        c.line((head.0 - s * 0.4, cy - s * 0.4), elbow, s * 0.12, leg, 1.0);
        c.line(elbow, tip, s * 0.1, leg, 1.0);
    }
    // Mandibles.
    for sgn in [-1.0, 1.0] {
        c.line((head.0 - s * 0.6, cy + sgn * s * 0.25), (head.0 - s * 1.35, cy + sgn * s * 0.55), s * 0.09, leg, 1.0);
    }
    c.line(thorax, waist, s * 0.34, body, 1.0);
    c.line(waist, (gaster.0 - s * 0.6, gaster.1), s * 0.5, body, 1.0);
    ball(c, head.0, head.1, s * 0.95, body, (-0.5, -0.7));
    ball(c, thorax.0, thorax.1, s * 0.8, body, (-0.5, -0.7));
    ball(c, thorax.0 + s * 0.7, thorax.1 + s * 0.15, s * 0.55, body, (-0.5, -0.7));
    ball(c, waist.0, waist.1, s * 0.26, body, (-0.5, -0.7));
    // The gaster is an egg, not a ball: longer than it is tall.
    for k in 0..16 {
        let t = k as f64 / 15.0;
        let x = gaster.0 - s * 0.9 + t * s * 2.0;
        let rr = s * 0.95 * (1.0 - (t - 0.42).abs().powf(1.7) * 1.5).max(0.12);
        ball(c, x, gaster.1 + t * s * 0.1, rr, body, (-0.5, -0.7));
    }
    // The eye, and a glint on the head.
    c.disc(head.0 - s * 0.3, cy - s * 0.55, s * 0.17, (12, 8, 8), 1.0);
    c.disc(head.0 - s * 0.36, cy - s * 0.62, s * 0.05, (190, 170, 150), 0.8);
}

fn hand(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    wash(c, (26, 24, 30), (14, 13, 16));
    let skin = (226, 178, 145);
    let s = u * 0.2;
    let palm = (cx, cy + s * 1.1);
    // Palm as a stack of capsules, then four fingers and a thumb.
    c.line((palm.0 - s * 0.7, palm.1 - s * 0.8), (palm.0 + s * 0.7, palm.1 - s * 0.8), s * 1.5, skin, 1.0);
    c.line((palm.0 - s * 0.6, palm.1 + s * 0.6), (palm.0 + s * 0.6, palm.1 + s * 0.6), s * 1.4, skin, 1.0);
    let tips = [1.55, 1.9, 1.8, 1.45];
    for k in 0..4 {
        let x = palm.0 + (k as f64 - 1.5) * s * 0.72;
        let top = palm.1 - s * 1.5 - s * tips[k];
        c.line((x, palm.1 - s * 1.3), (x, top), s * 0.33, skin, 1.0);
        ball(c, x, top, s * 0.17, shade(skin, 1.05), (-0.4, -0.6));
        // A crease at each knuckle.
        c.line((x - s * 0.16, palm.1 - s * 1.9), (x + s * 0.16, palm.1 - s * 1.9), s * 0.05, shade(skin, 0.75), 0.8);
    }
    let th = (palm.0 - s * 1.5, palm.1 - s * 0.2);
    c.line((palm.0 - s * 0.6, palm.1 + s * 0.2), th, s * 0.42, skin, 1.0);
    ball(c, th.0, th.1, s * 0.22, shade(skin, 1.05), (-0.4, -0.6));
    halo(c, cx - s, cy, u * 0.5, (255, 220, 200), 0.1);
}

fn body(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    wash(c, (22, 26, 38), (10, 12, 18));
    let skin = (228, 182, 150);
    let cloth = (70, 110, 160);
    let s = u * 0.115;
    let head = cy - s * 7.0;
    // Legs, arms, trunk, head: a person standing, seen from the front.
    for sgn in [-1.0, 1.0] {
        c.line((cx + sgn * s * 0.75, cy + s * 0.6), (cx + sgn * s * 1.0, cy + s * 4.2), s * 0.62, cloth, 1.0);
        c.line((cx + sgn * s * 1.0, cy + s * 4.2), (cx + sgn * s * 1.05, cy + s * 7.6), s * 0.52, cloth, 1.0);
        ball(c, cx + sgn * s * 1.05, cy + s * 7.8, s * 0.42, (40, 40, 48), (-0.4, -0.6));
        c.line((cx + sgn * s * 1.7, cy - s * 3.6), (cx + sgn * s * 2.3, cy - s * 0.4), s * 0.46, cloth, 1.0);
        c.line((cx + sgn * s * 2.3, cy - s * 0.4), (cx + sgn * s * 2.5, cy + s * 2.4), s * 0.4, skin, 1.0);
        ball(c, cx + sgn * s * 2.5, cy + s * 2.7, s * 0.3, skin, (-0.4, -0.6));
    }
    c.line((cx, cy - s * 4.2), (cx, cy + s * 0.8), s * 1.9, cloth, 1.0);
    c.line((cx, cy - s * 4.4), (cx, cy - s * 3.4), s * 2.6, cloth, 1.0);
    c.line((cx, cy - s * 5.4), (cx, cy - s * 4.6), s * 0.7, skin, 1.0);
    ball(c, cx, head, s * 1.25, skin, (-0.45, -0.6));
    // Hair, as a cap over the top of the head.
    for k in 0..30 {
        let a = PI + k as f64 / 29.0 * PI;
        c.disc(cx + a.cos() * s * 1.2, head + a.sin() * s * 1.2, s * 0.3, (60, 42, 30), 1.0);
    }
    halo(c, cx, cy, u * 1.1, (150, 190, 255), 0.06);
}

fn whale(c: &mut Canvas, cx: f64, cy: f64, u: f64, w: f64) {
    wash(c, (12, 40, 70), (4, 14, 30));
    let mut d = Dice::new(131);
    for _ in 0..90 {
        let x = d.next() * w;
        let y = d.next() * c.h as f64;
        c.disc(x, y, d.span(0.5, 1.6), (180, 220, 255), d.span(0.05, 0.2));
    }
    let blue = (70, 100, 135);
    let l = u * 0.95;
    // Body from snout to tail, tapering.
    let nose = (cx - l, cy);
    let tail = (cx + l, cy - u * 0.08);
    let steps = 42;
    let mut prev = nose;
    for i in 1..=steps {
        let t = i as f64 / steps as f64;
        let p = (nose.0 + (tail.0 - nose.0) * t, nose.1 + (tail.1 - nose.1) * t - (t * PI).sin() * u * 0.04);
        let thick = u * 0.3 * (t * PI).sin().max(0.05).powf(0.55) * (1.0 - t * 0.55);
        c.line(prev, p, thick.max(u * 0.02), blue, 1.0);
        prev = p;
    }
    // Flukes, flipper and the pale underside.
    c.line(tail, (tail.0 + u * 0.24, tail.1 - u * 0.22), u * 0.05, blue, 1.0);
    c.line(tail, (tail.0 + u * 0.24, tail.1 + u * 0.18), u * 0.05, blue, 1.0);
    c.line((cx - l * 0.35, cy + u * 0.1), (cx - l * 0.1, cy + u * 0.32), u * 0.05, shade(blue, 0.85), 1.0);
    for i in 0..steps {
        let t = i as f64 / steps as f64;
        let x = nose.0 + (tail.0 - nose.0) * t;
        let y = cy + u * 0.12 * (t * PI).sin();
        c.line((x, y), (x + l * 2.0 / steps as f64, y), u * 0.035, (190, 200, 205), 0.55);
    }
    c.disc(nose.0 + u * 0.12, cy - u * 0.06, u * 0.022, (10, 12, 16), 1.0);
    // A person alongside, to the same scale: a whale is 30 metres.
    let ps = u * 0.033;
    let px = cx - l * 0.2;
    let py = cy + u * 0.62;
    c.line((px, py - ps * 3.0), (px, py), ps * 0.9, (240, 230, 220), 1.0);
    ball(c, px, py - ps * 3.8, ps * 0.8, (240, 230, 220), (-0.4, -0.6));
    c.line((px, py), (px - ps * 1.2, py + ps * 2.6), ps * 0.6, (240, 230, 220), 1.0);
    c.line((px, py), (px + ps * 1.2, py + ps * 2.6), ps * 0.6, (240, 230, 220), 1.0);
    c.line((px, py - ps * 2.4), (px - ps * 1.6, py - ps * 0.6), ps * 0.5, (240, 230, 220), 1.0);
    c.line((px, py - ps * 2.4), (px + ps * 1.6, py - ps * 0.6), ps * 0.5, (240, 230, 220), 1.0);
}

fn pitch(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    fill(c, (26, 60, 30));
    let hw = u * 0.86;
    let hh = u * 0.54;
    // Mown stripes, then the markings.
    let bands = 12;
    for k in 0..bands {
        let x0 = cx - hw + k as f64 * hw * 2.0 / bands as f64;
        let x1 = x0 + hw * 2.0 / bands as f64;
        let g = if k % 2 == 0 { (52, 120, 56) } else { (44, 104, 48) };
        for x in (x0 as usize)..(x1 as usize).min(c.w) {
            for y in ((cy - hh) as usize)..((cy + hh) as usize).min(c.h) {
                c.put(x, y, g);
            }
        }
    }
    let white = (235, 240, 235);
    let t = u * 0.012;
    c.line((cx - hw, cy - hh), (cx + hw, cy - hh), t, white, 1.0);
    c.line((cx - hw, cy + hh), (cx + hw, cy + hh), t, white, 1.0);
    c.line((cx - hw, cy - hh), (cx - hw, cy + hh), t, white, 1.0);
    c.line((cx + hw, cy - hh), (cx + hw, cy + hh), t, white, 1.0);
    c.line((cx, cy - hh), (cx, cy + hh), t, white, 1.0);
    c.ring(cx, cy, u * 0.16, white, 1.0);
    c.disc(cx, cy, t, white, 1.0);
    for sgn in [-1.0, 1.0] {
        let x = cx + sgn * hw;
        c.line((x, cy - hh * 0.45), (x - sgn * u * 0.15, cy - hh * 0.45), t, white, 1.0);
        c.line((x, cy + hh * 0.45), (x - sgn * u * 0.15, cy + hh * 0.45), t, white, 1.0);
        c.line((x - sgn * u * 0.15, cy - hh * 0.45), (x - sgn * u * 0.15, cy + hh * 0.45), t, white, 1.0);
        c.line((x, cy - hh * 0.2), (x - sgn * u * 0.05, cy - hh * 0.2), t, white, 1.0);
        c.line((x, cy + hh * 0.2), (x - sgn * u * 0.05, cy + hh * 0.2), t, white, 1.0);
        c.line((x - sgn * u * 0.05, cy - hh * 0.2), (x - sgn * u * 0.05, cy + hh * 0.2), t, white, 1.0);
    }
}

fn tower(c: &mut Canvas, cx: f64, _cy: f64, u: f64, h: f64) {
    wash(c, (30, 44, 84), (120, 130, 160));
    let mut d = Dice::new(149);
    let base = h * 0.94;
    let top = h * 0.06;
    let halfw = u * 0.17;
    // A tapering tower in three set-backs, with lit windows.
    let tiers = [(0.0, 1.0), (0.42, 0.66), (0.72, 0.36)];
    for (from, wide) in tiers {
        let y0 = base - (base - top) * from;
        let y1 = top;
        let hw = halfw * wide;
        for x in ((cx - hw) as usize)..((cx + hw) as usize).min(c.w) {
            let dx = (x as f64 - cx) / hw;
            let f = 0.55 + 0.5 * (1.0 - dx.abs()).powf(0.6) - dx * 0.18;
            for y in (y1 as usize)..(y0 as usize).min(c.h) {
                c.put(x, y, shade((118, 126, 140), f));
            }
        }
        let mut y = y1 + u * 0.03;
        while y < y0 {
            let mut x = cx - hw + u * 0.02;
            while x < cx + hw - u * 0.01 {
                if d.odds(0.42) {
                    c.disc(x, y, u * 0.008, (255, 226, 160), d.span(0.5, 1.0));
                }
                x += u * 0.022;
            }
            y += u * 0.028;
        }
    }
    c.line((cx, top), (cx, top - u * 0.22), u * 0.008, (200, 205, 215), 1.0);
    halo(c, cx, top - u * 0.22, u * 0.05, (255, 120, 110), 0.8);
    // Ground haze, so the tower stands on something.
    for y in ((base) as usize)..c.h {
        for x in 0..c.w {
            c.blend(x as i64, y as i64, (90, 96, 110), 0.85);
        }
    }
}

fn everest(c: &mut Canvas, cx: f64, _cy: f64, u: f64, w: f64, h: f64) {
    wash(c, (20, 40, 90), (150, 175, 215));
    let mut d = Dice::new(163);
    let ground = h * 0.92;
    // Two ranges: the far one pale, the near one dark, and the peak.
    for (depth, rgb, hgt) in [(1.0_f64, (120, 140, 175), 0.42), (0.0, (72, 84, 110), 0.62)] {
        let peak = cx + (depth - 0.5) * u * 0.3;
        let top = ground - h * hgt;
        let n = 90;
        for k in 0..n {
            let t = k as f64 / n as f64;
            let x = t * w;
            // A ridge line: a tent with rough edges.
            let far = ((x - peak) / (u * (1.0 + depth * 0.5))).abs();
            let y = top + far.powf(1.25) * (ground - top) + d.span(-1.0, 1.0) * u * 0.012;
            let y = y.min(ground);
            for yy in (y as usize)..(ground as usize).min(c.h) {
                let f = 0.7 + 0.5 * (1.0 - (yy as f64 - y) / (ground - y + 1.0));
                c.put((x) as usize, yy, shade(rgb, f));
                let step = (w / n as f64).ceil() as usize;
                for e in 1..step {
                    if (x as usize + e) < c.w {
                        c.put(x as usize + e, yy, shade(rgb, f));
                    }
                }
            }
            // Snow on the high ground.
            if y < top + (ground - top) * 0.3 {
                let step = (w / n as f64).ceil() as usize;
                for e in 0..step.max(1) {
                    for yy in (y as usize)..((y + u * d.span(0.04, 0.12)) as usize).min(c.h) {
                        if x as usize + e < c.w {
                            c.put(x as usize + e, yy, (240, 246, 252));
                        }
                    }
                }
            }
        }
    }
    // A plume of snow off the summit, the way Everest always has one.
    for _ in 0..200 {
        let t = d.next();
        let x = cx + t * u * 0.9;
        let y = ground - h * 0.62 - t * u * 0.12 + d.span(-1.0, 1.0) * u * 0.05 * t;
        c.disc(x, y, d.span(0.5, 2.0), (255, 255, 255), (1.0 - t) * 0.35);
    }
}

fn city(c: &mut Canvas, w: f64, h: f64) {
    fill(c, (5, 6, 11));
    let mut d = Dice::new(179);
    let (cx, cy) = (w / 2.0, h / 2.0);
    // How built-up a place is: dense in the middle, thinning outwards,
    // and nothing in the water.
    let river = |x: f64| h * 0.63 + (x / w * 6.0).sin() * h * 0.09;
    let density = |x: f64, y: f64| {
        let (dx, dy) = ((x - cx) / (w * 0.5), (y - cy) / (h * 0.5));
        (1.0 - (dx * dx + dy * dy).sqrt()).max(0.0).powf(1.5)
    };
    // Streets: two families of lines crossing at a slight angle, the
    // way a real grid sits on the land.
    for (ang, count) in [(0.13_f64, 34usize), (0.13 + PI / 2.0, 30)] {
        for k in 0..count {
            let off = (k as f64 / count as f64 - 0.5) * w * 1.3;
            let (ca, sa) = (ang.cos(), ang.sin());
            let n = 520;
            for i in 0..n {
                let t = (i as f64 / n as f64 - 0.5) * w * 1.4;
                let x = cx + ca * t - sa * off;
                let y = cy + sa * t + ca * off * 0.62;
                if x < 0.0 || y < 0.0 || x >= w || y >= h {
                    continue;
                }
                if (y - river(x)).abs() < h * 0.035 {
                    continue;
                }
                let dens = density(x, y);
                if dens <= 0.0 || !d.odds(0.35 + dens * 0.6) {
                    continue;
                }
                let rgb = if d.odds(0.82) { (255, 202, 126) } else { (176, 212, 255) };
                let br = dens * d.span(0.35, 1.0);
                c.disc(x, y, d.span(0.35, 0.9), rgb, br);
                if d.odds(0.1) {
                    halo(c, x, y, d.span(1.5, 4.0), rgb, br * 0.3);
                }
            }
        }
    }
    // The bright core, where the buildings crowd together.
    for _ in 0..2200 {
        let a = d.next() * TAU;
        let k = d.next().powf(2.2);
        let x = cx + a.cos() * k * w * 0.2;
        let y = cy + a.sin() * k * h * 0.2;
        if (y - river(x)).abs() < h * 0.035 {
            continue;
        }
        let rgb = if d.odds(0.8) { (255, 210, 140) } else { (190, 220, 255) };
        c.disc(x, y, d.span(0.3, 0.8), rgb, (1.0 - k) * d.span(0.4, 1.0));
    }
    halo(c, cx, cy, w * 0.16, (255, 190, 110), 0.16);
    // The river: dark, with the lights of the far bank reflected in it.
    let mut prev = (0.0, river(0.0));
    for k in 1..=90 {
        let x = k as f64 / 90.0 * w;
        let p = (x, river(x));
        c.line(prev, p, h * 0.055, (7, 10, 20), 1.0);
        prev = p;
    }
    for _ in 0..260 {
        let x = d.next() * w;
        let y = river(x) + d.span(-h * 0.025, h * 0.025);
        c.disc(x, y, d.span(0.4, 1.4), (255, 190, 120), d.span(0.05, 0.25) * density(x, y));
    }
    // Two bridges.
    for bx in [w * 0.36, w * 0.64] {
        for k in 0..30 {
            let y = river(bx) - h * 0.045 + k as f64 / 29.0 * h * 0.09;
            c.disc(bx, y, 0.8, (255, 220, 160), 0.8);
        }
    }
}

fn coast(c: &mut Canvas, w: f64, h: f64) {
    fill(c, (10, 26, 52));
    let mut d = Dice::new(191);
    // Land on one side of a wandering line, sea on the other.
    let n = 200;
    let mut edge = vec![0.0; n + 1];
    let mut x = w * 0.52;
    for k in 0..=n {
        x += d.span(-1.0, 1.0) * w * 0.035;
        // Big bays as well as small wrinkles.
        let bay = ((k as f64 / n as f64) * 7.0).sin() * w * 0.09;
        edge[k] = (x + bay).clamp(w * 0.12, w * 0.82);
    }
    for y in 0..c.h {
        let t = y as f64 / h * n as f64;
        let k = (t as usize).min(n);
        let e = edge[k];
        for xx in 0..c.w {
            if (xx as f64) < e {
                let deep = ((e - xx as f64) / (w * 0.4)).clamp(0.0, 1.0);
                c.put(xx, y, mix((26, 70, 120), (6, 18, 44), deep));
            } else {
                let inland = ((xx as f64 - e) / (w * 0.5)).clamp(0.0, 1.0);
                let green = mix((58, 92, 52), (96, 108, 70), inland);
                c.put(xx, y, green);
            }
        }
    }
    // Snow on the high ground inland, and cloud over the sea.
    for _ in 0..900 {
        let k = d.next();
        let y = k * h;
        let e = edge[((k * n as f64) as usize).min(n)];
        let x = e + d.span(0.08, 0.5) * w;
        if x < w {
            c.disc(x, y, d.span(0.6, 2.4), (235, 240, 245), d.span(0.1, 0.5));
        }
    }
    cloud(c, 193, w * 0.22, h * 0.3, w * 0.2, h * 0.16, (255, 255, 255), 160);
    cloud(c, 197, w * 0.6, h * 0.8, w * 0.22, h * 0.14, (250, 250, 255), 140);
}

// ── worlds ──────────────────────────────────────────────────────────

fn moon(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    fill(c, (3, 3, 6));
    starfield(c, 211, 140);
    let r = u * 0.62;
    ball(c, cx, cy, r, (176, 172, 166), (-0.55, -0.45));
    let mut d = Dice::new(223);
    // Maria first, as broad dark patches, then craters on top.
    for _ in 0..7 {
        let a = d.next() * TAU;
        let k = d.next().powf(0.6) * 0.72;
        let x = cx + a.cos() * r * k;
        let y = cy + a.sin() * r * k;
        halo(c, x, y, r * d.span(0.18, 0.4), (70, 70, 80), 0.55);
    }
    for _ in 0..90 {
        let a = d.next() * TAU;
        let k = d.next().powf(0.5) * 0.93;
        let x = cx + a.cos() * r * k;
        let y = cy + a.sin() * r * k;
        let cr = d.span(0.012, 0.075) * r;
        // Lit rim on one side, shadow on the other.
        c.disc(x, y, cr, shade((150, 146, 142), 0.8), 0.7);
        c.ring(x - cr * 0.12, y - cr * 0.12, cr, (215, 212, 206), 0.5);
        c.disc(x + cr * 0.2, y + cr * 0.2, cr * 0.55, (110, 108, 106), 0.45);
    }
}

fn earth(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    fill(c, (2, 2, 6));
    starfield(c, 227, 120);
    let r = u * 0.6;
    halo(c, cx, cy, r * 1.22, (90, 150, 255), 0.35);
    ball(c, cx, cy, r, (36, 86, 160), (-0.5, -0.45));
    let mut d = Dice::new(229);
    // Continents as clumps of green, then ice, then weather.
    for _ in 0..9 {
        let a = d.next() * TAU;
        let k = d.next().powf(0.55) * 0.8;
        let x = cx + a.cos() * r * k;
        let y = cy + a.sin() * r * k;
        let rr = d.span(0.1, 0.3) * r;
        for _ in 0..90 {
            let aa = d.next() * TAU;
            let kk = d.next().powf(0.5);
            let px = x + aa.cos() * rr * kk;
            let py = y + aa.sin() * rr * kk;
            let dd = ((px - cx).powi(2) + (py - cy).powi(2)).sqrt() / r;
            if dd > 0.99 {
                continue;
            }
            let lam = 1.0 - dd * dd * 0.55;
            c.disc(px, py, rr * 0.16, shade(mix((70, 120, 54), (140, 130, 80), d.next()), lam), 0.9);
        }
    }
    for sgn in [-1.0, 1.0] {
        halo(c, cx, cy + sgn * r * 0.9, r * 0.35, (245, 250, 255), 0.5);
    }
    for _ in 0..40 {
        let a = d.next() * TAU;
        let k = d.next().powf(0.5) * 0.92;
        let x = cx + a.cos() * r * k;
        let y = cy + a.sin() * r * k;
        cloud(c, (d.next() * 1e6) as u64, x, y, r * 0.12, r * 0.06, (255, 255, 255), 14);
    }
    // The night side, cut cleanly against the lit half.
    for y in ((cy - r) as usize)..((cy + r) as usize).min(c.h) {
        for x in ((cx - r) as usize)..((cx + r) as usize).min(c.w) {
            let (dx, dy) = ((x as f64 - cx) / r, (y as f64 - cy) / r);
            if dx * dx + dy * dy > 1.0 {
                continue;
            }
            let lam = (dx * 0.55 + dy * 0.45 + (1.0 - dx * dx - dy * dy).max(0.0).sqrt() * 0.7).max(0.0);
            if lam < 0.35 {
                c.blend(x as i64, y as i64, (2, 4, 12), (0.35 - lam) / 0.35 * 0.9);
            }
        }
    }
}

fn jupiter(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    fill(c, (2, 2, 6));
    starfield(c, 233, 90);
    let r = u * 0.66;
    ball(c, cx, cy, r, (200, 170, 130), (-0.5, -0.45));
    let mut d = Dice::new(239);
    // Bands, following the curve of the ball.
    let bands = 15;
    for k in 0..bands {
        let t0 = -1.0 + 2.0 * k as f64 / bands as f64;
        let t1 = -1.0 + 2.0 * (k + 1) as f64 / bands as f64;
        let rgb = if k % 2 == 0 {
            mix((228, 206, 172), (246, 232, 206), d.next())
        } else {
            mix((150, 108, 74), (190, 150, 108), d.next())
        };
        for y in ((cy + t0 * r) as usize)..((cy + t1 * r) as usize).min(c.h) {
            let dy = (y as f64 - cy) / r;
            let hw = (1.0 - dy * dy).max(0.0).sqrt() * r;
            for x in ((cx - hw) as usize)..((cx + hw) as usize).min(c.w) {
                let dx = (x as f64 - cx) / r;
                let dz = (1.0 - dx * dx - dy * dy).max(0.0).sqrt();
                let lam = (dx * -0.5 + dy * -0.45 + dz).max(0.0) * 0.8 + 0.2;
                // A little turbulence along each band edge.
                let wob = ((x as f64 / r * 6.0 + k as f64).sin() * 0.04) as f64;
                let _ = wob;
                c.put(x, y, shade(rgb, lam));
            }
        }
    }
    // The Great Red Spot.
    let sx = cx + r * 0.3;
    let sy = cy + r * 0.28;
    for y in ((sy - r * 0.16) as usize)..((sy + r * 0.16) as usize).min(c.h) {
        for x in ((sx - r * 0.26) as usize)..((sx + r * 0.26) as usize).min(c.w) {
            let (dx, dy) = ((x as f64 - sx) / (r * 0.26), (y as f64 - sy) / (r * 0.16));
            let d2 = dx * dx + dy * dy;
            if d2 > 1.0 {
                continue;
            }
            c.blend(x as i64, y as i64, mix((200, 90, 60), (150, 60, 45), d2), (1.0 - d2).powf(0.5));
        }
    }
    for y in ((cy - r) as usize)..((cy + r) as usize).min(c.h) {
        for x in ((cx - r) as usize)..((cx + r) as usize).min(c.w) {
            let (dx, dy) = ((x as f64 - cx) / r, (y as f64 - cy) / r);
            let d2 = dx * dx + dy * dy;
            if d2 > 1.0 {
                continue;
            }
            let dz = (1.0 - d2).sqrt();
            let lam = (dx * -0.5 + dy * -0.45 + dz * 0.75).max(0.0);
            if lam < 0.3 {
                c.blend(x as i64, y as i64, (6, 5, 10), (0.3 - lam) / 0.3 * 0.75);
            }
        }
    }
}

fn sun(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    fill(c, (3, 2, 4));
    let r = u * 0.42;
    let mut d = Dice::new(251);
    // Corona: long streamers, faint and far out.
    for _ in 0..420 {
        let a = d.next() * TAU;
        let l = r * d.span(1.05, 2.6);
        let p = (cx + a.cos() * r * 1.0, cy + a.sin() * r * 1.0);
        let q = (cx + a.cos() * l, cy + a.sin() * l);
        c.line(p, q, d.span(0.6, 2.4), (255, 190, 110), d.span(0.02, 0.09));
    }
    halo(c, cx, cy, r * 2.4, (255, 160, 60), 0.5);
    halo(c, cx, cy, r * 1.3, (255, 220, 150), 0.8);
    // The surface: granules and a couple of spots.
    for y in ((cy - r) as usize)..((cy + r) as usize).min(c.h) {
        for x in ((cx - r) as usize)..((cx + r) as usize).min(c.w) {
            let (dx, dy) = ((x as f64 - cx) / r, (y as f64 - cy) / r);
            let d2 = dx * dx + dy * dy;
            if d2 > 1.0 {
                continue;
            }
            // Brighter in the middle, redder at the rim, as it really is.
            let limb = 1.0 - d2 * 0.45;
            c.put(x, y, shade(mix((255, 245, 200), (255, 150, 40), d2.powf(0.7)), limb));
        }
    }
    for _ in 0..700 {
        let a = d.next() * TAU;
        let k = d.next().powf(0.5) * 0.97;
        c.disc(cx + a.cos() * r * k, cy + a.sin() * r * k, d.span(0.4, 1.6), (255, 255, 230), d.span(0.05, 0.2));
    }
    for (sx, sy, sr) in [(0.25, -0.2, 0.09), (-0.32, 0.26, 0.06), (0.1, 0.4, 0.045)] {
        let x = cx + sx * r;
        let y = cy + sy * r;
        c.disc(x, y, sr * r, (120, 60, 20), 0.75);
        c.disc(x, y, sr * r * 0.55, (70, 30, 10), 0.85);
    }
    // Earth, to scale: one hundred and nine of these across the Sun.
    let er = r / 109.0;
    let ex = cx + u * 0.82;
    let ey = cy + u * 0.5;
    halo(c, ex, ey, (er * 6.0).max(2.5), (110, 170, 255), 0.7);
    c.disc(ex, ey, er.max(0.8), (120, 180, 255), 1.0);
}

// ── the solar system and out ────────────────────────────────────────

fn inner_system(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    fill(c, (2, 2, 6));
    starfield(c, 257, 110);
    halo(c, cx, cy, u * 0.3, (255, 190, 90), 0.85);
    c.disc(cx, cy, u * 0.035, (255, 245, 210), 1.0);
    // Mercury, Venus, Earth, Mars, to their real spacings.
    let au = u * 0.82;
    let bodies = [
        (0.387, 0.012, (170, 160, 150), "Mercury"),
        (0.723, 0.02, (230, 200, 150), "Venus"),
        (1.0, 0.021, (90, 150, 240), "Earth"),
        (1.524, 0.016, (210, 110, 70), "Mars"),
    ];
    for (a, sz, rgb, _n) in bodies {
        let rx = au * a;
        orbit(c, cx, cy, rx, rx * 0.42, (120, 140, 190), 0.4);
        let ang: f64 = match a {
            x if x < 0.4 => 0.7,
            x if x < 0.8 => 2.4,
            x if x < 1.1 => 4.3,
            _ => 5.5,
        };
        let x = cx + ang.cos() * rx;
        let y = cy + ang.sin() * rx * 0.42;
        halo(c, x, y, u * sz * 4.0, rgb, 0.55);
        ball(c, x, y, u * sz, rgb, (-0.5, -0.5));
    }
}

fn solar_system(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    fill(c, (2, 2, 6));
    starfield(c, 263, 130);
    halo(c, cx, cy, u * 0.14, (255, 190, 90), 0.9);
    c.disc(cx, cy, u * 0.014, (255, 245, 210), 1.0);
    let au = u * 0.031;
    let bodies = [
        (0.39, 0.006, (170, 160, 150)),
        (0.72, 0.009, (230, 200, 150)),
        (1.0, 0.009, (90, 150, 240)),
        (1.52, 0.007, (210, 110, 70)),
        (5.2, 0.022, (220, 180, 130)),
        (9.5, 0.019, (230, 210, 160)),
        (19.2, 0.014, (140, 210, 220)),
        (30.1, 0.014, (90, 130, 230)),
    ];
    for (a, sz, rgb) in bodies {
        let rx = au * a;
        orbit(c, cx, cy, rx, rx * 0.4, (110, 130, 180), 0.32);
        let ang = a * 2.1;
        let x = cx + ang.cos() * rx;
        let y = cy + ang.sin() * rx * 0.4;
        halo(c, x, y, u * sz * 4.0, rgb, 0.5);
        ball(c, x, y, u * sz, rgb, (-0.5, -0.5));
    }
    // The asteroid belt, between Mars and Jupiter.
    let mut d = Dice::new(269);
    for _ in 0..500 {
        let a = d.next() * TAU;
        let rr = au * d.span(2.1, 3.3);
        c.disc(cx + a.cos() * rr, cy + a.sin() * rr * 0.4, d.span(0.3, 0.8), (190, 180, 165), 0.45);
    }
}

fn oort(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    fill(c, (2, 2, 6));
    starfield(c, 271, 90);
    let mut d = Dice::new(277);
    // A hollow shell of ice, seen through. Denser at the rim, because
    // that is where the line of sight runs longest through it.
    for _ in 0..4200 {
        let a = d.next() * TAU;
        let k = d.span(0.62, 0.95);
        // Points on a sphere, projected: bunch up at the edge.
        let z = d.span(-1.0, 1.0);
        let rr = (1.0 - z * z).max(0.0).sqrt() * k;
        let x = cx + a.cos() * rr * u * 0.95;
        let y = cy + a.sin() * rr * u * 0.95;
        c.disc(x, y, d.span(0.3, 0.9), (200, 220, 255), d.span(0.1, 0.5));
    }
    // The whole planetary system is the speck in the middle.
    halo(c, cx, cy, u * 0.09, (255, 200, 110), 0.8);
    c.disc(cx, cy, u * 0.006, (255, 250, 220), 1.0);
    orbit(c, cx, cy, u * 0.02, u * 0.008, (150, 170, 220), 0.5);
}

fn near_stars(c: &mut Canvas, w: f64, h: f64) {
    fill(c, (2, 2, 7));
    starfield(c, 281, 260);
    let mut d = Dice::new(283);
    let (cx, cy) = (w / 2.0, h / 2.0);
    // A handful of near stars, each with its colour and its distance.
    let near = [
        (0.0, 0.0, 1.0, (255, 240, 210)),
        (-0.55, -0.3, 0.55, (255, 180, 120)),
        (0.62, 0.22, 0.5, (255, 200, 150)),
        (0.3, -0.58, 0.42, (200, 220, 255)),
        (-0.34, 0.6, 0.38, (255, 150, 110)),
        (0.78, -0.42, 0.3, (255, 255, 230)),
        (-0.8, 0.18, 0.28, (255, 170, 130)),
    ];
    for (fx, fy, br, rgb) in near {
        let x = cx + fx * w * 0.42;
        let y = cy + fy * h * 0.42;
        halo(c, x, y, (6.0 + br * 26.0) * (w / 400.0).max(0.6), rgb, br * 0.8);
        c.disc(x, y, (1.0 + br * 2.4) * (w / 400.0).max(0.6), (255, 255, 255), 1.0);
        // The four-point flare that a bright star seems to have.
        let l = (10.0 + br * 40.0) * (w / 400.0).max(0.6);
        c.line((x - l, y), (x + l, y), 1.0, rgb, br * 0.35);
        c.line((x, y - l * 0.5), (x, y + l * 0.5), 1.0, rgb, br * 0.35);
    }
    for _ in 0..70 {
        let x = d.next() * w;
        let y = d.next() * h;
        halo(c, x, y, d.span(2.0, 6.0), (200, 210, 255), 0.2);
    }
}

fn orion(c: &mut Canvas, w: f64, h: f64) {
    fill(c, (6, 4, 12));
    let (cx, cy) = (w / 2.0, h / 2.0);
    // Glowing hydrogen, dust lanes in front, and the young stars that
    // light the whole thing.
    cloud(c, 293, cx, cy, w * 0.42, h * 0.4, (200, 60, 80), 900);
    cloud(c, 307, cx - w * 0.12, cy - h * 0.06, w * 0.24, h * 0.24, (90, 140, 220), 500);
    cloud(c, 311, cx + w * 0.2, cy + h * 0.14, w * 0.2, h * 0.18, (220, 130, 90), 420);
    let mut d = Dice::new(313);
    for _ in 0..260 {
        let a = d.next() * TAU;
        let k = d.next().powf(0.7);
        let x = cx + a.cos() * w * 0.34 * k;
        let y = cy + a.sin() * h * 0.32 * k;
        halo(c, x, y, d.span(6.0, 22.0), (20, 8, 14), d.span(0.1, 0.35));
    }
    starfield(c, 317, 320);
    // The four bright young stars in the middle.
    for (fx, fy, br) in [(-0.05, -0.02, 1.0), (0.05, 0.03, 0.8), (-0.02, 0.06, 0.7), (0.08, -0.05, 0.6)] {
        let x = cx + fx * w;
        let y = cy + fy * h;
        halo(c, x, y, (14.0 + br * 40.0) * (w / 400.0).max(0.6), (190, 215, 255), br * 0.9);
        c.disc(x, y, 1.6 * (w / 400.0).max(0.6), (255, 255, 255), 1.0);
    }
}

fn cluster(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    fill(c, (3, 3, 8));
    starfield(c, 331, 120);
    let mut d = Dice::new(337);
    // Thousands of old stars, crowding towards a bright middle.
    for _ in 0..6000 {
        let a = d.next() * TAU;
        // A steep fall-off, so the core glows and the edge frays.
        let k = d.next().powf(2.6);
        let rr = k * u * 0.95;
        let x = cx + a.cos() * rr;
        let y = cy + a.sin() * rr * 0.96;
        let br = (1.0 - k).powf(0.6) * d.span(0.35, 1.0);
        let rgb = if d.odds(0.72) {
            mix((255, 220, 170), (255, 170, 110), d.next())
        } else {
            mix((230, 235, 255), (255, 255, 255), d.next())
        };
        c.disc(x, y, d.span(0.3, 1.0), rgb, br);
        if d.odds(0.06) {
            halo(c, x, y, d.span(2.0, 6.0), rgb, br * 0.4);
        }
    }
    halo(c, cx, cy, u * 0.4, (255, 230, 190), 0.5);
}

fn galaxy(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    fill(c, (2, 2, 7));
    starfield(c, 347, 90);
    let mut d = Dice::new(349);
    let r = u * 0.92;
    // The disc, as a broad faint wash.
    halo(c, cx, cy, r, (70, 90, 170), 0.35);
    // Four arms, logarithmic, with young blue stars along them.
    for arm in 0..4 {
        let off = arm as f64 / 4.0 * TAU;
        for _ in 0..2200 {
            let t = d.next().powf(0.7);
            let ang = off + t * 3.1;
            let rr = r * (0.09 + t * 0.9);
            // Scatter across the arm, wider further out.
            let spread = (0.02 + t * 0.09) * r;
            let x = cx + ang.cos() * rr + d.span(-1.0, 1.0) * spread;
            let y = cy + ang.sin() * rr * 0.42 + d.span(-1.0, 1.0) * spread * 0.42;
            let young = d.odds(0.35);
            let rgb = if young {
                mix((150, 190, 255), (230, 240, 255), d.next())
            } else {
                mix((255, 220, 170), (255, 245, 220), d.next())
            };
            let br = d.span(0.2, 0.9) * (1.0 - t * 0.35);
            c.disc(x, y, d.span(0.3, 1.0), rgb, br);
            if young && d.odds(0.03) {
                halo(c, x, y, d.span(3.0, 9.0), (120, 170, 255), 0.3);
            }
        }
        // Dust in front of each arm, a little inside it.
        for _ in 0..500 {
            let t = d.next().powf(0.7);
            let ang = off + t * 3.1 - 0.16;
            let rr = r * (0.12 + t * 0.88);
            let x = cx + ang.cos() * rr;
            let y = cy + ang.sin() * rr * 0.42;
            halo(c, x, y, d.span(2.0, 7.0), (10, 6, 14), d.span(0.15, 0.4));
        }
    }
    // The bulge, and the bar across it.
    for _ in 0..3000 {
        let a = d.next() * TAU;
        let k = d.next().powf(2.2);
        let x = cx + a.cos() * k * r * 0.3;
        let y = cy + a.sin() * k * r * 0.14;
        c.disc(x, y, d.span(0.3, 0.9), mix((255, 225, 175), (255, 250, 230), d.next()), (1.0 - k) * 0.8);
    }
    halo(c, cx, cy, r * 0.3, (255, 220, 160), 0.6);
    halo(c, cx, cy, r * 0.1, (255, 245, 215), 0.8);
    // The Sun, two thirds of the way out.
    let sx = cx + r * 0.62;
    let sy = cy + r * 0.16;
    c.ring(sx, sy, u * 0.05, (120, 255, 180), 0.8);
    c.disc(sx, sy, 1.2, (220, 255, 230), 1.0);
}

/// One spiral galaxy, small enough to sit beside others.
fn small_spiral(c: &mut Canvas, d: &mut Dice, x: f64, y: f64, r: f64, tilt: f64, squash: f64) {
    halo(c, x, y, r * 1.15, (66, 86, 165), 0.42);
    halo(c, x, y, r * 0.72, (74, 94, 175), 0.3);
    let (ct, st) = (tilt.cos(), tilt.sin());
    let place = |ang: f64, rr: f64| {
        let (px, py) = (ang.cos() * rr, ang.sin() * rr * squash);
        (x + px * ct - py * st, y + px * st + py * ct)
    };
    for arm in 0..2 {
        let off = arm as f64 * PI;
        for _ in 0..4200 {
            let t = d.next().powf(0.62);
            let spread = (0.035 + t * 0.11) * r;
            let ang = off + t * 5.0 + d.span(-0.1, 0.1);
            let rr = r * (0.12 + t * 0.88);
            let (px, py) = place(ang, rr);
            let px = px + d.span(-spread, spread);
            let py = py + d.span(-spread, spread) * squash;
            let young = d.odds(0.38);
            let rgb = if young { (160, 195, 255) } else { (255, 224, 180) };
            c.disc(px, py, d.span(0.25, 0.8), rgb, d.span(0.25, 0.9) * (1.0 - t * 0.3));
        }
        for _ in 0..300 {
            let t = d.next().powf(0.65);
            let (px, py) = place(off + t * 5.0 - 0.2, r * (0.15 + t * 0.85));
            halo(c, px, py, d.span(1.5, 5.0), (8, 5, 12), d.span(0.15, 0.4));
        }
    }
    // The bulge.
    for _ in 0..2200 {
        let a = d.next() * TAU;
        let k = d.next().powf(2.2);
        let (px, py) = place(a, k * r * 0.3);
        c.disc(px, py, d.span(0.25, 0.8), mix((255, 228, 180), (255, 250, 235), d.next()), (1.0 - k) * 0.85);
    }
    halo(c, x, y, r * 0.3, (255, 222, 168), 0.6);
}

/// A galaxy with no arms: an old, smooth ball of stars.
fn blob_galaxy(c: &mut Canvas, d: &mut Dice, x: f64, y: f64, r: f64, squash: f64, warm: bool) {
    let rgb = if warm { (255, 226, 185) } else { (215, 222, 255) };
    for _ in 0..900 {
        let a = d.next() * TAU;
        let k = d.next().powf(1.9);
        c.disc(x + a.cos() * k * r, y + a.sin() * k * r * squash, d.span(0.2, 0.6), rgb, (1.0 - k) * 0.55);
    }
    halo(c, x, y, r * 1.2, rgb, 0.12);
    halo(c, x, y, r * 0.35, rgb, 0.2);
}

fn local_group(c: &mut Canvas, w: f64, h: f64) {
    fill(c, (2, 2, 7));
    starfield(c, 353, 60);
    let mut d = Dice::new(359);
    let s = w.min(h * 2.2);
    // Andromeda and the Milky Way, the two that matter, tilted apart.
    small_spiral(c, &mut d, w * 0.31, h * 0.38, s * 0.17, 0.42, 0.34);
    small_spiral(c, &mut d, w * 0.7, h * 0.62, s * 0.145, -0.75, 0.46);
    // The Triangulum galaxy, smaller, and the satellites around both.
    small_spiral(c, &mut d, w * 0.52, h * 0.82, s * 0.07, 1.4, 0.6);
    for _ in 0..22 {
        let a = d.next() * TAU;
        let k = d.next().powf(0.7);
        // Satellites huddle near one of the two big ones.
        let (hx, hy) = if d.odds(0.5) { (w * 0.31, h * 0.38) } else { (w * 0.7, h * 0.62) };
        let x = hx + a.cos() * k * s * 0.3;
        let y = hy + a.sin() * k * s * 0.22;
        if x < 0.0 || y < 0.0 || x >= w || y >= h {
            continue;
        }
        let r = d.span(0.012, 0.03) * s;
        let squash = d.span(0.5, 1.0);
        let warm = d.odds(0.6);
        blob_galaxy(c, &mut d, x, y, r, squash, warm);
    }
}

fn virgo(c: &mut Canvas, w: f64, h: f64) {
    fill(c, (2, 2, 7));
    starfield(c, 367, 50);
    let mut d = Dice::new(373);
    let (cx, cy) = (w / 2.0, h / 2.0);
    // A thousand galaxies, crowding to the middle, biggest at the core.
    for _ in 0..340 {
        let a = d.next() * TAU;
        let k = d.next().powf(1.8);
        let x = cx + a.cos() * k * w * 0.48;
        let y = cy + a.sin() * k * h * 0.46;
        let r = d.span(0.004, 0.02) * w * (1.4 - k);
        let ell = d.span(0.35, 1.0);
        let rgb = if d.odds(0.6) {
            mix((255, 230, 190), (255, 210, 160), d.next())
        } else {
            mix((200, 215, 255), (240, 245, 255), d.next())
        };
        for _ in 0..180 {
            let aa = d.next() * TAU;
            let kk = d.next().powf(1.7);
            c.disc(x + aa.cos() * kk * r, y + aa.sin() * kk * r * ell, d.span(0.2, 0.6), rgb, (1.0 - kk) * 0.75);
        }
        halo(c, x, y, r * 1.6, rgb, 0.2);
    }
    // The giant at the centre, with its jet.
    halo(c, cx, cy, w * 0.05, (255, 235, 200), 0.6);
    for _ in 0..900 {
        let a = d.next() * TAU;
        let k = d.next().powf(2.0);
        c.disc(cx + a.cos() * k * w * 0.045, cy + a.sin() * k * w * 0.042, d.span(0.3, 0.9), (255, 240, 210), (1.0 - k) * 0.85);
    }
    c.line((cx, cy), (cx + w * 0.11, cy - h * 0.09), 1.6, (170, 210, 255), 0.7);
    halo(c, cx + w * 0.06, cy - h * 0.05, w * 0.02, (160, 200, 255), 0.35);
}

fn laniakea(c: &mut Canvas, w: f64, h: f64) {
    fill(c, (2, 2, 7));
    let mut d = Dice::new(379);
    let (cx, cy) = (w * 0.62, h * 0.44);
    // Streams of galaxies, all running the same way: that flow is what
    // the name is for.
    for _ in 0..70 {
        let a0 = d.next() * TAU;
        let r0 = d.span(0.2, 1.0);
        let start = (cx + a0.cos() * r0 * w * 0.7, cy + a0.sin() * r0 * h * 0.8);
        let mut p = start;
        let n = 26;
        for s in 0..n {
            let t = s as f64 / n as f64;
            // Curve towards the attractor in the middle.
            let to = ((cx - p.0), (cy - p.1));
            let len = (to.0 * to.0 + to.1 * to.1).sqrt().max(1.0);
            let step = (to.0 / len, to.1 / len);
            let q = (
                p.0 + step.0 * w * 0.03 + d.span(-1.0, 1.0) * w * 0.012,
                p.1 + step.1 * w * 0.03 + d.span(-1.0, 1.0) * w * 0.012,
            );
            let br = (1.0 - t) * 0.5;
            c.line(p, q, d.span(0.5, 1.4), mix((120, 150, 230), (200, 220, 255), t), br * 0.5);
            if d.odds(0.5) {
                let rgb = if d.odds(0.5) { (255, 230, 190) } else { (200, 220, 255) };
                halo(c, q.0, q.1, d.span(1.5, 5.0), rgb, 0.3);
                c.disc(q.0, q.1, d.span(0.3, 1.0), rgb, d.span(0.4, 1.0));
            }
            p = q;
        }
    }
    halo(c, cx, cy, w * 0.14, (255, 220, 190), 0.3);
    halo(c, cx, cy, w * 0.05, (255, 240, 220), 0.5);
}

fn web(c: &mut Canvas, w: f64, h: f64) {
    fill(c, (2, 2, 6));
    let mut d = Dice::new(383);
    // Knots joined by filaments, and nothing at all between them.
    let n = 22;
    let mut knot = Vec::new();
    for _ in 0..n {
        knot.push((d.next() * w, d.next() * h, d.span(0.4, 1.0)));
    }
    for i in 0..n {
        for j in (i + 1)..n {
            let (a, b) = (knot[i], knot[j]);
            let dist = ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt();
            // Only near knots are joined, which leaves the voids empty.
            if dist > w * 0.3 {
                continue;
            }
            let count = (dist / w * 260.0) as usize;
            let rgb = mix((150, 170, 240), (255, 230, 200), d.next());
            strand(c, &mut d, (a.0, a.1), (b.0, b.1), count.max(20), rgb, w * 0.008);
        }
    }
    for (x, y, br) in &knot {
        let r = w * 0.02 * br;
        for _ in 0..500 {
            let a = d.next() * TAU;
            let k = d.next().powf(1.8);
            let rgb = if d.odds(0.55) { (255, 225, 185) } else { (200, 215, 255) };
            c.disc(x + a.cos() * k * r * 2.2, y + a.sin() * k * r * 2.2, d.span(0.25, 0.9), rgb, (1.0 - k) * br);
        }
        halo(c, *x, *y, r * 3.0, (255, 220, 190), 0.22 * br);
    }
}

fn whole(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    fill(c, (1, 1, 4));
    let mut d = Dice::new(389);
    let r = u * 0.94;
    // Inside: the web, faint, because this is everything at once.
    for _ in 0..7000 {
        let a = d.next() * TAU;
        let k = d.next().powf(0.42);
        let x = cx + a.cos() * k * r;
        let y = cy + a.sin() * k * r * 0.99;
        let rgb = if d.odds(0.5) { (190, 200, 255) } else { (255, 225, 190) };
        c.disc(x, y, d.span(0.2, 0.8), rgb, d.span(0.1, 0.5) * (1.0 - k * 0.4));
    }
    for _ in 0..40 {
        let a0 = d.next() * TAU;
        let k0 = d.span(0.1, 0.9);
        let a1 = a0 + d.span(-0.5, 0.5);
        let k1 = k0 + d.span(-0.3, 0.3);
        let from = (cx + a0.cos() * k0 * r, cy + a0.sin() * k0 * r);
        let to = (cx + a1.cos() * k1 * r, cy + a1.sin() * k1 * r);
        strand(c, &mut d, from, to, 70, (170, 190, 245), r * 0.01);
    }
    // The edge: the glow left from the beginning, mottled the way it is.
    for k in 0..3 {
        let rr = r * (1.0 - k as f64 * 0.016);
        c.ring(cx, cy, rr, mix((255, 170, 90), (255, 120, 70), k as f64 / 3.0), 0.5 - k as f64 * 0.12);
    }
    for _ in 0..2600 {
        let a = d.next() * TAU;
        let k = d.span(0.955, 1.0);
        let x = cx + a.cos() * k * r;
        let y = cy + a.sin() * k * r;
        let warm = d.next();
        c.disc(x, y, d.span(0.5, 1.8), mix((255, 140, 70), (120, 160, 255), warm), d.span(0.15, 0.55));
    }
    halo(c, cx, cy, r * 1.14, (255, 150, 80), 0.18);
}
