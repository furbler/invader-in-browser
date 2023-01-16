use crate::canvas;
use std::io::Write;
use wasm_bindgen::Clamped;
use web_sys::ImageData;

pub struct DotMap {
    // ドット単位の処理をする範囲
    // 上からy文字目、左からxドット目にあるu8はmap[y][x]
    // 横8x28、縦26個のu8がある二次元配列
    pub map: Vec<Vec<u8>>,
}

impl DotMap {
    pub fn new() -> Self {
        // 0クリアしたドットマップを生成
        DotMap {
            map: vec![vec![0; canvas::GAME_WIDTH as usize]; (canvas::GAME_HEIGHT / 8) as usize],
        }
    }
    // すべて消す
    pub fn all_clear(&mut self) {
        self.map = vec![vec![0; canvas::GAME_WIDTH as usize]; (canvas::GAME_HEIGHT / 8) as usize]
    }
    // 指定したドット単位のY座標のすべてを1にして水平の線を引く
    pub fn draw_holizon_line(&mut self, y: i32) {
        let y = y as usize;
        let char_pos_y = y / 8;
        let mask_val: u8 = 1 << (y % 8);
        for i in 0..canvas::GAME_WIDTH as usize {
            self.map[char_pos_y][i] = self.map[char_pos_y][i] | mask_val;
        }
    }
    // DotMapを1ピクセル4バイトでrgbaを表し、u8のベクタにまとめる
    fn convert_to_color_bytes(&self, player_exploding: bool) -> Vec<u8> {
        let mut color_bytes: Vec<u8> = Vec::new();
        for i_char in 0..(canvas::GAME_HEIGHT / 8) as usize {
            for bit in 0..8 {
                for pos_x in 0..canvas::GAME_WIDTH as usize {
                    if self.map[i_char][pos_x] & (1 << bit) == 0 {
                        color_bytes.write(&[0, 0, 0, 255]).unwrap();
                    } else {
                        if player_exploding {
                            // プレイヤーが爆発中はすべて赤にする
                            color_bytes.write(&set_color(Color::Red)).unwrap();
                        } else {
                            // 高さに応じて色を変える
                            color_bytes.write(&pos2rgba(i_char)).unwrap();
                        }
                    }
                }
            }
        }
        color_bytes
    }
    pub fn dot_map2imagedata(&self, player_exploding: bool) -> (ImageData, Vec<u8>) {
        let rgba = self.convert_to_color_bytes(player_exploding);
        (rgba2imagedata(&rgba), rgba)
    }
}

// RGBAデータをImageDataに変換
fn rgba2imagedata(rgba: &Vec<u8>) -> ImageData {
    ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(rgba),
        canvas::GAME_WIDTH as _,
        canvas::GAME_HEIGHT as _,
    )
    .unwrap()
}

pub enum Color {
    Red,       // 赤色
    Purple,    // 紫色
    BLUE,      // 青色
    Green,     // 緑色
    Turquoise, // 水色
    Yellow,    // 黄色
    White,     // 白色
}
// 指定した色に対応するrgbaの値を返す
pub fn set_color(color: Color) -> [u8; 4] {
    match color {
        Color::Red => [210, 0, 0, 255],          // 赤色
        Color::Purple => [220, 20, 230, 255],    // 紫色
        Color::BLUE => [83, 83, 241, 255],       // 青色
        Color::Green => [98, 222, 109, 255],     // 緑色
        Color::Turquoise => [68, 200, 210, 255], // 水色
        Color::Yellow => [220, 210, 30, 255],    // 黄色
        Color::White => [220, 220, 220, 255],    // 白色
    }
}
// 引数の位置に対応したrgba値を返す
fn pos2rgba(char_y: usize) -> [u8; 4] {
    let color = match char_y {
        0 | 20..=22 | 25 => Color::Red,
        1 | 12..=15 => Color::Purple,
        2 | 3 => Color::BLUE,
        4..=7 => Color::Green,
        8..=11 | 23 | 24 => Color::Turquoise,
        16..=19 => Color::Yellow,
        _ => panic!("文字単位で{}行目は画面からはみだしています。", char_y),
    };
    set_color(color)
}
