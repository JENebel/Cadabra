# Cadabra

Cadabra is a chess engine writen entirely in rust, and is the successor to my first engine [JENCE](https://github.com/PQNebel/JENChessEngine).

If you are lucky, Cadabra is online on [Lichess](https://lichess.org/@/CadabraBot).

Cadabra is tested and listed on [CCRL](http://ccrl.chessdom.com/ccrl/404/). Thanks to the CCRL team!

ELO strength history:
| Version | CCRL Blitz | Lichess |
|---------|------------|---------|
| 1.0     | 1444       |         |
| 2.0     | 1970       | ≈ 2150  |

# Usage

The engine is designed to be used with a UCI compliant GUI. This program does not provide one. GUIs I have used for testing are Arena, CuteChess and Lucas Chess, but many others are available.

<!---
Precompiled binaries are provided under [releases](https://github.com/JENebel/Cadabra/releases). The BMI2 versions are prefferable, but may not be supported on older machines.
-->

For the best results, compile it yourself. Make sure rust and cargo are installed first. Then download and unzip the source code from [releases](https://github.com/JENebel/Cadabra/releases), and simply run the following command in the folder

    cargo build --release

The resulting binary will then be located in Cadabra/target/release/

# Commands

The engine supports the [UCI protocol](https://backscattering.de/chess/uci/) as well as these additional commands:

    d                 Pretty prints the current position.

    fen               Prints the Forsynth Edwards Notation(FEN) for the current position.

    x                 Quits the engine. Equivalent to the UCI 'quit' command.

    move [move]       Make a move on the current position.
                      The move must be legal, and should be formatted in UCI format, eg. a2a4 or b7b8q.

    eval              Print the static heuristic evaluation of the current position in centipawns.

    zobrist           Print the zobrist hash of the current position.

    perft [depth]     Perform a perft test to the desired depth.

    bench [save?]     Benchmark the engine. Performs a benchmark of perft and search performance.
                      If 'save' is appended, it saves the results for use as a baseline.
                      Future runs will then be compared to this result.
                      The preffered way is to run with the 'bench' argument instead to reduce vaiables.
  
    legal             Lists all legal moves on the current position.
  
    threefold         Prints true if in a threefold repetition stalemate, otherwise prints false.

    insufficient      Prints true if in an insufficient material draw, otherwise prints false.
  
    cleartt           Clears the internal transposition table manually.

    fillrate          Displays the current fill rate of transposition table in percentage.


<a id="options"></a>

# UCI options

These options are available through the GUI used, or can be manually changed if run in CLI.
  - Hash table size
    - Sets the hash table size to the desired amount of MBs
    - Default is 16 MB
    - "setoption name Hash value 128"
  - Thread count
    - Sets the amount of threads to the desired count
    - Default is 1
    - "setoption name Threads value 4"
  - Clear hash
    - Simply clears the internal hash table
    - "setoption name Clear Hash"

# Tools

## Benchmarking

There is an internal benchmarking tool for benchmarking performance of the engine. This is also important in testing changes.

A benchmark can be run by using the 'bench' command.
The best way to benchmark is to clone the repository and run the custom cargo command

    cargo benchmark

If debugging, it can be useful to use a debug profile. This is available with the following

    cargo dev_benhmark

To save a baseline to compare against, append 'save'

    cargo benchmark save

## Validator

A validator is also available. It is used exclusively for testing. It validates that the move generator is valid, and can track any errors. This makes it easy to identify bugs in the move generator.

To run this use the custom cargo command

    cargo validate

# Implementation

Move generation
  - Pregenrerated sliding piece attack tables using BMI2's PEXT instructions
  - Many other pregenerated tables to assist in pseudo legal move generation, pin masks etc. to avoid run time calculations
  - This results in a very fast move generator rivaling the best engines' generators, and often beating them in perft speed

Search
  - Negamax alpha beta search followed by quiescence search
  - Quiescence search with pruning
  - Lazy SMP multithreading (Currently only provides limited benefit in practice)
  - Hash table / transposition table
    - A simple replace always scheme is currently used
  - Iterative deepening with growing aspiration window
  - Check extensions
  - 50 move rule, 3-fold repetition and insufficient material draw detection
  - Effective time management
  - Late move reductions
  - Null move pruning
  - Reverse futility pruning
  - Mate distance pruning
  - Move sorting
    - MVV-LVA
    - Killer moves
    - History moves

Evaluation
  - Material scores
  - Interpiolated early/late game piece square tables
  - Mobility bonus
  - Protected king bonus
  - Several other piece dependent factors