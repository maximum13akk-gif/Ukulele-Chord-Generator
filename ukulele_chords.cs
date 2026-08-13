// ukulele_chords.cs — C# версия

using System;
using System.Collections.Generic;
using System.Linq;

class UkuleleChord {
    private string tuning;
    private string[] strings;
    private Dictionary<string, Dictionary<string, Dictionary<int, int>>> chords;

    public UkuleleChord(string tuning) {
        this.tuning = tuning;
        this.strings = tuning == "GCEA" ? new[] { "G", "C", "E", "A" } :
                       tuning == "ADF#B" ? new[] { "A", "D", "F#", "B" } :
                       new[] { "D", "G", "B", "E" };
        this.chords = new Dictionary<string, Dictionary<string, Dictionary<int, int>>>();
        InitChords();
    }

    private void InitChords() {
        chords["C"] = new Dictionary<string, Dictionary<int, int>> {
            ["major"] = new Dictionary<int, int> { {3, 0}, {2, 0}, {1, 0}, {0, 3} },
            ["minor"] = new Dictionary<int, int> { {3, 0}, {2, 3}, {1, 3}, {0, 3} },
            ["seventh"] = new Dictionary<int, int> { {3, 0}, {2, 0}, {1, 0}, {0, 1} }
        };
        chords["D"] = new Dictionary<string, Dictionary<int, int>> {
            ["major"] = new Dictionary<int, int> { {3, 2}, {2, 2}, {1, 2}, {0, 0} },
            ["minor"] = new Dictionary<int, int> { {3, 2}, {2, 1}, {1, 2}, {0, 0} },
            ["seventh"] = new Dictionary<int, int> { {3, 2}, {2, 1}, {1, 2}, {0, 0} }
        };
        chords["E"] = new Dictionary<string, Dictionary<int, int>> {
            ["major"] = new Dictionary<int, int> { {3, 1}, {2, 4}, {1, 0}, {0, 4} },
            ["minor"] = new Dictionary<int, int> { {3, 0}, {2, 4}, {1, 3}, {0, 2} },
            ["seventh"] = new Dictionary<int, int> { {3, 0}, {2, 0}, {1, 0}, {0, 4} }
        };
        chords["F"] = new Dictionary<string, Dictionary<int, int>> {
            ["major"] = new Dictionary<int, int> { {3, 2}, {2, 0}, {1, 1}, {0, 0} },
            ["minor"] = new Dictionary<int, int> { {3, 1}, {2, 0}, {1, 1}, {0, 3} },
            ["seventh"] = new Dictionary<int, int> { {3, 2}, {2, 0}, {1, 1}, {0, 3} }
        };
        chords["G"] = new Dictionary<string, Dictionary<int, int>> {
            ["major"] = new Dictionary<int, int> { {3, 0}, {2, 2}, {1, 3}, {0, 2} },
            ["minor"] = new Dictionary<int, int> { {3, 0}, {2, 2}, {1, 3}, {0, 3} },
            ["seventh"] = new Dictionary<int, int> { {3, 0}, {2, 2}, {1, 3}, {0, 2} }
        };
        chords["A"] = new Dictionary<string, Dictionary<int, int>> {
            ["major"] = new Dictionary<int, int> { {3, 0}, {2, 1}, {1, 2}, {0, 0} },
            ["minor"] = new Dictionary<int, int> { {3, 0}, {2, 0}, {1, 2}, {0, 2} },
            ["seventh"] = new Dictionary<int, int> { {3, 0}, {2, 0}, {1, 2}, {0, 1} }
        };
        chords["B"] = new Dictionary<string, Dictionary<int, int>> {
            ["major"] = new Dictionary<int, int> { {3, 4}, {2, 3}, {1, 2}, {0, 0} },
            ["minor"] = new Dictionary<int, int> { {3, 4}, {2, 2}, {1, 2}, {0, 2} },
            ["seventh"] = new Dictionary<int, int> { {3, 4}, {2, 1}, {1, 2}, {0, 2} }
        };
    }

    public Dictionary<int, int> GetChord(string root, string type) {
        if (chords.ContainsKey(root) && chords[root].ContainsKey(type)) {
            return chords[root][type];
        }
        return null;
    }

    public void PrintFretboard(Dictionary<int, int> pos) {
        Console.WriteLine($"\n   {string.Join(" ", strings)}");
        Console.WriteLine($"   {new string('─', strings.Length * 2 + 1)}");
        for (int fret = 0; fret < 5; fret++) {
            string line = fret == 0 ? "  " : $"{fret} ";
            line += " |";
            for (int str = 0; str < 4; str++) {
                if (pos.ContainsKey(str) && pos[str] == fret) {
                    line += " ● |";
                } else if (pos.ContainsKey(str) && pos[str] < fret) {
                    line += "   |";
                } else {
                    line += "   |";
                }
            }
            Console.WriteLine(line);
        }
        Console.WriteLine("\nАппликатура:");
        for (int str = 0; str < 4; str++) {
            if (pos.ContainsKey(str)) {
                if (pos[str] == 0) {
                    Console.WriteLine($"  {strings[str]}: открытая");
                } else {
                    Console.WriteLine($"  {strings[str]}: {pos[str]}-й лад");
                }
            }
        }
    }

    public void ListChords() {
        Console.WriteLine("Доступные аккорды для укулеле:");
        foreach (var root in chords.Keys.OrderBy(k => k)) {
            var types = string.Join(", ", chords[root].Keys);
            Console.WriteLine($"  {root}: {types}");
        }
    }

    public static void Main(string[] args) {
        string chord = "C", type = "major", tuning = "GCEA";
        bool list = false;

        for (int i = 0; i < args.Length; i++) {
            if (args[i] == "--chord") chord = args[++i];
            else if (args[i] == "--type") type = args[++i];
            else if (args[i] == "--tuning") tuning = args[++i];
            else if (args[i] == "--list") list = true;
        }

        var generator = new UkuleleChord(tuning);

        if (list) {
            generator.ListChords();
            return;
        }

        var pos = generator.GetChord(chord, type);
        if (pos == null) {
            Console.WriteLine($"❌ Аккорд {chord} {type} не найден.");
            Console.WriteLine("Используйте --list для просмотра всех доступных аккордов.");
            return;
        }

        Console.WriteLine($"\n🎸 Аккорд: {chord} ({type}) | Настройка: {tuning}");
        generator.PrintFretboard(pos);
    }
}
