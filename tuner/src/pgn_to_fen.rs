use std::{path::PathBuf, io::{BufReader, LineWriter, Write}, fs::File, time::Instant};
use cadabra::*;
use pgn_reader::{Visitor, Skip, BufferedReader, SanPlus};
use rand::{thread_rng, seq::SliceRandom};
use shakmaty::{*, fen::Epd, Position};

struct GameParser {
    games: usize,
    pos: Chess,
    lines: Vec<String>,
    fens: u64,
    outcome: String,
    prev_fen: String,
    context: SearchContext,
    equal_score: usize,
    different_score: usize,
}

impl GameParser {
    fn new() -> GameParser {
        GameParser { 
            games: 0,
            pos: Chess::default(),
            lines: Vec::new(),
            outcome: String::new(),
            fens: 0,
            prev_fen: String::new(),
            context: SearchContext::new(Search::new(Settings::default()), SearchArgs::new_simple_depth(0), cadabra::Position::start_pos(), Instant::now(), false),
            equal_score: 0,
            different_score: 0,
        }
    }
}

impl Visitor for GameParser {
    type Result = Result<(), ()>;

    fn begin_game(&mut self) {
        self.pos = Chess::default();
        self.games += 1;
        if self.games % 1000 == 0 {
            println!("Games: {}. Equal score: {}%", self.games, self.equal_score as f64 / (self.equal_score + self.different_score) as f64 * 100.);
        }
    }

    fn header(&mut self, key: &[u8], value: pgn_reader::RawHeader<'_>) {
        if key == b"Result" {
            self.outcome = String::from_utf8(value.0.to_vec()).unwrap();
        }
    }

    fn san(&mut self, san_plus: SanPlus) {
        // Skip if it was a book move
        if let Ok(m) = san_plus.san.to_move(&self.pos) {
            self.pos.play_unchecked(&m);
            self.prev_fen = Epd::from_position(self.pos.clone(), EnPassantMode::Always).to_string();
            self.fens += 1;
        }
    }

    fn comment(&mut self, comment: pgn_reader::RawComment<'_>) {
        let comment = String::from_utf8(comment.0.to_vec()).unwrap();
        if comment.contains("book") || comment.contains("M") {
            return;
        }

        let pos = cadabra::Position::from_fen(&self.prev_fen).unwrap();
        let eval = pos.evaluate(CONST_EVALUATOR);
        let q_sqore = quiescence(&pos, -INFINITY, INFINITY, 0, &mut self.context, CONST_EVALUATOR);

        if eval != q_sqore {
            self.different_score += 1;
            return;
        } else {
            self.equal_score += 1;
        }

        let result = match self.outcome.as_str() {
            "1-0" => "1",
            "0-1" => "0",
            "1/2-1/2" => "½",
            r => panic!("Invalid result: {r}"),
        };

        self.lines.append(&mut vec![format!("[{}]{}", result, self.prev_fen)]);
        //self.writer.write_fmt(format_args!("{}*{}\n", result, self.prev_fen)).unwrap();
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true) // stay in the mainline
    }

    fn end_game(&mut self) -> Self::Result {
        Ok(())
    }
}

/// Generates a fen file from a pgn file.
/// 
/// Excludes book moves and moves where mate is found.
pub fn generate_fen_from_pgn(pgn_path: PathBuf) -> PathBuf {
    let fen_path = pgn_path.with_extension("fen");

    let f = File::open(pgn_path).expect("Unable to open file");
    let mut br = BufReader::new(f);
    let mut writer = LineWriter::new(File::create(fen_path.clone()).expect("Unable to create file"));

    let mut reader = BufferedReader::new(&mut br);
    let mut parser = GameParser::new();

    let before = Instant::now();
    reader.read_all(&mut parser).unwrap();
    println!("Parsed {} games in {}", parser.games, pretty_duration::pretty_duration(&before.elapsed(), None));
    let before = Instant::now();
    parser.lines.shuffle(&mut thread_rng());
    writer.write_all(parser.lines.join("\n").as_bytes()).unwrap();
    println!("Wrote {} fens in {}", parser.lines.len(), pretty_duration::pretty_duration(&before.elapsed(), None));

    fen_path
}