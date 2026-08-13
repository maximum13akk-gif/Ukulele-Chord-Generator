

### 1. `ukulele_chords.py` (Python)

```python
# ukulele_chords.py — Python версия

import sys
import argparse
from colorama import init, Fore, Style

init(autoreset=True)

class UkuleleChord:
    def __init__(self, tuning='GCEA'):
        self.tuning = tuning
        self.strings = ['G', 'C', 'E', 'A'] if tuning == 'GCEA' else ['A', 'D', 'F#', 'B'] if tuning == 'ADF#B' else ['D', 'G', 'B', 'E']
        self.chords = self._init_chords()

    def _init_chords(self):
        # Аккорды для стандартной настройки GCEA
        # Формат: {струна: лад} (0 - открытая, None - не играется)
        return {
            'C': {'major': {3: 0, 2: 0, 1: 0, 0: 3}, 'minor': {3: 0, 2: 3, 1: 3, 0: 3}, 'seventh': {3: 0, 2: 0, 1: 0, 0: 1}},
            'D': {'major': {3: 2, 2: 2, 1: 2, 0: 0}, 'minor': {3: 2, 2: 1, 1: 2, 0: 0}, 'seventh': {3: 2, 2: 1, 1: 2, 0: 0}},
            'E': {'major': {3: 1, 2: 4, 1: 0, 0: 4}, 'minor': {3: 0, 2: 4, 1: 3, 0: 2}, 'seventh': {3: 0, 2: 0, 1: 0, 0: 4}},
            'F': {'major': {3: 2, 2: 0, 1: 1, 0: 0}, 'minor': {3: 1, 2: 0, 1: 1, 0: 3}, 'seventh': {3: 2, 2: 0, 1: 1, 0: 3}},
            'G': {'major': {3: 0, 2: 2, 1: 3, 0: 2}, 'minor': {3: 0, 2: 2, 1: 3, 0: 3}, 'seventh': {3: 0, 2: 2, 1: 3, 0: 2}},
            'A': {'major': {3: 0, 2: 1, 1: 2, 0: 0}, 'minor': {3: 0, 2: 0, 1: 2, 0: 2}, 'seventh': {3: 0, 2: 0, 1: 2, 0: 1}},
            'B': {'major': {3: 4, 2: 3, 1: 2, 0: 0}, 'minor': {3: 4, 2: 2, 1: 2, 0: 2}, 'seventh': {3: 4, 2: 1, 1: 2, 0: 2}},
        }

    def get_chord(self, root, chord_type):
        if root in self.chords and chord_type in self.chords[root]:
            return self.chords[root][chord_type]
        return None

    def print_fretboard(self, pos):
        """Печать грифа укулеле (4 струны, 4 лада)."""
        strings = self.strings
        print("\n" + Fore.CYAN + f"   {' '.join(strings)}")
        print("   " + "─" * (len(strings) * 2 + 1))

        for fret in range(5):
            line = f"{fret if fret > 0 else ' '} |"
            for str_idx in range(4):
                if str_idx in pos and pos[str_idx] == fret:
                    line += " ● |"
                elif str_idx in pos and pos[str_idx] is not None and pos[str_idx] < fret:
                    line += "   |"
                else:
                    line += "   |"
            print(line)

        print("\n" + Fore.YELLOW + "Аппликатура:")
        for str_idx in range(4):
            if str_idx in pos:
                if pos[str_idx] == 0:
                    print(f"  {strings[str_idx]}: открытая")
                else:
                    print(f"  {strings[str_idx]}: {pos[str_idx]}-й лад")

    def list_chords(self):
        print(Fore.CYAN + "Доступные аккорды для укулеле:")
        for root in sorted(self.chords.keys()):
            types = list(self.chords[root].keys())
            print(f"  {root}: {', '.join(types)}")

def main():
    parser = argparse.ArgumentParser(description='Ukulele Chord Generator')
    parser.add_argument('--chord', default='C', help='Корень аккорда (C, D, E, F, G, A, B)')
    parser.add_argument('--type', default='major', help='Тип аккорда (major, minor, seventh)')
    parser.add_argument('--tuning', default='GCEA', choices=['GCEA', 'ADF#B', 'DGBE'], help='Настройка укулеле')
    parser.add_argument('--list', action='store_true', help='Показать все доступные аккорды')
    args = parser.parse_args()

    generator = UkuleleChord(args.tuning)

    if args.list:
        generator.list_chords()
        sys.exit(0)

    pos = generator.get_chord(args.chord, args.type)
    if pos is None:
        print(Fore.RED + f"❌ Аккорд {args.chord} {args.type} не найден.")
        print("Используйте --list для просмотра всех доступных аккордов.")
        sys.exit(1)

    print(f"\n{Fore.GREEN}🎸 Аккорд: {args.chord} ({args.type}) | Настройка: {args.tuning}")
    generator.print_fretboard(pos)

if __name__ == "__main__":
    main()
