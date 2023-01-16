use crate::array_sprite::array_sprite;
use crate::canvas;
use crate::dot_map::Color;
use crate::dot_map::*;
use crate::math::Vec2;
use std::io::Write;
use wasm_bindgen::Clamped;
use web_sys::ImageData;

pub struct BottomArea {
    bottom: Vec<Vec<u8>>,
    num_sprite: Vec<Vec<u8>>,
    player_sprite: Vec<u8>,
}

impl BottomArea {
    pub fn new(num_sprite: Vec<Vec<u8>>, player_sprite: Vec<u8>) -> Self {
        BottomArea {
            // 0クリアしたドットマップを生成
            bottom: vec![
                vec![0; canvas::BOTTOM_WIDTH as usize];
                (canvas::BOTTOM_HEIGHT / 8) as usize
            ],
            num_sprite,
            player_sprite,
        }
    }
    // すべて消す
    pub fn all_clear(&mut self) {
        self.bottom =
            vec![vec![0; canvas::BOTTOM_WIDTH as usize]; (canvas::BOTTOM_HEIGHT / 8) as usize];
    }
    pub fn draw(&mut self, player_life: i32) {
        self.all_clear();
        // 残機の数を表示する(1桁)
        array_sprite(
            &mut self.bottom,
            Vec2::new(8, 0),
            &self.num_sprite[player_life as usize],
        );
        // 残機-1の数だけプレイヤーの画像を並べる
        let mut pos = Vec2::new(24, 0);
        for _ in 0..player_life - 1 {
            array_sprite(&mut self.bottom, pos, &self.player_sprite);
            pos.x += self.player_sprite.len() as i32;
        }
    }

    pub fn dot_map2imagedata(&self, player_exploding: bool) -> (ImageData, Vec<u8>) {
        let rgba = self.convert_to_color_bytes(player_exploding);
        (Self::rgba2imagedata(&rgba), rgba)
    }

    // RGBAデータをImageDataに変換
    fn rgba2imagedata(rgba: &Vec<u8>) -> ImageData {
        ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(rgba),
            canvas::BOTTOM_WIDTH as _,
            canvas::BOTTOM_HEIGHT as _,
        )
        .unwrap()
    }

    // DotMapを1ピクセル4バイトでrgbaを表し、u8のベクタにまとめる
    fn convert_to_color_bytes(&self, player_exploding: bool) -> Vec<u8> {
        let mut color_bytes: Vec<u8> = Vec::new();
        for i_char in 0..(canvas::BOTTOM_HEIGHT / 8) as usize {
            for bit in 0..8 {
                for pos_x in 0..canvas::BOTTOM_WIDTH as usize {
                    if self.bottom[i_char][pos_x] & (1 << bit) == 0 {
                        color_bytes.write(&[0, 0, 0, 255]).unwrap();
                    } else {
                        if player_exploding {
                            // プレイヤーが爆発中はすべて赤にする
                            color_bytes.write(&set_color(Color::Red)).unwrap();
                        } else {
                            color_bytes.write(&set_color(Color::Turquoise)).unwrap();
                        }
                    }
                }
            }
        }
        color_bytes
    }
}
