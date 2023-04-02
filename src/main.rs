/**
 * This is a chess engine written in Rust
 * This program will be used to play chess an program an AI. It will use the best optimization possible such as BitBoard etc...
 * This program will be used to learn Rust and to learn how to program an AI
 * Each piece will be represented by a u8 which will contain the type of piece and the color.
 * The board will be represented by a 8x8 array of u8
*/

const NORTH_MASK: u64 = 0x000000000000FF00;
const SOUTH_MASK: u64 = 0x00FF000000000000;
const EAST_MASK: u64 = 0x0101010101010100;
const WEST_MASK: u64 = 0x0080808080808080;
const NOT_A_FILE: u64 = 0xFEFEFEFEFEFEFEFE;
const NOT_H_FILE: u64 = 0x7F7F7F7F7F7F7F7F;
struct Board {
    white_pawns: u64,
    white_pawns_start: u64,
    black_pawns: u64,
    black_pawns_start: u64,
    white_knights: u64,
    black_knights: u64,
    white_rooks: u64,
    black_rooks: u64,
    white_bishops: u64,
    black_bishops: u64,
    white_queens: u64,
    black_queens: u64,
    white_king: u64,
    black_king: u64,
    occupied: u64,
}

impl Board {
    fn new() -> Self {
        // Bitboard pour les pions blancs
        let mut white_pawns: u64 =
            0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;
        let mut white_pawns_start: u64 =
            0b00000000_00000000_00000000_00000000_00000000_00000000__11111111_00000000;

        // Bitboard pour les pions noirs
        let mut black_pawns: u64 =
            0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
        let mut black_pawns_start: u64 =
            0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000;

        // Bitboard pour les cavaliers blancs
        let mut white_knights: u64 =
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010;

        // Bitboard pour les cavaliers noirs
        let mut black_knights: u64 =
            0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000;

        // Bitboard pour les tours blanches
        let mut white_rooks: u64 =
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001;

        // Bitboard pour les tours noires
        let mut black_rooks: u64 =
            0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000;

        // Bitboard pour les fous blancs
        let mut white_bishops: u64 =
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100;

        // Bitboard pour les fous noirs
        let mut black_bishops: u64 =
            0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000;

        // Bitboard pour les reines blanches
        let mut white_queens: u64 =
            0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00010000;

        // Bitboard pour les reines noires
        let mut black_queens: u64 =
            0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00100100;

        // Bitboard pour les rois blancs
        let mut white_king: u64 =
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000;

        // Bitboard pour les rois noirs
        let mut black_king: u64 =
            0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;

        let occupied = white_pawns
            | black_pawns
            | white_knights
            | black_knights
            | white_rooks
            | black_rooks
            | white_bishops
            | black_bishops
            | white_queens
            | black_queens
            | white_king
            | black_king;

        Board {
            white_pawns,
            white_pawns_start,
            black_pawns,
            black_pawns_start,
            white_knights,
            black_knights,
            white_rooks,
            black_rooks,
            white_bishops,
            black_bishops,
            white_queens,
            black_queens,
            white_king,
            black_king,
            occupied,
        }
    }

    fn white_pawn_moves(&self) -> u64 {
        let mut moves: u64 = 0;

        // Déplacements en avant
        moves |= self.white_pawns << 8;

        // Déplacements en avant depuis la position de départ
        moves |= ((self.white_pawns & self.white_pawns_start) << 16) & !self.occupied;

        // Prise à droite
        moves |= (self.white_pawns & 0x7F7F7F7F7F7F7F7F) << 9;

        // Prise à gauche
        moves |= (self.white_pawns & 0xFEFEFEFEFEFEFEFE) << 7;

        return moves;
    }

    fn black_pawn_moves(&self) -> u64 {
        let mut moves: u64 = 0;

        // Déplacements en avant
        moves |= self.black_pawns >> 8;

        // Déplacements en avant depuis la position de départ
        moves |= ((self.black_pawns & self.black_pawns_start) >> 16) & !self.occupied;

        // Prise à droite
        moves |= (self.black_pawns & 0xFEFEFEFEFEFEFEFE) >> 9;

        // Prise à gauche
        moves |= (self.black_pawns & 0x7F7F7F7F7F7F7F7F) >> 7;

        return moves;
    }

    fn white_knight_moves(&self) -> u64 {
        let mut moves: u64 = 0;

        // Déplacements en L
        moves |= (self.white_knights & 0xFEFEFEFEFEFEFEFE) << 6;
        moves |= (self.white_knights & 0x7F7F7F7F7F7F7F7F) << 10;
        moves |= (self.white_knights & 0xFEFEFEFEFEFEFEFE) >> 10;
        moves |= (self.white_knights & 0x7F7F7F7F7F7F7F7F) >> 6;
        moves |= (self.white_knights & 0x00FFFFFFFFFFFF00) << 15;
        moves |= (self.white_knights & 0x00FFFFFFFFFFFF00) >> 17;
        moves |= (self.white_knights & 0x00FFFFFFFFFFFF00) << 17;
        moves |= (self.white_knights & 0x00FFFFFFFFFFFF00) >> 15;

        return moves;
    }

    fn black_knight_moves(&self) -> u64 {
        let mut moves: u64 = 0;

        // Déplacements en L
        moves |= (self.black_knights & 0xFEFEFEFEFEFEFEFE) << 6;
        moves |= (self.black_knights & 0x7F7F7F7F7F7F7F7F) << 10;
        moves |= (self.black_knights & 0xFEFEFEFEFEFEFEFE) >> 10;
        moves |= (self.black_knights & 0x7F7F7F7F7F7F7F7F) >> 6;
        moves |= (self.black_knights & 0x00FFFFFFFFFFFF00) << 15;
        moves |= (self.black_knights & 0x00FFFFFFFFFFFF00) >> 17;
        moves |= (self.black_knights & 0x00FFFFFFFFFFFF00) << 17;
        moves |= (self.black_knights & 0x00FFFFFFFFFFFF00) >> 15;

        return moves;
    }

    fn white_rooks_moves(&self) -> u64 {
        let mut moves: u64 = 0;

        let mut north: u64 = self.white_rooks;
        while north & NORTH_MASK != 0 {
            north = north << 8;
            moves |= north;
            if north & self.occupied != 0 {
                break;
            }
        }

        let mut south: u64 = self.white_rooks;
        while south & SOUTH_MASK != 0 {
            south = south >> 8;
            moves |= south;
            if south & self.occupied != 0 {
                break;
            }
        }

        let mut east: u64 = self.white_rooks;
        while east & EAST_MASK != 0 {
            east = (east << 1) & NOT_A_FILE;
            moves |= east;
            if east & self.occupied != 0 {
                break;
            }
        }

        let mut west: u64 = self.white_rooks;
        while west & WEST_MASK != 0 {
            west = (west >> 1) & NOT_H_FILE;
            moves |= west;
            if west & self.occupied != 0 {
                break;
            }
        }

        moves
    }
}

fn main() {
    let board = Board::new();
    println!("{:064b}", board.white_pawn_moves());
    println!("{:064b}", board.black_pawn_moves());
    println!("{:064b}", board.white_knight_moves());
    println!("{:064b}", board.black_knight_moves());
    println!("{:064b}", board.white_rooks_moves());
}
