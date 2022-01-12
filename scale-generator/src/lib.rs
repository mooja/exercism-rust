// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.

static NOTES: [(&str, &str); 12] = [
    ("A", "A"),
    ("A#", "Bb"),
    ("B", "B"),
    ("C", "C"),
    ("C#", "Db"),
    ("D", "D"),
    ("D#", "Eb"),
    ("E", "E"),
    ("F", "F"),
    ("F#", "Gb"),
    ("G", "G"),
    ("G#", "Ab"),
];

#[derive(Debug)]
pub struct Error;

#[derive(Debug)]
pub struct Note(usize);

impl From<&str> for Note {
    fn from(input: &str) -> Self {
        let note_idx = NOTES
            .iter()
            .position(|&(a, b)| input == a || input == b)
            .expect(&format!("no note {}", input));

        Note {
            0: note_idx 
        }
    }
}

#[derive(Debug)]
enum DisplayMode {
    UseSharps,
    UseFlats,
}

#[derive(Debug)]
pub struct Scale {
    notes: Vec<Note>,
    dm: DisplayMode,
}

fn display_mode(tonic: &str, intervals: &str) -> DisplayMode {
    let sharps = "C, G, D, A, E, B, F#, e, b, f#, c#, g#, d#, a"
        .split(',')
        .map(|s| String::from(s.trim()))
        .collect::<Vec<String>>();

    if sharps.contains(&tonic.to_string()) {
        DisplayMode::UseSharps
    } else {
        DisplayMode::UseFlats
    }
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let dm = display_mode(tonic, intervals);
        let mut tonic = tonic.chars().collect::<Vec<_>>();
        tonic[0].make_ascii_uppercase();
        let tonic = &tonic.iter().collect::<String>();

        let tonic_note = Note::from(&tonic[..]);
        let mut scale = Scale {
            notes: vec![tonic_note],
            dm: dm
        };

        let mut last_note_idx = scale.notes[0].0;

        for interval_ch in intervals.chars() {
            if interval_ch == 'm' {
                last_note_idx = (last_note_idx + 1) % 12;
            } else if interval_ch == 'M' {
                last_note_idx = (last_note_idx + 2) % 12;
            } else {
                last_note_idx = (last_note_idx + 3) % 12;
            }

            scale.notes.push(Note { 0: last_note_idx });
        }

        scale.notes.pop().unwrap();
        Ok(scale)
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        use std::iter;
        let interval = iter::repeat('m').take(12).collect::<String>();
        Scale::new(tonic, &interval)
    }

    pub fn enumerate(&self) -> Vec<String> {
        match self.dm {
            DisplayMode::UseSharps => self
                .notes
                .iter()
                .map(|n| NOTES[n.0].0.to_string())
                .collect(),

            DisplayMode::UseFlats => self
                .notes
                .iter()
                .map(|n| NOTES[n.0].1.to_string())
                .collect(),
        }
    }
}
