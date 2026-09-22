//! universe — every scale there is, one step at a time.
//!
//! The app opens on a human body, halfway up the ladder, and steps out
//! to the cosmic web or in to the quantum foam. Each step is a picture
//! of what that scale holds, drawn in real pixels where the terminal
//! shows them and in coloured half-blocks where it does not.
//!
//! A picture is painted when the step changes or the window is resized.
//! Nothing runs on a timer, so sitting on one scale costs nothing.

mod ladder;
mod paint;

use crust::style;
use crust::{seq, Crust, Cursor, Input, Pane};
use ladder::{HUMAN, RUNGS};
use std::io::Write;

const RUST_RGB: (u8, u8, u8) = (247, 76, 0);
const HEAD_RGB: (u8, u8, u8) = (247, 140, 60);
const BAR_BG: (u8, u8, u8) = (38, 38, 38);
const DIM_RGB: (u8, u8, u8) = (140, 140, 150);
const MARK_RGB: (u8, u8, u8) = (255, 200, 90);

/// The ruler runs over these powers of ten.
const LOW: f64 = -35.0;
const HIGH: f64 = 27.0;

struct App {
    step: usize,
    pixels: Option<glow::Display>,
}

fn main() {
    let mut start: Option<usize> = None;
    for a in std::env::args().skip(1) {
        match a.as_str() {
            "-h" | "--help" => {
                println!("universe — every scale of the universe (Fe2O3 suite)");
                println!();
                println!("Usage: universe [STEP]");
                println!();
                println!("  STEP   open on this rung, 1 to {}", RUNGS.len());
                println!("  -l     list every rung and its size");
                println!("  -v     print version");
                println!();
                println!("Keys: up and down step out and in, H returns to the human body,");
                println!("g and G jump to the ends, q quits. The rung is remembered.");
                return;
            }
            "-v" | "--version" => {
                println!("universe {}", env!("CARGO_PKG_VERSION"));
                return;
            }
            "--png" => {
                // A rung as a picture file, for the README and for
                // looking at a scene without a terminal in the way.
                let args: Vec<String> = std::env::args().skip(1).collect();
                let n: usize = args.get(1).and_then(|v| v.parse().ok()).unwrap_or(HUMAN + 1);
                let file = args.get(2).cloned().unwrap_or_else(|| "universe.png".into());
                let (cols, rows) = (args.get(3).and_then(|v| v.parse().ok()).unwrap_or(160u16),
                                    args.get(4).and_then(|v| v.parse().ok()).unwrap_or(44u16));
                let mut c = glow::Canvas::with_cell(cols, rows, (10, 20));
                ladder::paint(n.saturating_sub(1).min(RUNGS.len() - 1), &mut c);
                std::fs::write(&file, c.png()).expect("write png");
                println!("{} → {file}", RUNGS[n.saturating_sub(1).min(RUNGS.len() - 1)].name);
                return;
            }
            "-l" | "--list" => {
                for (i, r) in RUNGS.iter().enumerate() {
                    println!("{:>3}  {:<24} {}", i + 1, r.name, human(r.span));
                }
                return;
            }
            other => match other.parse::<usize>() {
                Ok(n) if n >= 1 && n <= RUNGS.len() => start = Some(n - 1),
                _ => {
                    eprintln!("universe: no rung {other}; try -l for the list");
                    std::process::exit(1);
                }
            },
        }
    }

    use std::io::IsTerminal;
    if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
        let i = start.unwrap_or(HUMAN);
        println!("{}  ·  {}", RUNGS[i].name, human(RUNGS[i].span));
        println!("{}", RUNGS[i].blurb);
        return;
    }

    let mut app = App {
        step: start.unwrap_or_else(|| remembered().unwrap_or(HUMAN)),
        pixels: None,
    };

    Crust::init();
    Crust::set_app_identity("Universe");
    let (mut cols, mut rows) = Crust::terminal_size();
    let mut status = Pane::new(1, rows, cols, 1, 250, 236);
    status.scroll = false;

    draw_all(&mut app, &mut status, cols, rows);

    loop {
        let key = match Input::getchr(None) {
            Some(k) => k,
            None => continue,
        };
        match key.as_str() {
            "q" | "ESC" => break,
            "UP" | "k" | "+" | "=" | "l" | "RIGHT" => {
                if app.step + 1 < RUNGS.len() {
                    app.step += 1;
                    remember(app.step);
                    draw_all(&mut app, &mut status, cols, rows);
                }
            }
            "DOWN" | "j" | "-" | "_" | "h" | "LEFT" => {
                if app.step > 0 {
                    app.step -= 1;
                    remember(app.step);
                    draw_all(&mut app, &mut status, cols, rows);
                }
            }
            "g" | "HOME" => {
                app.step = 0;
                remember(app.step);
                draw_all(&mut app, &mut status, cols, rows);
            }
            "G" | "END" => {
                app.step = RUNGS.len() - 1;
                remember(app.step);
                draw_all(&mut app, &mut status, cols, rows);
            }
            "H" => {
                app.step = HUMAN;
                remember(app.step);
                draw_all(&mut app, &mut status, cols, rows);
            }
            "PgUP" => {
                app.step = (app.step + 5).min(RUNGS.len() - 1);
                remember(app.step);
                draw_all(&mut app, &mut status, cols, rows);
            }
            "PgDOWN" => {
                app.step = app.step.saturating_sub(5);
                remember(app.step);
                draw_all(&mut app, &mut status, cols, rows);
            }
            "WINCH" => {
                let (nc, nr) = Crust::terminal_size();
                if (nc, nr) != (cols, rows) {
                    cols = nc;
                    rows = nr;
                    status = Pane::new(1, rows, cols, 1, 250, 236);
                    status.scroll = false;
                    if let Some(d) = app.pixels.as_mut() {
                        d.clear_all();
                    }
                    draw_all(&mut app, &mut status, cols, rows);
                }
            }
            _ => {}
        }
    }

    if let Some(d) = app.pixels.as_mut() {
        d.clear_all();
    }
    Crust::cleanup();
}

/// Where the app keeps the rung you left off on.
fn state_path() -> std::path::PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
    std::path::Path::new(&home).join(".universe")
}

fn remembered() -> Option<usize> {
    let text = std::fs::read_to_string(state_path()).ok()?;
    let n: usize = text.trim().parse().ok()?;
    if n < RUNGS.len() {
        Some(n)
    } else {
        None
    }
}

fn remember(step: usize) {
    let _ = std::fs::write(state_path(), format!("{step}\n"));
}

/// A length in the unit a person would use for it.
fn human(m: f64) -> String {
    const LY: f64 = 9.4607e15;
    const AU: f64 = 1.495_978_707e11;
    let (v, unit) = if m < 1e-12 {
        return format!("{:.0e} m", m).replace("e-", "e−");
    } else if m < 1e-9 {
        (m * 1e12, "pm")
    } else if m < 1e-6 {
        (m * 1e9, "nm")
    } else if m < 1e-3 {
        (m * 1e6, "µm")
    } else if m < 1.0 {
        (m * 1e3, "mm")
    } else if m < 1e3 {
        (m, "m")
    } else if m < AU * 0.5 {
        (m / 1e3, "km")
    } else if m < LY * 0.5 {
        (m / AU, "AU")
    } else if m < LY * 1e3 {
        (m / LY, "light years")
    } else if m < LY * 1e6 {
        (m / (LY * 1e3), "thousand light years")
    } else if m < LY * 1e9 {
        (m / (LY * 1e6), "million light years")
    } else {
        (m / (LY * 1e9), "billion light years")
    };
    if v >= 100.0 {
        format!("{v:.0} {unit}")
    } else if v >= 10.0 {
        format!("{v:.1} {unit}")
    } else {
        format!("{v:.2} {unit}")
    }
}

/// The power of ten, as a superscript, for the ends of the ruler.
fn power(n: i32) -> String {
    let digits = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
    let mut s = String::from("10");
    if n < 0 {
        s.push('⁻');
    }
    for ch in n.abs().to_string().chars() {
        s.push(digits[ch.to_digit(10).unwrap_or(0) as usize]);
    }
    s
}

fn move_to(row: u16, col: u16) -> String {
    Cursor::at(col, row)
}

fn draw_all(app: &mut App, status: &mut Pane, cols: u16, rows: u16) {
    if rows < 8 || cols < 30 {
        Cursor::clear_screen_down();
        print!("{}", move_to(1, 1));
        print!("{}", style::rgb("universe needs a larger window", Some(HEAD_RGB), None, ""));
        std::io::stdout().flush().ok();
        return;
    }
    draw_header(app, cols);
    draw_picture(app, cols, rows);
    draw_foot(app, cols, rows);
    status.say(&help_line());
    std::io::stdout().flush().ok();
}

fn draw_header(app: &App, cols: u16) {
    let r = &RUNGS[app.step];
    let info = format!(
        " {}  {}  {}",
        style::rgb("universe", Some(RUST_RGB), None, "b"),
        style::bold(r.name),
        style::rgb(&human(r.span), Some(HEAD_RGB), None, "")
    );
    let right = format!("{} of {} ", app.step + 1, RUNGS.len());
    let pad = (cols as usize)
        .saturating_sub(crust::display_width(&info))
        .saturating_sub(crust::display_width(&right));
    let armed = style::rgb("", None, Some(BAR_BG), "");
    let armed = armed.trim_end_matches(style::RESET);
    let line = info.replace(style::RESET, &format!("{}{}", style::RESET, armed));
    print!(
        "{}{}",
        move_to(1, 1),
        style::rgb(&format!("{line}{}{right}", " ".repeat(pad)), None, Some(BAR_BG), "")
    );
}

/// The picture area: rows 2 to rows-3.
fn picture_rows(rows: u16) -> u16 {
    rows.saturating_sub(4).max(1)
}

fn draw_picture(app: &mut App, cols: u16, rows: u16) {
    let ph = picture_rows(rows);
    let pixels = app.pixels.get_or_insert_with(glow::Display::new).supported();
    if pixels {
        let mut canvas = glow::Canvas::new(cols, ph);
        ladder::paint(app.step, &mut canvas);
        if let Some(d) = app.pixels.as_mut() {
            d.swap_canvas(&canvas, 1, 2);
        }
    } else {
        // Four pixels per cell, then averaged down to half-blocks: the
        // shapes keep their edges instead of turning into stairs.
        let mut canvas = glow::Canvas::with_cell(cols, ph, (2, 4));
        ladder::paint(app.step, &mut canvas);
        let lines = paint::half_blocks(&canvas, cols, ph);
        let mut s = String::new();
        for (i, line) in lines.iter().enumerate() {
            s.push_str(&move_to(2 + i as u16, 1));
            s.push_str(line);
        }
        print!("{s}");
    }
}

fn draw_foot(app: &App, cols: u16, rows: u16) {
    let r = &RUNGS[app.step];
    let w = cols as usize;
    // The one line about what you are looking at, centred under it.
    let blurb = fit(r.blurb, w.saturating_sub(2));
    let left = (w.saturating_sub(crust::display_width(&blurb))) / 2;
    print!(
        "{}{}{}",
        move_to(rows - 2, 1),
        " ".repeat(left),
        style::rgb(&blurb, Some(DIM_RGB), None, "i")
    );
    print!("{}", seq::ERASE_EOL);

    // The ruler: every scale there is, with a mark where you stand.
    let lo = power(LOW as i32);
    let hi = power(HIGH as i32);
    let lw = crust::display_width(&lo) + 1;
    let rw = crust::display_width(&hi) + 1;
    let bar = w.saturating_sub(lw + rw + 2).max(4);
    let t = ((r.span.log10() - LOW) / (HIGH - LOW)).clamp(0.0, 1.0);
    let at = ((t * (bar - 1) as f64).round() as usize).min(bar - 1);
    let mut track = String::new();
    for i in 0..bar {
        if i == at {
            track.push_str(&style::rgb("◆", Some(MARK_RGB), None, "b"));
        } else if i % 10 == 0 {
            track.push_str(&style::rgb("┼", Some((90, 90, 110)), None, ""));
        } else {
            track.push_str(&style::rgb("─", Some((60, 60, 78)), None, ""));
        }
    }
    print!(
        "{} {}{} {}",
        move_to(rows - 1, 1),
        style::rgb(&lo, Some((110, 110, 130)), None, ""),
        track,
        style::rgb(&hi, Some((110, 110, 130)), None, "")
    );
    print!("{}", seq::ERASE_EOL);
}

/// Cut a line to fit, with a full stop rather than a broken word.
fn fit(s: &str, w: usize) -> String {
    if crust::display_width(s) <= w {
        return s.to_string();
    }
    let mut out = String::new();
    for word in s.split_whitespace() {
        if crust::display_width(&out) + word.len() + 2 > w {
            break;
        }
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(word);
    }
    out.push('…');
    out
}

fn help_line() -> String {
    format!(
        " {}  {}  {}  {}  {}",
        style::rgb("↑↓", Some(HEAD_RGB), None, "b"),
        style::dim("out and in"),
        style::rgb("H", Some(HEAD_RGB), None, "b"),
        style::dim("human · g G ends · PgUp PgDn five at a time · q quits"),
        style::dim(&format!("v{}", env!("CARGO_PKG_VERSION")))
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_ladder_only_ever_grows() {
        for pair in RUNGS.windows(2) {
            assert!(pair[1].span > pair[0].span, "{} is not larger than {}", pair[1].name, pair[0].name);
        }
        assert_eq!(RUNGS[HUMAN].name, "Human body", "the app opens on a person");
    }

    #[test]
    fn every_rung_fits_on_the_ruler() {
        for r in RUNGS {
            let e = r.span.log10();
            assert!(e >= LOW && e <= HIGH, "{} sits off the ruler at 1e{e}", r.name);
        }
    }

    #[test]
    fn lengths_are_written_the_way_a_person_says_them() {
        assert_eq!(human(1.7), "1.70 m");
        assert_eq!(human(2.5e-3), "2.50 mm");
        assert_eq!(human(3e-10), "300 pm");
        assert_eq!(human(1.2e4), "12.0 km");
        assert!(human(6e11).ends_with("AU"), "an orbit is measured in AU");
        assert_eq!(human(5e17), "52.9 light years");
        assert_eq!(human(3e21), "317 thousand light years");
        assert!(human(8.8e26).contains("billion light years"));
    }

    #[test]
    fn powers_of_ten_read_as_superscripts() {
        assert_eq!(power(-35), "10⁻³⁵");
        assert_eq!(power(27), "10²⁷");
        assert_eq!(power(0), "10⁰");
    }

    #[test]
    fn every_rung_paints_something_other_than_the_background() {
        for i in 0..RUNGS.len() {
            let mut c = glow::Canvas::with_cell(40, 12, (4, 8));
            ladder::paint(i, &mut c);
            let lit = c.rgba.chunks(4).filter(|p| p[0] > 12 || p[1] > 12 || p[2] > 12).count();
            assert!(lit > 40, "rung {} ({}) drew almost nothing", i, RUNGS[i].name);
        }
    }
}
