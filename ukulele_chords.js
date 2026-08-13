// ukulele_chords.js — JavaScript версия

class UkuleleChord {
    constructor(tuning = 'GCEA') {
        this.tuning = tuning;
        this.strings = tuning === 'GCEA' ? ['G', 'C', 'E', 'A'] :
                       tuning === 'ADF#B' ? ['A', 'D', 'F#', 'B'] :
                       ['D', 'G', 'B', 'E'];
        this.chords = this._initChords();
    }

    _initChords() {
        return {
            C: { major: {3:0, 2:0, 1:0, 0:3}, minor: {3:0, 2:3, 1:3, 0:3}, seventh: {3:0, 2:0, 1:0, 0:1} },
            D: { major: {3:2, 2:2, 1:2, 0:0}, minor: {3:2, 2:1, 1:2, 0:0}, seventh: {3:2, 2:1, 1:2, 0:0} },
            E: { major: {3:1, 2:4, 1:0, 0:4}, minor: {3:0, 2:4, 1:3, 0:2}, seventh: {3:0, 2:0, 1:0, 0:4} },
            F: { major: {3:2, 2:0, 1:1, 0:0}, minor: {3:1, 2:0, 1:1, 0:3}, seventh: {3:2, 2:0, 1:1, 0:3} },
            G: { major: {3:0, 2:2, 1:3, 0:2}, minor: {3:0, 2:2, 1:3, 0:3}, seventh: {3:0, 2:2, 1:3, 0:2} },
            A: { major: {3:0, 2:1, 1:2, 0:0}, minor: {3:0, 2:0, 1:2, 0:2}, seventh: {3:0, 2:0, 1:2, 0:1} },
            B: { major: {3:4, 2:3, 1:2, 0:0}, minor: {3:4, 2:2, 1:2, 0:2}, seventh: {3:4, 2:1, 1:2, 0:2} }
        };
    }

    getChord(root, type) {
        if (this.chords[root] && this.chords[root][type]) {
            return this.chords[root][type];
        }
        return null;
    }

    printFretboard(pos) {
        console.log(`\n   ${this.strings.join(' ')}`);
        console.log(`   ${'─'.repeat(this.strings.length * 2 + 1)}`);
        for (let fret = 0; fret < 5; fret++) {
            let line = fret === 0 ? '  ' : `${fret} `;
            line += ' |';
            for (let str = 0; str < 4; str++) {
                if (pos.hasOwnProperty(str) && pos[str] === fret) {
                    line += ' ● |';
                } else if (pos.hasOwnProperty(str) && pos[str] < fret) {
                    line += '   |';
                } else {
                    line += '   |';
                }
            }
            console.log(line);
        }
        console.log('\nАппликатура:');
        for (let str = 0; str < 4; str++) {
            if (pos.hasOwnProperty(str)) {
                if (pos[str] === 0) {
                    console.log(`  ${this.strings[str]}: открытая`);
                } else {
                    console.log(`  ${this.strings[str]}: ${pos[str]}-й лад`);
                }
            }
        }
    }

    listChords() {
        console.log('Доступные аккорды для укулеле:');
        for (const root of Object.keys(this.chords).sort()) {
            const types = Object.keys(this.chords[root]);
            console.log(`  ${root}: ${types.join(', ')}`);
        }
    }
}

function main() {
    const args = process.argv.slice(2);
    let chord = 'C', type = 'major', tuning = 'GCEA', list = false;

    for (let i = 0; i < args.length; i++) {
        if (args[i] === '--chord') chord = args[++i];
        else if (args[i] === '--type') type = args[++i];
        else if (args[i] === '--tuning') tuning = args[++i];
        else if (args[i] === '--list') list = true;
    }

    const generator = new UkuleleChord(tuning);

    if (list) {
        generator.listChords();
        return;
    }

    const pos = generator.getChord(chord, type);
    if (!pos) {
        console.log(`❌ Аккорд ${chord} ${type} не найден.`);
        console.log('Используйте --list для просмотра всех доступных аккордов.');
        return;
    }

    console.log(`\n🎸 Аккорд: ${chord} (${type}) | Настройка: ${tuning}`);
    generator.printFretboard(pos);
}

if (require.main === module) main();
