// ukulele_chords.java — Java версия

import java.util.*;

public class ukulele_chords {
    private String tuning;
    private String[] strings;
    private Map<String, Map<String, Map<Integer, Integer>>> chords;

    public ukulele_chords(String tuning) {
        this.tuning = tuning;
        this.strings = tuning.equals("GCEA") ? new String[]{"G", "C", "E", "A"} :
                       tuning.equals("ADF#B") ? new String[]{"A", "D", "F#", "B"} :
                       new String[]{"D", "G", "B", "E"};
        this.chords = new LinkedHashMap<>();
        initChords();
    }

    private void initChords() {
        Map<String, Map<Integer, Integer>> c = new LinkedHashMap<>();
        c.put("major", new HashMap<Integer, Integer>() {{ put(3, 0); put(2, 0); put(1, 0); put(0, 3); }});
        c.put("minor", new HashMap<Integer, Integer>() {{ put(3, 0); put(2, 3); put(1, 3); put(0, 3); }});
        c.put("seventh", new HashMap<Integer, Integer>() {{ put(3, 0); put(2, 0); put(1, 0); put(0, 1); }});
        chords.put("C", c);

        Map<String, Map<Integer, Integer>> d = new LinkedHashMap<>();
        d.put("major", new HashMap<Integer, Integer>() {{ put(3, 2); put(2, 2); put(1, 2); put(0, 0); }});
        d.put("minor", new HashMap<Integer, Integer>() {{ put(3, 2); put(2, 1); put(1, 2); put(0, 0); }});
        d.put("seventh", new HashMap<Integer, Integer>() {{ put(3, 2); put(2, 1); put(1, 2); put(0, 0); }});
        chords.put("D", d);

        Map<String, Map<Integer, Integer>> e = new LinkedHashMap<>();
        e.put("major", new HashMap<Integer, Integer>() {{ put(3, 1); put(2, 4); put(1, 0); put(0, 4); }});
        e.put("minor", new HashMap<Integer, Integer>() {{ put(3, 0); put(2, 4); put(1, 3); put(0, 2); }});
        e.put("seventh", new HashMap<Integer, Integer>() {{ put(3, 0); put(2, 0); put(1, 0); put(0, 4); }});
        chords.put("E", e);

        Map<String, Map<Integer, Integer>> f = new LinkedHashMap<>();
        f.put("major", new HashMap<Integer, Integer>() {{ put(3, 2); put(2, 0); put(1, 1); put(0, 0); }});
        f.put("minor", new HashMap<Integer, Integer>() {{ put(3, 1); put(2, 0); put(1, 1); put(0, 3); }});
        f.put("seventh", new HashMap<Integer, Integer>() {{ put(3, 2); put(2, 0); put(1, 1); put(0, 3); }});
        chords.put("F", f);

        Map<String, Map<Integer, Integer>> g = new LinkedHashMap<>();
        g.put("major", new HashMap<Integer, Integer>() {{ put(3, 0); put(2, 2); put(1, 3); put(0, 2); }});
        g.put("minor", new HashMap<Integer, Integer>() {{ put(3, 0); put(2, 2); put(1, 3); put(0, 3); }});
        g.put("seventh", new HashMap<Integer, Integer>() {{ put(3, 0); put(2, 2); put(1, 3); put(0, 2); }});
        chords.put("G", g);

        Map<String, Map<Integer, Integer>> a = new LinkedHashMap<>();
        a.put("major", new HashMap<Integer, Integer>() {{ put(3, 0); put(2, 1); put(1, 2); put(0, 0); }});
        a.put("minor", new HashMap<Integer, Integer>() {{ put(3, 0); put(2, 0); put(1, 2); put(0, 2); }});
        a.put("seventh", new HashMap<Integer, Integer>() {{ put(3, 0); put(2, 0); put(1, 2); put(0, 1); }});
        chords.put("A", a);

        Map<String, Map<Integer, Integer>> b = new LinkedHashMap<>();
        b.put("major", new HashMap<Integer, Integer>() {{ put(3, 4); put(2, 3); put(1, 2); put(0, 0); }});
        b.put("minor", new HashMap<Integer, Integer>() {{ put(3, 4); put(2, 2); put(1, 2); put(0, 2); }});
        b.put("seventh", new HashMap<Integer, Integer>() {{ put(3, 4); put(2, 1); put(1, 2); put(0, 2); }});
        chords.put("B", b);
    }

    public Map<Integer, Integer> getChord(String root, String type) {
        if (chords.containsKey(root) && chords.get(root).containsKey(type)) {
            return chords.get(root).get(type);
        }
        return null;
    }

    public void printFretboard(Map<Integer, Integer> pos) {
        System.out.println("\n   " + String.join(" ", strings));
        System.out.println("   " + "─".repeat(strings.length * 2 + 1));
        for (int fret = 0; fret < 5; fret++) {
            String line = fret == 0 ? "  " : String.valueOf(fret);
            line += " |";
            for (int str = 0; str < 4; str++) {
                if (pos.containsKey(str) && pos.get(str) == fret) {
                    line += " ● |";
                } else if (pos.containsKey(str) && pos.get(str) < fret) {
                    line += "   |";
                } else {
                    line += "   |";
                }
            }
            System.out.println(line);
        }
        System.out.println("\nАппликатура:");
        for (int str = 0; str < 4; str++) {
            if (pos.containsKey(str)) {
                if (pos.get(str) == 0) {
                    System.out.printf("  %s: открытая\n", strings[str]);
                } else {
                    System.out.printf("  %s: %d-й лад\n", strings[str], pos.get(str));
                }
            }
        }
    }

    public void listChords() {
        System.out.println("Доступные аккорды для укулеле:");
        for (String root : new TreeSet<>(chords.keySet())) {
            Set<String> types = chords.get(root).keySet();
            System.out.printf("  %s: %s\n", root, String.join(", ", types));
        }
    }

    public static void main(String[] args) {
        String chord = "C", type = "major", tuning = "GCEA";
        boolean list = false;

        for (int i = 0; i < args.length; i++) {
            if (args[i].equals("--chord")) chord = args[++i];
            else if (args[i].equals("--type")) type = args[++i];
            else if (args[i].equals("--tuning")) tuning = args[++i];
            else if (args[i].equals("--list")) list = true;
        }

        ukulele_chords generator = new ukulele_chords(tuning);

        if (list) {
            generator.listChords();
            return;
        }

        Map<Integer, Integer> pos = generator.getChord(chord, type);
        if (pos == null) {
            System.out.printf("❌ Аккорд %s %s не найден.\n", chord, type);
            System.out.println("Используйте --list для просмотра всех доступных аккордов.");
            return;
        }

        System.out.printf("\n🎸 Аккорд: %s (%s) | Настройка: %s\n", chord, type, tuning);
        generator.printFretboard(pos);
    }
}
