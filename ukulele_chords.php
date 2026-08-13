<?php
// ukulele_chords.php — PHP версия

class UkuleleChord {
    private $tuning;
    private $strings;
    private $chords;

    public function __construct($tuning = 'GCEA') {
        $this->tuning = $tuning;
        $this->strings = $tuning == 'GCEA' ? ['G', 'C', 'E', 'A'] :
                        ($tuning == 'ADF#B' ? ['A', 'D', 'F#', 'B'] :
                        ['D', 'G', 'B', 'E']);
        $this->chords = $this->initChords();
    }

    private function initChords() {
        return [
            'C' => [
                'major' => [3=>0, 2=>0, 1=>0, 0=>3],
                'minor' => [3=>0, 2=>3, 1=>3, 0=>3],
                'seventh' => [3=>0, 2=>0, 1=>0, 0=>1]
            ],
            'D' => [
                'major' => [3=>2, 2=>2, 1=>2, 0=>0],
                'minor' => [3=>2, 2=>1, 1=>2, 0=>0],
                'seventh' => [3=>2, 2=>1, 1=>2, 0=>0]
            ],
            'E' => [
                'major' => [3=>1, 2=>4, 1=>0, 0=>4],
                'minor' => [3=>0, 2=>4, 1=>3, 0=>2],
                'seventh' => [3=>0, 2=>0, 1=>0, 0=>4]
            ],
            'F' => [
                'major' => [3=>2, 2=>0, 1=>1, 0=>0],
                'minor' => [3=>1, 2=>0, 1=>1, 0=>3],
                'seventh' => [3=>2, 2=>0, 1=>1, 0=>3]
            ],
            'G' => [
                'major' => [3=>0, 2=>2, 1=>3, 0=>2],
                'minor' => [3=>0, 2=>2, 1=>3, 0=>3],
                'seventh' => [3=>0, 2=>2, 1=>3, 0=>2]
            ],
            'A' => [
                'major' => [3=>0, 2=>1, 1=>2, 0=>0],
                'minor' => [3=>0, 2=>0, 1=>2, 0=>2],
                'seventh' => [3=>0, 2=>0, 1=>2, 0=>1]
            ],
            'B' => [
                'major' => [3=>4, 2=>3, 1=>2, 0=>0],
                'minor' => [3=>4, 2=>2, 1=>2, 0=>2],
                'seventh' => [3=>4, 2=>1, 1=>2, 0=>2]
            ]
        ];
    }

    public function getChord($root, $type) {
        if (isset($this->chords[$root]) && isset($this->chords[$root][$type])) {
            return $this->chords[$root][$type];
        }
        return null;
    }

    public function printFretboard($pos) {
        echo "\n   " . implode(' ', $this->strings) . "\n";
        echo "   " . str_repeat('─', count($this->strings) * 2 + 1) . "\n";
        for ($fret = 0; $fret < 5; $fret++) {
            $line = $fret == 0 ? '  ' : "$fret ";
            $line .= ' |';
            for ($str = 0; $str < 4; $str++) {
                if (isset($pos[$str]) && $pos[$str] == $fret) {
                    $line .= ' ● |';
                } elseif (isset($pos[$str]) && $pos[$str] < $fret) {
                    $line .= '   |';
                } else {
                    $line .= '   |';
                }
            }
            echo $line . "\n";
        }
        echo "\nАппликатура:\n";
        for ($str = 0; $str < 4; $str++) {
            if (isset($pos[$str])) {
                if ($pos[$str] == 0) {
                    echo "  {$this->strings[$str]}: открытая\n";
                } else {
                    echo "  {$this->strings[$str]}: {$pos[$str]}-й лад\n";
                }
            }
        }
    }

    public function listChords() {
        echo "Доступные аккорды для укулеле:\n";
        $roots = array_keys($this->chords);
        sort($roots);
        foreach ($roots as $root) {
            $types = implode(', ', array_keys($this->chords[$root]));
            echo "  $root: $types\n";
        }
    }
}

function main($argv) {
    $chord = 'C';
    $type = 'major';
    $tuning = 'GCEA';
    $list = false;

    for ($i = 1; $i < count($argv); $i++) {
        if ($argv[$i] == '--chord') { $chord = $argv[++$i]; }
        elseif ($argv[$i] == '--type') { $type = $argv[++$i]; }
        elseif ($argv[$i] == '--tuning') { $tuning = $argv[++$i]; }
        elseif ($argv[$i] == '--list') { $list = true; }
    }

    $generator = new UkuleleChord($tuning);

    if ($list) {
        $generator->listChords();
        return;
    }

    $pos = $generator->getChord($chord, $type);
    if ($pos === null) {
        echo "❌ Аккорд $chord $type не найден.\n";
        echo "Используйте --list для просмотра всех доступных аккордов.\n";
        return;
    }

    echo "\n🎸 Аккорд: $chord ($type) | Настройка: $tuning\n";
    $generator->printFretboard($pos);
}

$argc = $_SERVER['argc'] ?? 0;
$argv = $_SERVER['argv'] ?? [];
main($argv);
?>
