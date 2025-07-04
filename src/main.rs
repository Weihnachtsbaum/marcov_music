use std::fs;

use markov::Chain;
use midly::Smf;

fn main() {
    let input = fs::read("input.mid").expect("Could not read MIDI");
    let smf = Smf::parse(&input).expect("Could not parse MIDI");
    println!("{:?}", smf.header);
    println!("Tracks: {}", smf.tracks.len());
    let mut chain = Chain::new();
    let mut out = Smf::new(smf.header);
    for track in smf.tracks {
        chain.feed(track);
        out.tracks.push(chain.generate());
    }
    out.save("out.mid").expect("Could not save MIDI");
    println!("Saved output to `out.mid`");
}
