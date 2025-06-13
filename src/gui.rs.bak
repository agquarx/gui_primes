// src/gui.rs
//! Functional-style GUI glue: pure compute + streaming + UI.

use std::{
    env, fs,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

use crate::primes::{cache_stats, compute_with_memo, PrimeType};
use egui::{self, ProgressBar};

pub struct MyApp {
    output: Arc<Mutex<String>>,
    prog: Arc<Mutex<f32>>,
    elapsed: Arc<Mutex<f32>>,

    calc_running: bool,
    started: Instant,
    should_stop: Arc<AtomicBool>,

    selected: PrimeType,
    range_start: u64,
    range_end: u64,
    last_span: u64,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            output: Arc::new(Mutex::new("Waiting…".into())),
            prog: Arc::new(Mutex::new(0.0)),
            elapsed: Arc::new(Mutex::new(0.0)),
            calc_running: false,
            started: Instant::now(),
            should_stop: Arc::new(AtomicBool::new(false)),
            selected: PrimeType::Mersenne,
            range_start: 2,
            range_end: 1_000,
            last_span: 1_000,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.calc_running {
            *self.elapsed.lock().unwrap() = self.started.elapsed().as_secs_f32();
        }

        let text = self.output.lock().unwrap().clone();
        let prog = *self.prog.lock().unwrap();
        let secs = *self.elapsed.lock().unwrap();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_sized(
                [ui.available_size().x, ui.available_size().y * 0.55],
                egui::TextEdit::multiline(&mut text.clone())
                    .font(egui::TextStyle::Monospace)
                    .interactive(false),
            );

            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.add(
                    ProgressBar::new(prog)
                        .desired_width(ui.available_width() * 0.75)
                        .show_percentage(),
                );
                ui.label(format!("{secs:.1} s"));
            });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label("Start:");
                ui.add(egui::Slider::new(&mut self.range_start, 2..=10_000_000).text("n"));
            });
            ui.horizontal(|ui| {
                ui.label("End:");
                ui.add(
                    egui::Slider::new(&mut self.range_end, (self.range_start + 1)..=10_000_000)
                        .text("n"),
                );
            });

            ui.label("Prime family:");
            let prev = self.selected;
            egui::ComboBox::from_label("family")
                .selected_text(format!("{:?}", self.selected))
                .show_ui(ui, |ui| {
                for &t in &[
                    PrimeType::Mersenne,
                    PrimeType::SophieGermain,
                    PrimeType::Twin,
                    PrimeType::Palindromic,
                    PrimeType::Sexy,
                    PrimeType::Cousin,
                    PrimeType::Emirp,
                    PrimeType::Safe,
                    PrimeType::Chen,
                    PrimeType::Circular,
                    PrimeType::Fermat,
                    PrimeType::Cuban,
                    PrimeType::Ebl,
                    PrimeType::Proth,
                    PrimeType::Cullen,
                    PrimeType::Woodall,
                    PrimeType::Thabit,
                    PrimeType::Euclid,
                    PrimeType::Fibonacci,
                    PrimeType::Perrin,
                    PrimeType::Happy,
                    PrimeType::Wilson,
                    PrimeType::CenteredHex,
                ] {
                    if ui
                        .selectable_label(self.selected == t, format!("{t:?}"))
                        .clicked()
                    {
                        self.selected = t;
                    }
                }
            });

            if self.selected != prev {
                *self.output.lock().unwrap() = String::new();
                *self.prog.lock().unwrap() = 0.0;
                *self.elapsed.lock().unwrap() = 0.0;
                self.started = Instant::now();
            }

            ui.add_space(6.0);
            ui.horizontal(|ui| {
                if ui.button("Calculate").clicked() && !self.calc_running {
                    self.launch(ctx);
                }
                if ui.button("Stop").clicked() {
                    self.should_stop.store(true, Ordering::SeqCst);
                }
                if ui.button("Dump").clicked() {
                    let home = env::var("HOME").unwrap_or_default();
                    let path = Path::new(&home).join(".dump");
                    let _ = fs::write(path, &*self.output.lock().unwrap());
                }
                if ui.button("Suck").clicked() && !self.calc_running {
                    let home = env::var("HOME").unwrap_or_default();
                    let path = Path::new(&home).join(".dump");
                    if let Ok(s) = fs::read_to_string(path) {
                        if let Some(last) =
                            s.split(',').filter_map(|x| x.trim().parse::<u64>().ok()).next_back()
                        {
                            self.range_start = last + 1;
                            self.range_end = self.range_start + self.last_span.max(1_000);
                        }
                    }
                    self.launch(ctx);
                }
                if ui.button("Cache").clicked() {
                    println!("cache entries: {}", cache_stats());
                }
                if ui.button("Exit").clicked() {
                    std::process::exit(0);
                }
            });
        });

        if self.calc_running && prog >= 1.0 {
            self.calc_running = false;
        }

        ctx.request_repaint_after(Duration::from_millis(100));
    }
}

impl MyApp {
    fn launch(&mut self, ctx: &egui::Context) {
        self.calc_running = true;
        self.should_stop.store(false, Ordering::SeqCst);
        self.started = Instant::now();
        self.last_span = self.range_end - self.range_start;
        *self.prog.lock().unwrap() = 0.0;
        *self.elapsed.lock().unwrap() = 0.0;
        *self.output.lock().unwrap() = String::new();

        let out = self.output.clone();
        let pr = self.prog.clone();
        let stop = self.should_stop.clone();
        let fam = self.selected;
        let a = self.range_start;
        let b = self.range_end;
        let ctx2 = ctx.clone();

        thread::spawn(move || {
            let v = compute_with_memo(fam, a, b);
            
            // If no primes found, just set progress to 1.0
            if v.is_empty() {
                *pr.lock().unwrap() = 1.0;
                ctx2.request_repaint();
                return;
            }
            
            for (i, s) in v.iter().enumerate() {
                if stop.load(Ordering::SeqCst) {
                    break;
                }
                
                // Add the prime to the output
                {
                    let mut o = out.lock().unwrap();
                    if !o.is_empty() {
                        o.push_str(", ");
                    }
                    o.push_str(s);
                }
                
                // Create smoother progress updates between this prime and the next
                let current_progress = (i as f32) / (v.len() as f32);
                let next_progress = ((i + 1) as f32) / (v.len() as f32);
                
                // Number of intermediate steps (more steps = smoother progress)
                let steps = 10;
                
                for step in 0..=steps {
                    // Calculate intermediate progress between current and next
                    let progress = current_progress + (next_progress - current_progress) * (step as f32 / steps as f32);
                    *pr.lock().unwrap() = progress;
                    ctx2.request_repaint();
                    
                    // Much shorter sleep time for smoother updates
                    thread::sleep(Duration::from_millis(20));
                    
                    if stop.load(Ordering::SeqCst) {
                        break;
                    }
                }
            }
            *pr.lock().unwrap() = 1.0;
            ctx2.request_repaint();
        });
    }
}

pub fn run_gui() -> eframe::Result<()> {
    eframe::run_native(
        "GUI Primes",
        eframe::NativeOptions::default(),
        Box::new(|_| Box::new(MyApp::default())),
    )
}
