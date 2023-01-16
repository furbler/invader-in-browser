use crate::dot_map::DotMap;
use crate::math::Vec2;

pub trait ArraySprite {
    //  スプライトを置く位置
    fn pos(&self) -> Vec2;
    // スプライト
    fn sprite(&self) -> &[u8];

    // バイト境界をまたぐ(y軸方向へ連続的に移動する)物体の描画を透過ありで行う
    fn array_shifted_sprite(&self, dot_map: &mut DotMap) {
        let pos = self.pos();
        let sprite = self.sprite();
        let char_y = (pos.y / 8) as usize;
        let char_offset_bit = (pos.y % 8) as u8;
        for x in 0..sprite.len() {
            // 1にしたいbitには1、透過部分には0をおく
            let bit_mask: u8 = sprite[x] << char_offset_bit;
            dot_map.map[char_y][pos.x as usize + x] |= bit_mask;
        }
        if char_offset_bit != 0 {
            // 下側にはみ出した部分
            for x in 0..sprite.len() {
                // 1にしたいbitには1、透過部分には0をおく
                let bit_mask = sprite[x] >> (8 - char_offset_bit);
                dot_map.map[char_y + 1][pos.x as usize + x] |= bit_mask;
            }
        }
    }
    // バイト境界をまたがない物体の描画を透過なしで行う(上書き)
    fn array_sprite(&self, dot_map: &mut DotMap) {
        array_sprite(&mut dot_map.map, self.pos(), self.sprite());
    }
    // 引数の座標からスプライトのサイズの矩形部分を消す
    fn erase(&self, dot_map: &mut DotMap, pos: Vec2) {
        erase(&mut dot_map.map, pos, self.sprite());
    }
    // スプライトの部分のみ消し、残りは透過する
    fn erase_shifted(&self, dot_map: &mut DotMap, pos: Vec2) {
        let sprite = self.sprite();

        let char_y = (pos.y / 8) as usize;
        let char_offset_bit = (pos.y % 8) as u8;
        for x in 0..sprite.len() {
            // 0にしたいbitには0、透過部分には1をおく
            let bit_mask: u8 = !(sprite[x] << char_offset_bit);
            dot_map.map[char_y][pos.x as usize + x] &= bit_mask;
        }
        if char_offset_bit != 0 {
            // 下側にはみ出した部分
            for x in 0..sprite.len() {
                // 0にしたいbitには0、透過部分には1をおく
                let bit_mask = !(sprite[x] >> (8 - char_offset_bit));
                dot_map.map[char_y + 1][pos.x as usize + x] &= bit_mask;
            }
        }
    }
    // この当たり判定時には移動前の弾の描画は消されていなければならない(残っていると前回の弾と衝突判定することがある)
    fn is_collide(&self, dot_map: &mut DotMap) -> bool {
        let pos = self.pos();
        let sprite = self.sprite();
        let char_y = (pos.y / 8) as usize;
        let offset_bit = (pos.y % 8) as u8;
        // 移動した弾の部分のビットマスクを作る
        for i in 0..self.sprite().len() {
            let bit_mask = sprite[i] & 0b1111_1111;
            // ビットがバイトの境界をまたぐときの上下それぞれの判定
            let high = dot_map.map[char_y][pos.x as usize] & (bit_mask << offset_bit) != 0;
            let low = if offset_bit != 0 {
                dot_map.map[char_y + 1][pos.x as usize] & (bit_mask >> (8 - offset_bit)) != 0
            } else {
                false
            };
            // 何かに衝突していたら
            if high || low {
                return true;
            }
        }
        false
    }
}

// バイト境界をまたがない物体の描画を透過なしで行う(上書き)
pub fn array_sprite(dot_map: &mut Vec<Vec<u8>>, pos: Vec2, sprite: &[u8]) {
    let char_y = (pos.y / 8) as usize;
    for dx in 0..sprite.len() {
        dot_map[char_y][pos.x as usize + dx] = sprite[dx];
    }
}

// 引数の座標からスプライトのサイズの矩形部分を消す
pub fn erase(dot_map: &mut Vec<Vec<u8>>, pos: Vec2, sprite: &[u8]) {
    // 前回描画した部分を0で消す
    let char_y = (pos.y / 8) as usize;
    for dx in 0..sprite.len() {
        dot_map[char_y][pos.x as usize + dx] = 0;
    }
}
