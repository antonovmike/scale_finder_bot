use crate::scales::*;

#[allow(unused)]
pub fn octave() -> (&'static str, &'static str) {
    ("C  D  E F  G  A  H C  ",
    "2  2  1 2  2  2  1 2  ")
}

#[allow(unused)]
pub fn semitones(scale: &str) -> String {
    let current_scale = match &scale.to_lowercase()[..] {
        "major" => MAJOR_IONIAN,
        "ionian" => MAJOR_IONIAN,
        "dorian" => DORIAN,
        "phrygian" => PHRYGIAN,
        "lydian" => LYDIAN,
        "mixolydian" => MIXOLYDIAN,
        "minor" => MINOR_AEOLIAN,
        "aeolian" => MINOR_AEOLIAN,
        "locrian" => LOCRIAN,
        &_ => [0, 0, 0, 0, 0, 0, 0, 0],
    };
    let mut answer = "".to_string();
    for i in current_scale {
        if i == 1 {
            answer.push('1');
            answer.push(' ')
        } else {
            answer.push('2');
            answer.push(' ');
            answer.push(' ')
        };
    }
    format!("{}: {}", scale.to_uppercase(), answer)
}

#[allow(unused)]
pub fn scale_builder(note: char, acc: char, scale: &str) -> Vec<String> {
    let (mut note_name, mut any_acc) = note_and_acc(note, acc);
    if note_name == 'X' { return vec![] }
    if any_acc   == 'X' { return vec![] }

    if note == 'E' && any_acc == '#' {
        any_acc = ' ';
        note_name = 'F'
    }
    if note == 'F' && any_acc == 'b' {
        any_acc = ' ';
        note_name = 'E'
    }
    if note == 'H' && any_acc == '#' {
        any_acc = ' ';
        note_name = 'C'
    }
    if note == 'C' && any_acc == 'b' {
        any_acc = ' ';
        note_name = 'H'
    }

    let current_scale = match &scale.to_lowercase()[..] {
        "major"      => MAJOR_IONIAN,
        "ionian"     => MAJOR_IONIAN,
        "dorian"     => DORIAN,
        "phrygian"   => PHRYGIAN,
        "lydian"     => LYDIAN,
        "mixolydian" => MIXOLYDIAN,
        "minor"      => MINOR_AEOLIAN,
        "aeolian"    => MINOR_AEOLIAN,
        "locrian"    => LOCRIAN,
        &_ => [0, 0, 0, 0, 0, 0, 0, 0],
    };

    let (note_semitones, note_sequence) = sequencer(note_name);

    if any_acc == ' ' {
        return root_clean(note_semitones, note_sequence, current_scale)
    }
    if any_acc == '#' {
        return root_sharp(note_semitones, note_sequence, current_scale)
    }
    if any_acc == 'b' {
        return root_flat(note_semitones, note_sequence, current_scale)
    }

    return vec![]
}

fn root_clean(note_semitones: Vec<u8>, note_sequence: Vec<char>, current_scale: [u8; 8]) -> Vec<String> {
    let mut empty_vec: Vec<String> = vec![];
    let mut index = 0;
    let mut shift_up = false;
    let mut shift_down = false;

    for i in note_semitones {
        if i == current_scale[index] {
            if !shift_down && !shift_up {
                let note = format!("{}", note_sequence[index]);
                empty_vec.push(note);
            }
            if shift_down && !shift_up {
                let note = format!("{}{}", note_sequence[index], 'b');
                empty_vec.push(note);
            }
            if shift_up && !shift_down {
                let note = format!("{}{}", note_sequence[index], '#');
                empty_vec.push(note);
            }
        } else if i < current_scale[index] && !shift_up && !shift_down {
            let note = format!("{}", note_sequence[index]);
            empty_vec.push(note);
            shift_up = true
        } else if i > current_scale[index] && !shift_up && !shift_down {
            let note = format!("{}", note_sequence[index]);
            empty_vec.push(note);
            shift_down = true
        } else if i < current_scale[index] && shift_up && !shift_down {
            let note = format!("{}{}", note_sequence[index], '#');
            empty_vec.push(note);
        } else if i > current_scale[index] && shift_up && !shift_down {
            let note = format!("{}{}", note_sequence[index], '#');
            empty_vec.push(note);
            shift_up = false
        } else if i > current_scale[index] && shift_down && !shift_up {
            let note = format!("{}{}", note_sequence[index], 'b');
            empty_vec.push(note);
        } else if i < current_scale[index] && shift_down && !shift_up {
            let note = format!("{}{}", note_sequence[index], 'b');
            empty_vec.push(note);
            shift_down = false
        }
        index += 1
    }

    empty_vec
}

fn root_sharp(note_semitones: Vec<u8>, note_sequence: Vec<char>, current_scale: [u8; 8]) -> Vec<String> {
    let mut empty_vec: Vec<String> = vec![];
    let mut index = 0;
    let mut shift_up = true;

    for i in note_semitones {
        if i == current_scale[index] {
            if !shift_up {
                let note = format!("{}", note_sequence[index]);
                empty_vec.push(note);
            }
            if shift_up {
                let note = format!("{}{}", note_sequence[index], '#');
                empty_vec.push(note);
            }
        } else if i < current_scale[index] && !shift_up {
            let note = format!("{}", note_sequence[index]);
            empty_vec.push(note);
            shift_up = true
        } else if i > current_scale[index] && !shift_up {
            let note = format!("{}", note_sequence[index]);
            empty_vec.push(note);
        } else if i < current_scale[index] && shift_up {
            let note = format!("{}{}", note_sequence[index], '#');
            empty_vec.push(note);
        } else if i > current_scale[index] && shift_up {
            let note = format!("{}{}", note_sequence[index], '#');
            empty_vec.push(note);
            shift_up = false
        }

        index += 1
    }

    empty_vec
}

fn root_flat(note_semitones: Vec<u8>, note_sequence: Vec<char>, current_scale: [u8; 8]) -> Vec<String> {
    let mut empty_vec: Vec<String> = vec![];
    let mut index = 0;
    let mut shift_down = true;

    for i in note_semitones {
        if i == current_scale[index] {
            if !shift_down {
                let note = format!("{}", note_sequence[index]);
                empty_vec.push(note);
            }
            if shift_down {
                let note = format!("{}{}", note_sequence[index], 'b');
                empty_vec.push(note);
            }
        } else if i < current_scale[index] && !shift_down {
            let note = format!("{}", note_sequence[index]);
            empty_vec.push(note);
        } else if i > current_scale[index] && !shift_down {
            let note = format!("{}", note_sequence[index]);
            empty_vec.push(note);
            shift_down = true
        } else if i > current_scale[index] && shift_down {
            let note = format!("{}{}", note_sequence[index], 'b');
            empty_vec.push(note);
        } else if i < current_scale[index] && shift_down {
            let note = format!("{}{}", note_sequence[index], 'b');
            empty_vec.push(note);
            shift_down = false
        }
        index += 1
    }

    empty_vec
}

fn sequencer(note_name: char) -> (Vec<u8>, Vec<char>) {
    let note_step: (char, u8) = match note_name {
        'C' => ('C', 2),
        'D' => ('D', 2),
        'E' => ('E', 1),
        'F' => ('F', 2),
        'G' => ('G', 2),
        'A' => ('A', 2),
        'H' => ('H', 1),
        'B' => ('H', 1),
        _ => (' ', 0),
    };
    // Split octave at the root note
    let split_here = OCTAVE_STEPS.iter().position(|&r| r == note_step).unwrap();
    let (first, second) = OCTAVE_STEPS.split_at(split_here);
    // Unite both parts. Now note sequense starts with root note
    let mut a = first.to_vec();
    a.push(note_step);
    let mut b = second.to_vec();
    b.append(&mut a);

    let note_sequence_tuple = b;
    let mut note_semitones: Vec<u8> = vec![];
    for i in note_sequence_tuple.clone() { note_semitones.push(i.1) }

    let mut note_sequence: Vec<char> = vec![];
    for i in note_sequence_tuple { note_sequence.push(i.0) }

    (note_semitones, note_sequence)
}

// Check if note_str is correct: CDEFGAH or B
// Check for accidentals: flat and sharp
#[allow(unused)]
fn note_and_acc(note: char, acc: char) -> (char, char) {
    let note_upper = note
        .to_uppercase()
        .to_string()
        .chars()
        .next()
        .expect("string is empty");
    let mut note_name: char = ' ';

    if "CDEFGAH".contains(note_upper) { note_name = note_upper } 
	else if note_upper == 'B' { note_name = 'H' } 
	else { note_name = 'X' }

    let mut any_acc = 'X';
    if acc == 'b' || acc == 'B' { any_acc = 'b' }
    if acc == '#' { any_acc = '#' }
    if acc == ' ' { any_acc = ' ' }

    (note_name, any_acc)
}
