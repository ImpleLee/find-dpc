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
                placeability::always,
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
    let first_line_all = first_lines.iter().fold(BitBoard(0), |acc, x| acc.combine(*x));
    let second_line_all = second_lines.iter().fold(BitBoard(0), |acc, x| acc.combine(*x));
    let third_line_all = third_lines.iter().fold(BitBoard(0), |acc, x| acc.combine(*x));
    let fourth_line_all = fourth_lines.iter().fold(BitBoard(0), |acc, x| acc.combine(*x));

    fn floating(lower: u64, upper: u64) -> bool {
        fn get_poses(x: u64) -> Vec<u64> {
            (0..10).filter(|&i| x & (1 << i) != 0).collect()
        }
        fn get_ranges(x: u64) -> Vec<(u64, u64)> {
            let left = x & !(x << 1);
            let right = x & !(x >> 1);
            get_poses(left).into_iter().zip(get_poses(right).into_iter()).collect()
        }
        let mut no_floating = true;
        for (l, r) in get_ranges(upper) {
            let mut found_base = false;
            for (l2, r2) in get_ranges(lower) {
                if l <= r2 && l2 <= r {
                    found_base = true;
                    break
                }
            }
            if !found_base {
                no_floating = false;
                break
            }
        }
        !no_floating
    }

    let COLUMN_MAP = 0b0101010101010101010101010101010101010101u64;
    let PARITY_MAP = 0b0101010101101010101001010101011010101010u64;
    first_lines.iter().zip(first_lines.iter().rev())
        .chain(second_lines.iter().zip(second_lines.iter().rev()))
        .chain(third_lines.iter().zip(third_lines.iter().rev()))
        .chain(fourth_lines.iter().zip(fourth_lines.iter().rev()))
    .combinations(12).filter_map(|cells| {
        let board = cells.iter().fold(BitBoard(0), |acc, (u, _)| acc.combine(**u));
        let mirror = cells.iter().fold(BitBoard(0), |acc, (_, v)| acc.combine(**v));
        if board.0 > mirror.0 {
            return None
        }
        let first = board.0 & first_line_all.0;
        let second = board.0 & second_line_all.0;
        let third = board.0 & third_line_all.0;
        let fourth = board.0 & fourth_line_all.0;
        if first == first_line_all.0 || second == second_line_all.0 ||
           third == third_line_all.0 || fourth == fourth_line_all.0 {
            return None
        }
        if first == 0 || second == 0 ||
           third == 0 && fourth != 0 {
            return None
        }
        if match ((board.0 & PARITY_MAP).count_ones(), (board.0 & COLUMN_MAP).count_ones()) {
                (5|7, _) => false,
                (6|4|8, 6|4|8|2|10) => false,
                _ => true
        } {
            return None
        }
        if third == 0 {
            return Some(board)
        }
        let second_floating = floating(first, second >> 10);
        let third_floating = floating(second, third >> 10);
        let fourth_floating = floating(third, fourth >> 10);
        if (second_floating as u64) + (third_floating as u64) + (fourth_floating as u64) >= 2 {
            return None
        }
        Some(board)
    }).par_bridge().filter(|board| all_pc_able(*board)).for_each(|board| {
        println!("{}", board.0);
    });
}
