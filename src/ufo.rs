use crate::audio::Audio;
use crate::canvas;
use crate::math::Vec2;
use crate::{array_sprite::ArraySprite, dot_map::DotMap};
use instant::Instant;
use web_sys::AudioBufferSourceNode;

pub struct Explosion {
    pos: Vec2,
    pub show_cnt: Option<i32>, // 生存フラグ(表示残りカウント)
    score: i32,                // 撃破時のスコア
    sprite: Vec<u8>,           // 左側から縦8ピクセルずつを8bitのベクタで表す
    sprite_num: Vec<Vec<u8>>,
}

impl Explosion {
    fn create_effect(&mut self, dot_map: &mut DotMap, pos: Vec2) {
        self.pos = pos;
        self.show_cnt = Some(0);
        // 爆発エフェクトを表示
        self.array_shifted_sprite(dot_map);
    }
    fn update_draw(&mut self, dot_map: &mut DotMap) {
        if let Some(cnt) = self.show_cnt {
            // カウント終了
            if 120 < cnt {
                // 描画した部分を0で消す
                self.erase(dot_map, self.pos);
                self.show_cnt = None;
                return;
            } else if 20 < cnt {
                // 描画した部分を0で消す
                self.erase(dot_map, self.pos);
                // スコアを描画
                self.draw_score(dot_map);
            }
            self.show_cnt = Some(cnt + 1);
        }
    }
    fn draw_score(&self, dot_map: &mut DotMap) {
        let i_sprite: Vec<usize>;
        match self.score {
            50 => {
                i_sprite = vec![5, 0];
            }
            100 => {
                i_sprite = vec![1, 0, 0];
            }
            150 => {
                i_sprite = vec![1, 5, 0];
            }
            300 => {
                i_sprite = vec![3, 0, 0];
            }
            _ => panic!("UFOの点数が不正です。"),
        }
        let mut pos = self.pos;
        for i in 0..i_sprite.len() {
            Self::array_sprite_num(dot_map, &self.sprite_num[i_sprite[i]], pos);
            pos.x += 8;
        }
    }
    // バイト境界をまたがない物体の描画を透過なしで行う(上書き)
    fn array_sprite_num(dot_map: &mut DotMap, sprite: &Vec<u8>, pos: Vec2) {
        let char_y = (pos.y / 8) as usize;
        for dx in 0..sprite.len() {
            dot_map.map[char_y][pos.x as usize + dx] = sprite[dx];
        }
    }
}
impl ArraySprite for Explosion {
    fn pos(&self) -> Vec2 {
        self.pos
    }
    fn sprite(&self) -> &[u8] {
        &self.sprite
    }
}

pub struct Ufo {
    width: i32,
    pos: Vec2,              // 左上位置
    pre_pos: Vec2,          // 前回描画時の位置
    live: bool,             // 存在しているか否か
    move_dir: i32,          // 移動方向
    lapse_time: Instant,    // 前回画面から消滅したときの時刻
    score_table: [i32; 15], // プレイヤーの発射数に対応した獲得得点表
    sprite: Vec<u8>,        // 左側から縦8ピクセルずつを8bitのベクタで表す
    pub explosion: Explosion,
    flying_sound: Option<AudioBufferSourceNode>,
}

impl Ufo {
    pub fn new(sprite: Vec<u8>, explosion_sprite: Vec<u8>, num_list: Vec<Vec<u8>>) -> Self {
        Ufo {
            width: sprite.len() as i32,
            pos: Vec2::new(0, 8),
            pre_pos: Vec2::new(0, 8),
            live: false,
            move_dir: 1,
            score_table: [
                50, 50, 100, 150, 100, 100, 50, 300, 100, 100, 100, 50, 150, 100, 100,
            ],
            lapse_time: Instant::now(),
            sprite,
            explosion: Explosion {
                pos: Vec2::new(0, 0),
                show_cnt: None,
                sprite: explosion_sprite,
                sprite_num: num_list,
                score: 0,
            },
            flying_sound: None,
        }
    }
    pub fn reset(&mut self) {
        self.live = false;
        self.lapse_time = Instant::now();
    }
    fn remove(&mut self, dot_map: &mut DotMap) {
        self.live = false;
        // 移動方向反転
        self.move_dir *= -1;
        // タイマーリセット
        self.lapse_time = Instant::now();

        // 前回描画した部分を消す
        self.erase(dot_map, self.pre_pos);
        // 飛行音のループ再生を止める
        if let Some(sound_node) = &self.flying_sound {
            sound_node.stop().unwrap();
        }
    }
    // プレイヤーの弾が当たった場合
    pub fn hit_player_bullet(&mut self, dot_map: &mut DotMap, fire_cnt: i32, audio: &Audio) -> i32 {
        // UFOを消す
        self.remove(dot_map);
        // 爆発エフェクト描画
        self.explosion.create_effect(dot_map, self.pos);

        // UFO撃破音再生
        if let Some(sound) = &audio.ufo_explosion {
            audio.play_once_sound(sound);
        }

        let score = self.score_table[(fire_cnt - 1) as usize % 15];
        self.explosion.score = score;
        score
    }
    pub fn update(&mut self, dot_map: &mut DotMap, fire_cnt: i32, alien_num: i32, audio: &Audio) {
        self.pre_pos = self.pos;
        self.explosion.update_draw(dot_map);
        // 画面の反対側まで到達した場合
        if (self.move_dir < 0 && self.pos.x < 8)
            || (0 < self.move_dir && canvas::GAME_WIDTH - 8 <= self.pos.x + self.width)
        {
            self.remove(dot_map);
            return;
        }
        // 移動中
        if self.live {
            self.pos.x += self.move_dir;
        } else {
            // 消滅してから一定時間経過して、かつエイリアンの数が8以上だったら
            if 25 < self.lapse_time.elapsed().as_secs() && 7 < alien_num {
                // UFOが出現する瞬間
                self.live = true;
                // プレイヤーの発射数が偶数であれば右から左へ動く
                if fire_cnt % 2 == 0 {
                    self.pos.x = canvas::GAME_WIDTH - self.width - 8;
                    self.move_dir = -1;
                } else {
                    // 奇数ならば左から右へ動く
                    self.pos.x = 8;
                    self.move_dir = 1;
                }

                // UFO飛行音ループ再生開始
                if let Some(sound) = &audio.ufo_flying {
                    self.flying_sound = Some(audio.play_looping_sound(sound));
                }
            }
        }
        self.draw(dot_map);
    }

    // UFOをドットマップに描画(縦方向のバイト境界はまたがない)
    fn draw(&self, dot_map: &mut DotMap) {
        if !self.live {
            return;
        }
        // 前回描画した部分を0で消す
        self.erase(dot_map, self.pre_pos);
        // 移動後描画する(透過無し)
        self.array_sprite(dot_map);
    }
}

impl ArraySprite for Ufo {
    fn pos(&self) -> Vec2 {
        self.pos
    }
    fn sprite(&self) -> &[u8] {
        &self.sprite
    }
}
