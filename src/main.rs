use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let notes_and_durations = vec![
        (440, Duration::from_secs(1)),
        (880, Duration::from_secs(3)),
        (1200, Duration::from_secs(5)),
    ];

    let notes_and_durations2 = vec![
        (220, Duration::from_secs(1)),
        (1200, Duration::from_secs(3)),
        (1500, Duration::from_secs(5)),
    ];

    let rows_of_notes_and_durations =
        vec![notes_and_durations.clone(), notes_and_durations2.clone()];

    play_many_notes(rows_of_notes_and_durations.clone());
}

fn play_notes_with_duration(notes_and_durations: Vec<(u32, Duration)>) {
    let stream_handle = OutputStream::try_default().unwrap();
    let (sender, receiver) = mpsc::channel();

    // Spawn a thread to play the audio
    thread::spawn(move || {
        // Create a vector to hold the audio sinks
        let mut sinks = Vec::new();

        // Loop over the notes and durations, creating a source and sink for each
        for (note, duration) in receiver.iter() {
            let source = SineWave::new(note).take_duration(duration);
            let sink = Sink::try_new(&stream_handle.1).unwrap();
            sink.append(source);
            //sink.play();

            sinks.push((sink, duration));
        }

        // Wait for the audio to finish playing
        for (sink, duration) in sinks {
            sink.play();
            thread::sleep(duration);
            sink.stop();
        }
    });

    for (note, duration) in notes_and_durations {
        sender.send((note, duration)).unwrap();
    }
    std::thread::sleep(std::time::Duration::from_secs(5));
}

fn play_many_notes(notes_and_durations_table: Vec<Vec<(u32, Duration)>>) {
    for notes_vector in notes_and_durations_table {
        play_notes_with_duration(notes_vector.clone());
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
