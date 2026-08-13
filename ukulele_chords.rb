# ukulele_chords.rb — Ruby версия

class UkuleleChord
  attr_reader :chords, :strings

  def initialize(tuning = 'GCEA')
    @tuning = tuning
    @strings = case tuning
               when 'GCEA' then ['G', 'C', 'E', 'A']
               when 'ADF#B' then ['A', 'D', 'F#', 'B']
               else ['D', 'G', 'B', 'E']
               end
    @chords = init_chords
  end

  def init_chords
    {
      'C' => {
        'major' => {3=>0, 2=>0, 1=>0, 0=>3},
        'minor' => {3=>0, 2=>3, 1=>3, 0=>3},
        'seventh' => {3=>0, 2=>0, 1=>0, 0=>1}
      },
      'D' => {
        'major' => {3=>2, 2=>2, 1=>2, 0=>0},
        'minor' => {3=>2, 2=>1, 1=>2, 0=>0},
        'seventh' => {3=>2, 2=>1, 1=>2, 0=>0}
      },
      'E' => {
        'major' => {3=>1, 2=>4, 1=>0, 0=>4},
        'minor' => {3=>0, 2=>4, 1=>3, 0=>2},
        'seventh' => {3=>0, 2=>0, 1=>0, 0=>4}
      },
      'F' => {
        'major' => {3=>2, 2=>0, 1=>1, 0=>0},
        'minor' => {3=>1, 2=>0, 1=>1, 0=>3},
        'seventh' => {3=>2, 2=>0, 1=>1, 0=>3}
      },
      'G' => {
        'major' => {3=>0, 2=>2, 1=>3, 0=>2},
        'minor' => {3=>0, 2=>2, 1=>3, 0=>3},
        'seventh' => {3=>0, 2=>2, 1=>3, 0=>2}
      },
      'A' => {
        'major' => {3=>0, 2=>1, 1=>2, 0=>0},
        'minor' => {3=>0, 2=>0, 1=>2, 0=>2},
        'seventh' => {3=>0, 2=>0, 1=>2, 0=>1}
      },
      'B' => {
        'major' => {3=>4, 2=>3, 1=>2, 0=>0},
        'minor' => {3=>4, 2=>2, 1=>2, 0=>2},
        'seventh' => {3=>4, 2=>1, 1=>2, 0=>2}
      }
    }
  end

  def get_chord(root, type)
    @chords.dig(root, type)
  end

  def print_fretboard(pos)
    puts "\n   #{@strings.join(' ')}"
    puts "   " + "─" * (@strings.length * 2 + 1)
    (0..4).each do |fret|
      line = fret == 0 ? '  ' : "#{fret} "
      line << ' |'
      (0..3).each do |str|
        if pos.key?(str) && pos[str] == fret
          line << ' ● |'
        elsif pos.key?(str) && pos[str] < fret
          line << '   |'
        else
          line << '   |'
        end
      end
      puts line
    end
    puts "\nАппликатура:"
    (0..3).each do |str|
      if pos.key?(str)
        if pos[str] == 0
          puts "  #{@strings[str]}: открытая"
        else
          puts "  #{@strings[str]}: #{pos[str]}-й лад"
        end
      end
    end
  end

  def list_chords
    puts "Доступные аккорды для укулеле:"
    @chords.keys.sort.each do |root|
      types = @chords[root].keys.join(', ')
      puts "  #{root}: #{types}"
    end
  end
end

def main
  chord = 'C'
  type = 'major'
  tuning = 'GCEA'
  list = false

  args = ARGV
  i = 0
  while i < args.size
    case args[i]
    when '--chord' then chord = args[i+1]; i += 2
    when '--type' then type = args[i+1]; i += 2
    when '--tuning' then tuning = args[i+1]; i += 2
    when '--list' then list = true; i += 1
    else i += 1
    end
  end

  generator = UkuleleChord.new(tuning)

  if list
    generator.list_chords
    return
  end

  pos = generator.get_chord(chord, type)
  if pos.nil?
    puts "❌ Аккорд #{chord} #{type} не найден."
    puts "Используйте --list для просмотра всех доступных аккордов."
    return
  end

  puts "\n🎸 Аккорд: #{chord} (#{type}) | Настройка: #{tuning}"
  generator.print_fretboard(pos)
end

main if __FILE__ == $0
