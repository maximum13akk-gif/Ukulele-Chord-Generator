// ukulele_chords.go — Go версия

package main

import (
	"flag"
	"fmt"
	"strings"
)

type UkuleleChord struct {
	Tuning  string
	Strings []string
	Chords  map[string]map[string]map[int]int
}

func NewUkuleleChord(tuning string) *UkuleleChord {
	uc := &UkuleleChord{
		Tuning: tuning,
		Chords: make(map[string]map[string]map[int]int),
	}
	uc.initStrings()
	uc.initChords()
	return uc
}

func (uc *UkuleleChord) initStrings() {
	switch uc.Tuning {
	case "GCEA":
		uc.Strings = []string{"G", "C", "E", "A"}
	case "ADF#B":
		uc.Strings = []string{"A", "D", "F#", "B"}
	default:
		uc.Strings = []string{"D", "G", "B", "E"}
	}
}

func (uc *UkuleleChord) initChords() {
	uc.Chords["C"] = map[string]map[int]int{
		"major":   {3: 0, 2: 0, 1: 0, 0: 3},
		"minor":   {3: 0, 2: 3, 1: 3, 0: 3},
		"seventh": {3: 0, 2: 0, 1: 0, 0: 1},
	}
	uc.Chords["D"] = map[string]map[int]int{
		"major":   {3: 2, 2: 2, 1: 2, 0: 0},
		"minor":   {3: 2, 2: 1, 1: 2, 0: 0},
		"seventh": {3: 2, 2: 1, 1: 2, 0: 0},
	}
	uc.Chords["E"] = map[string]map[int]int{
		"major":   {3: 1, 2: 4, 1: 0, 0: 4},
		"minor":   {3: 0, 2: 4, 1: 3, 0: 2},
		"seventh": {3: 0, 2: 0, 1: 0, 0: 4},
	}
	uc.Chords["F"] = map[string]map[int]int{
		"major":   {3: 2, 2: 0, 1: 1, 0: 0},
		"minor":   {3: 1, 2: 0, 1: 1, 0: 3},
		"seventh": {3: 2, 2: 0, 1: 1, 0: 3},
	}
	uc.Chords["G"] = map[string]map[int]int{
		"major":   {3: 0, 2: 2, 1: 3, 0: 2},
		"minor":   {3: 0, 2: 2, 1: 3, 0: 3},
		"seventh": {3: 0, 2: 2, 1: 3, 0: 2},
	}
	uc.Chords["A"] = map[string]map[int]int{
		"major":   {3: 0, 2: 1, 1: 2, 0: 0},
		"minor":   {3: 0, 2: 0, 1: 2, 0: 2},
		"seventh": {3: 0, 2: 0, 1: 2, 0: 1},
	}
	uc.Chords["B"] = map[string]map[int]int{
		"major":   {3: 4, 2: 3, 1: 2, 0: 0},
		"minor":   {3: 4, 2: 2, 1: 2, 0: 2},
		"seventh": {3: 4, 2: 1, 1: 2, 0: 2},
	}
}

func (uc *UkuleleChord) getChord(root, chordType string) map[int]int {
	if chordMap, ok := uc.Chords[root]; ok {
		if pos, ok := chordMap[chordType]; ok {
			return pos
		}
	}
	return nil
}

func (uc *UkuleleChord) printFretboard(pos map[int]int) {
	fmt.Println("\n   " + strings.Join(uc.Strings, " "))
	fmt.Println("   " + strings.Repeat("─", len(uc.Strings)*2+1))
	for fret := 0; fret < 5; fret++ {
		line := ""
		if fret == 0 {
			line = "  "
		} else {
			line = fmt.Sprintf("%d ", fret)
		}
		line += " |"
		for str := 0; str < 4; str++ {
			if val, ok := pos[str]; ok && val == fret {
				line += " ● |"
			} else if val, ok := pos[str]; ok && val < fret {
				line += "   |"
			} else {
				line += "   |"
			}
		}
		fmt.Println(line)
	}
	fmt.Println("\nАппликатура:")
	for str := 0; str < 4; str++ {
		if val, ok := pos[str]; ok {
			if val == 0 {
				fmt.Printf("  %s: открытая\n", uc.Strings[str])
			} else {
				fmt.Printf("  %s: %d-й лад\n", uc.Strings[str], val)
			}
		}
	}
}

func (uc *UkuleleChord) listChords() {
	fmt.Println("Доступные аккорды для укулеле:")
	for root := range uc.Chords {
		types := []string{}
		for t := range uc.Chords[root] {
			types = append(types, t)
		}
		fmt.Printf("  %s: %s\n", root, strings.Join(types, ", "))
	}
}

func main() {
	chord := flag.String("chord", "C", "Корень аккорда")
	ctype := flag.String("type", "major", "Тип аккорда")
	tuning := flag.String("tuning", "GCEA", "Настройка (GCEA, ADF#B, DGBE)")
	list := flag.Bool("list", false, "Показать все аккорды")
	flag.Parse()

	generator := NewUkuleleChord(*tuning)

	if *list {
		generator.listChords()
		return
	}

	pos := generator.getChord(*chord, *ctype)
	if pos == nil {
		fmt.Printf("❌ Аккорд %s %s не найден.\n", *chord, *ctype)
		fmt.Println("Используйте --list для просмотра всех доступных аккордов.")
		return
	}

	fmt.Printf("\n🎸 Аккорд: %s (%s) | Настройка: %s\n", *chord, *ctype, *tuning)
	generator.printFretboard(pos)
}
