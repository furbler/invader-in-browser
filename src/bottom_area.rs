use crate::array_sprite::array_sprite;
use crate::canvas;
use crate::math::Vec2;

pub struct BottomArea {
    pub bottom: Vec<Vec<u8>>,
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
}
