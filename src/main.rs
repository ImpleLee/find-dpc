use pcf::*;
use itertools::Itertools;
use std::sync::atomic::{ AtomicBool, Ordering };
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;


fn main() {
    let first_lines = [BitBoard(1<<0), BitBoard(1<<1), BitBoard(1<<2), BitBoard(1<<3), BitBoard(1<<4),
                      BitBoard(1<<5), BitBoard(1<<6), BitBoard(1<<7), BitBoard(1<<8), BitBoard(1<<9)];
    let second_lines = [BitBoard(1<<10), BitBoard(1<<11), BitBoard(1<<12), BitBoard(1<<13), BitBoard(1<<14),
                       BitBoard(1<<15), BitBoard(1<<16), BitBoard(1<<17), BitBoard(1<<18), BitBoard(1<<19)];
    let third_lines = [BitBoard(1<<20), BitBoard(1<<21), BitBoard(1<<22), BitBoard(1<<23), BitBoard(1<<24),
                      BitBoard(1<<25), BitBoard(1<<26), BitBoard(1<<27), BitBoard(1<<28), BitBoard(1<<29)];
    let fourth_lines = [BitBoard(1<<30), BitBoard(1<<31), BitBoard(1<<32), BitBoard(1<<33), BitBoard(1<<34),
                       BitBoard(1<<35), BitBoard(1<<36), BitBoard(1<<37), BitBoard(1<<38), BitBoard(1<<39)];
    fn all_pc_able(board: BitBoard) -> bool {
        let mut ok = true;
        for permutation in PIECES.iter().cloned().permutations(7) {
            let abort = AtomicBool::new(false);
            solve_pc(
                &permutation,
                board,
                true,
                true,
                &abort,
                placeability::simple_srs_spins,
                |_| {
                    abort.store(true, Ordering::Release);
                }
            );
            if !abort.load(Ordering::Acquire) {
                ok = false;
                break
            }
        }
        ok
    }
    println!("two cells in first line");
    for (i, firstcell) in first_lines.iter().enumerate() {
        for (j, secondcell) in first_lines.iter().enumerate() {
            if i >= j {
                continue
            }
            if all_pc_able(firstcell.combine(*secondcell)) {
                println!("{} {}", i, j);
            }
        }
    }
    println!("one cell in first line, one in second line");
    for (i, firstcell) in first_lines.iter().enumerate() {
        for (j, secondcell) in second_lines.iter().enumerate() {
            if i > j {
                continue
            }
            if all_pc_able(firstcell.combine(*secondcell)) {
                println!("{} {}", i, j);
            }
        }
    }
    let first_line_all = first_lines.iter().fold(BitBoard(0), |acc, x| acc.combine(*x));
    let second_line_all = second_lines.iter().fold(BitBoard(0), |acc, x| acc.combine(*x));
    let third_line_all = third_lines.iter().fold(BitBoard(0), |acc, x| acc.combine(*x));
    let fourth_line_all = fourth_lines.iter().fold(BitBoard(0), |acc, x| acc.combine(*x));

    println!("12 cells in all 4 lines");
    let cells = first_lines.iter().zip(first_lines.iter().rev())
        .chain(second_lines.iter().zip(second_lines.iter().rev()))
        .chain(third_lines.iter().zip(third_lines.iter().rev()))
        .chain(fourth_lines.iter().zip(fourth_lines.iter().rev()));
    cells.combinations(12).map(|cells| {
        let board = cells.iter().fold(BitBoard(0), |acc, (u, _)| acc.combine(**u));
        let mirror = cells.iter().fold(BitBoard(0), |acc, (_, v)| acc.combine(**v));
        return (board, mirror)
    }).filter(|(board, mirror)| {
        if board.0 > mirror.0 {
            return false
        }
        if board.0 & first_line_all.0 == first_line_all.0 ||
           board.0 & second_line_all.0 == second_line_all.0 ||
           board.0 & third_line_all.0 == third_line_all.0 ||
           board.0 & fourth_line_all.0 == fourth_line_all.0 {
            return false
        }
        if board.0 & first_line_all.0 == 0 {
            return false
        }
        if board.0 & second_line_all.0 == 0 {
            return false
        }
        if board.0 & third_line_all.0 == 0 && board.0 & fourth_line_all.0 != 0 {
            return false
        }
        true
    }).map(|(board, _)| board).par_bridge().filter(|board| all_pc_able(*board)).for_each(|board| {
        println!("{}", board.0);
    });
}
