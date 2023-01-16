use crate::array_sprite::array_sprite;
use crate::canvas;
use crate::math::Vec2;

pub struct TopArea {
    pub top: Vec<Vec<u8>>,
    num_sprite: Vec<Vec<u8>>,
}

impl TopArea {
    pub fn new(num_sprite: Vec<Vec<u8>>) -> Self {
        // 0クリアしたドットマップを生成
        TopArea {
            top: vec![vec![0; canvas::TOP_WIDTH as usize]; (canvas::TOP_HEIGHT / 8) as usize],
            num_sprite,
        }
    }
    // すべて消す
    pub fn all_clear(&mut self) {
        self.top = vec![vec![0; canvas::TOP_WIDTH as usize]; (canvas::TOP_HEIGHT / 8) as usize];
    }
    // 上に獲得得点を表示
    pub fn draw_score(&mut self, mut score: i32) {
        let mut score_num = Vec::new();
        for _ in 0..5 {
            score_num.push(score % 10);
            score /= 10;
        }
        let mut pos = Vec2::new(24, 24);
        for i in (0..5).rev() {
            array_sprite(&mut self.top, pos, &self.num_sprite[score_num[i] as usize]);
            pos.x += 8;
        }
    }
}
