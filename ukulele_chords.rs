// ukulele_chords.rs — Rust версия

use std::collections::HashMap;

struct UkuleleChord {
    tuning: String,
    strings: Vec<String>,
    chords: HashMap<String, HashMap<String, HashMap<usize, usize>>>,
}

impl UkuleleChord {
    fn new(tuning: &str) -> Self {
        let strings = match tuning {
            "GCEA" => vec!["G".to_string(), "C".to_string(), "E".to_string(), "A".to_string()],
            "ADF#B" => vec!["A".to_string(), "D".to_string(), "F#".to_string(), "B".to_string()],
            _ => vec!["D".to_string(), "G".to_string(), "B".to_string(), "E".to_string()],
        };
        let mut chord = UkuleleChord {
            tuning: tuning.to_string(),
            strings,
            chords: HashMap::new(),
        };
        chord.init_chords();
        chord
    }

    fn init_chords(&mut self) {
        let mut c = HashMap::new();
        c.insert("major".to_string(), {
            let mut m = HashMap::new();
            m.insert(3, 0); m.insert(2, 0); m.insert(1, 0); m.insert(0, 3);
            m
        });
        c.insert("minor".to_string(), {
            let mut m = HashMap::new();
            m.insert(3, 0); m.insert(2, 3); m.insert(1, 3); m.insert(0, 3);
            m
        });
        c.insert("seventh".to_string(), {
            let mut m = HashMap::new();
            m.insert(3, 0); m.insert(2, 0); m.insert(1, 0); m.insert(0, 1);
            m
        });
        self.chords.insert("C".to_string(), c);

        let mut d = HashMap::new();
        d.insert("major".to_string(), {
            let mut m = HashMap::new();
            m.insert(3, 2); m.insert(2, 2); m.insert(1, 2); m.insert(0, 0);
            m
        });
        d.insert("minor".to_string(), {
            let mut m = HashMap::new();
            m.insert(3, 2); m.insert(2, 1); m.insert(1, 2); m.insert(0, 0);
            m
        });
        d.insert("seventh".to_string(), {
            let mut m = HashMap::new();
            m.insert(3, 2); m.insert(2, 1); m.insert(1, 2); m.insert(0, 0);
            m
        });
        self.chords.insert("D".to_string(), d);

        let mut e = HashMap::new();
        e.insert("major".to_string(), {
            let mut m = HashMap::new();
            m.insert(3, 1); m.insert(2, 4); m.insert(1, 0); m.insert(0, 4);
            m
        });
        e.insert("minor".to_string(), {
            let mut m = HashMap::new();
            m.insert(3, 0); m.insert(2, 4); m.insert(1, 3); m.insert(0, 2);
            m
        });
        e.insert("seventh".to_string(), {
            let mut m = HashMap::new();
            m.insert(3, 0); m.insert(2, 0); m.insert(1, 0); m.insert(0, 4);
            m
        });
        self.chords.insert("E".to_string(), e);

        let mut f = HashMap::new();
        f.insert("major".to_string(), {
            let mut m = HashMap::new();
            m.insert(3, 2); m.insert(2, 0); m.insert(1, 1); m.insert(0, 0);
            m
        });
        f.insert("minor".to_string(), {
            let mut m = HashMap::new();
            m.insert(3, 1); m.insert(2, 0); m.insert(1, 1); m.insert(0, 3);
            m
        });
        f.insert("seventh".to_string(), {
            let mut m = HashMap::new();
            m.insert(3, 2); m.insert(2, 0); m.insert(1, 1); m.insert(0, 3);
            m
        });
        self.chords.insert("F".to_string(), f);

        let mut g = HashMap::new();
        g.insert("major".to_string(), {
            let mut m = HashMap::new();
            m.insert(3, 0); m.insert(2, 2); m.insert(1, 3); m.insert(0, 2);
            m
        });
        g.insert("minor".to_string(), {
            let mut m = HashMap::new();
            m.insert(3, 0); m.insert(2, 2); m.insert(1, 3); m.insert(0, 3);
            m
        });
        g.insert("seventh".to_string(), {
            let mut m = HashMap::new();
            m.insert(3, 0); m.insert(2, 2); m.insert(1, 3); m.insert(0, 2);
            m
        });
        self.chords.insert("G".to_string(), g);

        let mut a = HashMap::new();
        a.insert("major".to_string(), {
            let mut m = HashMap::new();
            m.insert(3, 0); m.insert(2, 1); m.insert(1, 2); m.insert(0, 0);
            m
        });
        a.insert("minor".to_string(), {
            let mut m = HashMap::new();
            m.insert(3, 0); m.insert(2, 0); m.insert(1, 2); m.insert(0, 2);
            m
        });
        a.insert("seventh".to_string(), {
            let mut m = HashMap::new();
            m.insert(3, 0); m.insert(2, 0); m.insert(1, 2); m.insert(0, 1);
            m
        });
        self.chords.insert("A".to_string(), a);

        let mut b = HashMap::new();
        b.insert("major".to_string(), {
            let mut m = HashMap::new();
            m.insert(3, 4); m.insert(2, 3); m.insert(1, 2); m.insert(0, 0);
            m
        });
        b.insert("minor".to_string(), {
            let mut m = HashMap::new();
            m.insert(3, 4); m.insert(2, 2); m.insert(1, 2); m.insert(0, 2);
            m
        });
        b.insert("seventh".to_string(), {
            let mut m = HashMap::new();
            m.insert(3, 4); m.insert(2, 1); m.insert(1, 2); m.insert(0, 2);
            m
        });
        self.chords.insert("B".to_string(), b);
    }

    fn get_chord(&self, root: &str, chord_type: &str) -> Option<HashMap<usize, usize>> {
        if let Some(root_map) = self.chords.get(root) {
            if let Some(pos) = root_map.get(chord_type) {
                return Some(pos.clone());
            }
        }
        None
    }

    fn print_fretboard(&self, pos: &HashMap<usize, usize>) {
        println!("\n   {}", self.strings.join(" "));
        println!("   {}", "─".repeat(self.strings.len() * 2 + 1));
        for fret in 0..5 {
            let mut line = if fret == 0 { "  ".to_string() } else { format!("{} ", fret) };
            line.push_str(" |");
            for str in 0..4 {
                if let Some(&val) = pos.get(&str) {
                    if val == fret {
                        line.push_str(" ● |");
                    } else if val < fret {
                        line.push_str("   |");
                    } else {
                        line.push_str("   |");
                    }
                } else {
                    line.push_str("   |");
                }
            }
            println!("{}", line);
        }
        println!("\nАппликатура:");
        for str in 0..4 {
            if let Some(&val) = pos.get(&str) {
                if val == 0 {
                    println!("  {}: открытая", self.strings[str]);
                } else {
                    println!("  {}: {}-й лад", self.strings[str], val);
                }
            }
        }
    }

    fn list_chords(&self) {
        println!("Доступные аккорды для укулеле:");
        let mut roots: Vec<_> = self.chords.keys().collect();
        roots.sort();
        for root in roots {
            let types: Vec<_> = self.chords[root].keys().collect();
            println!("  {}: {}", root, types.join(", "));
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut chord = "C".to_string();
    let mut ctype = "major".to_string();
    let mut tuning = "GCEA".to_string();
    let mut list = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--chord" => { chord = args[i+1].clone(); i += 2; }
            "--type" => { ctype = args[i+1].clone(); i += 2; }
            "--tuning" => { tuning = args[i+1].clone(); i += 2; }
            "--list" => { list = true; i += 1; }
            _ => { i += 1; }
        }
    }

    let generator = UkuleleChord::new(&tuning);

    if list {
        generator.list_chords();
        return;
    }

    if let Some(pos) = generator.get_chord(&chord, &ctype) {
        println!("\n🎸 Аккорд: {} ({}) | Настройка: {}", chord, ctype, tuning);
        generator.print_fretboard(&pos);
    } else {
        println!("❌ Аккорд {} {} не найден.", chord, ctype);
        println!("Используйте --list для просмотра всех доступных аккордов.");
    }
}
