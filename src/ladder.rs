//! The rungs of the ladder, smallest first, and the picture for each.
//!
//! A rung says how wide the picture is in metres, not how big the thing
//! is. The thing sits inside that width with room around it, so one step
//! really does look like a step outwards.

use crate::paint::*;
use crate::photo::{self, Pic};
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
    // Loops and kinks of space, forming and going again. The rings do
    // the work; the glow around them is kept small, or this one scene
    // would cost more than all the others together.
    for _ in 0..240 {
        let x = d.next() * w;
        let y = d.next() * h;
        let r = d.span(0.008, 0.035) * w;
        let rgb = mix((120, 80, 220), (60, 190, 210), d.next());
        halo(c, x, y, r * 1.2, rgb, 0.16);
        c.ring(x, y, r, rgb, d.span(0.25, 0.7));
    }
    for _ in 0..200 {
        let x = d.next() * w;
        let y = d.next() * h;
        let a = d.next() * TAU;
        let l = d.span(0.008, 0.045) * w;
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
        for s in 1..=30 {
            let t = s as f64 / 30.0;
            let x = cx + len + t * u * 0.9;
            let yy = y + (t * 9.0 + k as f64).sin() * rad * 0.45 * t;
            c.line(prev, (x, yy), u * 0.016, (150, 210, 180), 0.75);
            prev = (x, yy);
        }
    }
    // The body as one pass over its box: distance to the middle line
    // gives both the outline and the shading, so nothing is drawn twice.
    let (x0, x1) = ((cx - len - rad) as i64, (cx + len + rad) as i64);
    let (y0, y1) = ((cy - rad) as i64, (cy + rad) as i64);
    for y in y0.max(0)..y1.min(c.h as i64) {
        for x in x0.max(0)..x1.min(c.w as i64) {
            let px = x as f64 + 0.5;
            let py = y as f64 + 0.5;
            // How far from the line between the two end points.
            let nearest = px.clamp(cx - len, cx + len);
            let dx = px - nearest;
            let dy = py - cy;
            let dist = (dx * dx + dy * dy).sqrt() / rad;
            if dist > 1.0 {
                continue;
            }
            let round = (1.0 - dist * dist).sqrt();
            let lit = 0.35 + 0.85 * round * (1.0 - (dy / rad + 0.35).abs() * 0.45).max(0.25);
            let edge = ((1.0 - dist) * rad).clamp(0.0, 1.0);
            c.blend(x, y, shade((104, 200, 152), lit), edge);
        }
    }
    // The tangle of DNA inside.
    let mut prev = (cx - len * 0.5, cy);
    for _ in 0..60 {
        let p = (cx + d.span(-len * 0.75, len * 0.75), cy + d.span(-rad * 0.55, rad * 0.55));
        c.line(prev, p, u * 0.012, (58, 128, 98), 0.5);
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
        // An irregular lump: a ring of wandering radii, filled by a
        // single pass over the box rather than by fanning lines.
        let n = 13;
        let mut edge = [0.0f64; 13];
        for k in 0..n {
            edge[k] = r * d.span(0.68, 1.15);
        }
        let (x0, x1) = ((x - r * 1.2) as i64, (x + r * 1.2) as i64);
        let (y0, y1) = ((y - r * 1.2) as i64, (y + r * 1.2) as i64);
        for py in y0.max(0)..y1.min(c.h as i64) {
            for px in x0.max(0)..x1.min(c.w as i64) {
                let (dx, dy) = (px as f64 + 0.5 - x, py as f64 + 0.5 - y);
                let dist = (dx * dx + dy * dy).sqrt();
                let ang = dy.atan2(dx) + PI;
                // Between two of the wandering radii, smoothly.
                let f = ang / TAU * n as f64;
                let i0 = (f as usize) % n;
                let i1 = (i0 + 1) % n;
                let t = f - f.floor();
                let want = edge[i0] * (1.0 - t) + edge[i1] * t;
                if dist > want {
                    continue;
                }
                let k = dist / want;
                // Facets: lighter towards the light, darker at the rim.
                let lit = 0.5 + 0.75 * (1.0 - k * k) - (dx + dy) / (r * 6.0);
                c.blend(px, py, shade(rgb, lit.clamp(0.15, 1.5)), ((want - dist) * 1.5).clamp(0.0, 1.0));
            }
        }
        halo(c, x - r * 0.3, y - r * 0.3, r * 0.7, (255, 245, 220), 0.22);
    }
}
fn ant(c: &mut Canvas, _cx: f64, _cy: f64, _u: f64) {
    fill(c, (10, 8, 7));
    // An engraving of a worker ant, seen from above.
    photo::inset(c, Pic::Ant, 0.96);
}
fn hand(c: &mut Canvas, _cx: f64, _cy: f64, _u: f64) {
    photo::cover(c, Pic::Hand, 1.0);
}
fn body(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    fill(c, (7, 7, 14));
    halo(c, cx, cy, u * 1.5, (90, 130, 210), 0.12);
    photo::inset(c, Pic::Body, 0.94);
}
fn whale(c: &mut Canvas, cx: f64, cy: f64, u: f64, _w: f64) {
    wash(c, (10, 32, 58), (3, 10, 24));
    photo::inset(c, Pic::Whale, 0.98);
    // A person alongside, to the same scale: a blue whale is thirty
    // metres, and that is the whole point of this rung.
    let ps = u * 0.062;
    let (px, py) = (cx - u * 0.82, cy + u * 0.5);
    c.line((px, py - ps * 2.8), (px, py), ps * 0.8, (235, 228, 220), 1.0);
    c.disc(px, py - ps * 3.5, ps * 0.72, (235, 228, 220), 1.0);
    c.line((px, py), (px - ps * 1.1, py + ps * 2.4), ps * 0.55, (235, 228, 220), 1.0);
    c.line((px, py), (px + ps * 1.1, py + ps * 2.4), ps * 0.55, (235, 228, 220), 1.0);
    c.line((px, py - ps * 2.2), (px - ps * 1.5, py - ps * 0.5), ps * 0.45, (235, 228, 220), 1.0);
    c.line((px, py - ps * 2.2), (px + ps * 1.5, py - ps * 0.5), ps * 0.45, (235, 228, 220), 1.0);
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

fn everest(c: &mut Canvas, _cx: f64, _cy: f64, _u: f64, _w: f64, _h: f64) {
    // The Himalaya from orbit: twenty-four kilometres of ridge and
    // glacier, with Everest among the peaks in the middle.
    photo::cover(c, Pic::Everest, 1.0);
}
fn city(c: &mut Canvas, _w: f64, _h: f64) {
    // A real city at night, photographed from the space station.
    photo::cover(c, Pic::City, 1.15);
}
fn coast(c: &mut Canvas, _w: f64, _h: f64) {
    // Scandinavia, in the spring, with the snow still on the high ground.
    photo::cover(c, Pic::Coast, 1.0);
}
fn moon(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    fill(c, (3, 3, 6));
    starfield(c, 211, 140);
    photo::moon_globe(c, cx, cy, u * 0.92, (-0.55, -0.45));
}
fn earth(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    fill(c, (2, 2, 6));
    starfield(c, 227, 120);
    let r = u * 0.9;
    halo(c, cx, cy, r * 1.16, (90, 150, 255), 0.32);
    photo::globe(c, Pic::Earth, cx, cy, r);
}
fn jupiter(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    fill(c, (2, 2, 6));
    starfield(c, 233, 90);
    photo::globe(c, Pic::Jupiter, cx, cy, u * 0.94);
}
fn sun(c: &mut Canvas, cx: f64, cy: f64, u: f64) {
    fill(c, (3, 2, 4));
    let r = u * 0.66;
    halo(c, cx, cy, r * 1.9, (255, 150, 50), 0.45);
    photo::globe(c, Pic::Sun, cx, cy, r);
    // Earth, to scale: a hundred and nine of these across the Sun.
    let er = (r / 109.0).max(0.9);
    let (ex, ey) = (cx + u * 0.84, cy + u * 0.72);
    halo(c, ex, ey, (er * 7.0).max(3.0), (110, 170, 255), 0.7);
    c.disc(ex, ey, er, (130, 185, 255), 1.0);
}
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
    cloud(c, 293, cx, cy, w * 0.42, h * 0.4, (200, 60, 80), 300);
    cloud(c, 307, cx - w * 0.12, cy - h * 0.06, w * 0.24, h * 0.24, (90, 140, 220), 200);
    cloud(c, 311, cx + w * 0.2, cy + h * 0.14, w * 0.2, h * 0.18, (220, 130, 90), 170);
    let mut d = Dice::new(313);
    for _ in 0..200 {
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
    // Our own galaxy, as it would look from outside: a barred spiral,
    // drawn from what the surveys have mapped.
    photo::cover(c, Pic::Galaxy, 1.0);
    // And the Sun, in a gap between two arms, two thirds of the way out.
    let (sx, sy) = (cx - u * 0.26, cy + u * 0.52);
    halo(c, sx, sy, u * 0.1, (120, 255, 180), 0.35);
    c.ring(sx, sy, u * 0.055, (150, 255, 200), 0.9);
    c.disc(sx, sy, u * 0.012, (240, 255, 245), 1.0);
}
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
