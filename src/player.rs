use crate::alien::Alien;
use crate::audio::Audio;
use crate::canvas;
use crate::input::KeyDown;
use crate::math::Vec2;
use crate::ufo::Ufo;
use crate::{array_sprite::ArraySprite, dot_map::DotMap};
// プレイヤーの弾のスピード
const PLAYER_BULLET_DELTA: i32 = 4;

pub struct Bullet {
    pos: Vec2,                   // 左上位置
    live: bool,                  // 弾が存在しているか否か
    explosion_effect_show: bool, // 爆発エフェクトを表示するならば真
    ban_fire_cnt: Option<i32>,   // 射撃禁止状態の残りカウント
    pub fire_cnt: i32,           // ステージ開始からの累計射撃数
    pub score: i32,              // 獲得点数
    sprite: Vec<u8>,             // 左側から縦8ピクセルずつを8bitのベクタで表す
    explosion_sprite: Vec<u8>,   // 爆発画像
}

impl Bullet {
    pub fn new(sprite: Vec<u8>, explosion_sprite: Vec<u8>) -> Self {
        Bullet {
            pos: Vec2::new(0, 0),
            live: false,
            explosion_effect_show: false,
            ban_fire_cnt: None,
            fire_cnt: 0,
            sprite,
            explosion_sprite,
            score: 0,
        }
    }
    // 弾を発射
    fn fire(&mut self, x: i32, y: i32) {
        if self.live {
            return;
        }
        // 弾が画面上に存在しない場合
        self.pos = Vec2::new(x, y);
        self.live = true;
        self.fire_cnt += 1;
        self.explosion_effect_show = false;
    }
    pub fn reset_all(&mut self) {
        self.reset_stage();
        self.score = 0;
    }
    pub fn reset_stage(&mut self) {
        self.live = false;
        self.explosion_effect_show = false;
        // 開始後一定時間は射撃禁止
        self.ban_fire_cnt = Some(80);
        self.fire_cnt = 0;
    }
    pub fn update(
        &mut self,
        dot_map: &mut DotMap,
        player: &mut Player,
        ufo: &mut Ufo,
        alien: &mut Alien,
        input_key: &KeyDown,
        audio: &Audio,
    ) {
        // 弾が存在していたら
        if self.live {
            // 前回の弾を消す
            self.erase_shifted(dot_map, self.pos);
            // 弾の移動
            self.pos.y -= PLAYER_BULLET_DELTA;
            // 弾が画面上部に行ったら
            if self.pos.y < 0 {
                self.pos.y = 0;
                // 弾を消す
                self.live = false;
                self.ban_fire_cnt = Some(15);
                self.explosion_effect_show = true;
                // 自身のx座標が爆発エフェクトの中心になるようずらす
                self.pos.x = self.pos.x - self.explosion_sprite.len() as i32 / 2;
            } else {
                // 移動後の弾の部分に何か物体が存在したら
                if self.is_collide(dot_map) {
                    self.collided(dot_map, ufo, alien, audio);
                }
            }
        } else {
            // 弾が画面上に無く、射撃可能状態で、プレイヤーが生きていて、かつ発射ボタンが押された場合(スペース、Enter)
            if self.ban_fire_cnt == None && player.explosion_cnt == None && input_key.shot {
                self.fire(player.pos.x + 7, player.pos.y - 8);
                // 発射音再生
                if let Some(sound) = &audio.player_shot {
                    audio.play_once_sound(sound);
                }
                // プレイヤーの一つ上の行の判定
                if self.is_collide(dot_map) {
                    self.collided(dot_map, ufo, alien, audio);
                }
            }
        }
        // スコアボーナス
        if 1500 <= self.score && !player.life_up {
            player.life += 1;
            // 残機が増えるのは1度だけ
            player.life_up = true;
        }
        self.draw(dot_map);
    }

    fn collided(&mut self, dot_map: &mut DotMap, ufo: &mut Ufo, alien: &mut Alien, audio: &Audio) {
        // 何かに衝突したので弾を消す
        self.live = false;
        self.ban_fire_cnt = Some(15);
        // 爆発エフェクトを表示する
        self.explosion_effect_show = true;
        // 衝突したのがUFOだった場合
        if self.pos.y / 8 < 2 {
            // UFOの爆発エフェクト表示中でなければ
            if ufo.explosion.show_cnt == None {
                // UFOの撃破時には点数を加算
                self.score += ufo.hit_player_bullet(dot_map, self.fire_cnt, audio);
            }
            // 爆発エフェクトは表示しない
            self.explosion_effect_show = false;
        } else if self.pos.y <= alien.ref_alien_pos.y + 6 {
            // 衝突したのがUFO(の高さ)より下かつ、リファレンスエイリアンより上だった場合のみ
            // エイリアンに当たっていた場合
            if let Some(i) = alien.pos2index(self.pos) {
                // 撃破したエイリアンの点数を追加
                self.score += Alien::index2score(i);
                alien.remove(dot_map, i, audio);
                // 爆発エフェクトは表示しない
                self.explosion_effect_show = false;
            }
        }
        // 自身のx座標が爆発エフェクトの中心になるようずらす
        self.pos.x = self.pos.x - 4;
        self.pos.y += 2;
    }

    // プレイヤーの弾をドットマップに描画(縦方向のバイト境界をまたぐ可能性有り)
    fn draw(&mut self, dot_map: &mut DotMap) {
        if let Some(cnt) = self.ban_fire_cnt {
            self.ban_fire_cnt = if cnt < 0 {
                if self.explosion_effect_show {
                    // 爆発エフェクトを消す
                    self.erase_shifted(dot_map, self.pos);
                }
                None
            } else {
                if self.explosion_effect_show {
                    // 弾の爆発エフェクト表示
                    self.array_shifted_sprite(dot_map);
                }
                Some(cnt - 1)
            };
        }
        // 弾がなければ何もしない
        if !self.live {
            return;
        }
        // 移動後描画する
        self.array_shifted_sprite(dot_map);
    }
}
impl ArraySprite for Bullet {
    fn pos(&self) -> Vec2 {
        self.pos
    }
    fn sprite(&self) -> &[u8] {
        // 描画するのが弾か爆発エフェクトか
        if self.explosion_effect_show {
            &self.explosion_sprite
        } else {
            &self.sprite
        }
    }
}

pub struct Player {
    width: i32,                       // 描画サイズの幅
    pub pos: Vec2,                    // 左上位置
    pre_pos: Vec2,                    // 前回描画時の位置
    pub const_max_explosion_cnt: i32, // 撃破されてから再出撃までのカウント数(定数)
    pub explosion_cnt: Option<i32>,   // Some(再出撃までの残りカウント)
    pub life: i32,
    life_up: bool,   // スコアボーナスで残機が増加済であれば真
    sprite: Vec<u8>, // 左側から縦8ピクセルずつを8bitのベクタで表す
    explosion_sprite: [Vec<u8>; 2],
}
impl Player {
    pub fn new(sprite: Vec<u8>, explosion_sprite1: Vec<u8>, explosion_sprite2: Vec<u8>) -> Self {
        Player {
            width: sprite.len() as i32,
            pos: Vec2::new(8, canvas::GAME_HEIGHT - 8 * 3),
            pre_pos: Vec2::new(8, canvas::GAME_HEIGHT - 8 * 3),
            const_max_explosion_cnt: 160,
            explosion_cnt: None,
            life: 3,
            life_up: false,
            sprite,
            explosion_sprite: [explosion_sprite1, explosion_sprite2],
        }
    }
    pub fn reset_all(&mut self) {
        self.reset_stage();
        self.life = 3;
        self.life_up = false;
    }
    pub fn reset_stage(&mut self) {
        self.pos = Vec2::new(8, canvas::GAME_HEIGHT - 8 * 3);
        self.pre_pos = Vec2::new(8, canvas::GAME_HEIGHT - 8 * 3);
        self.explosion_cnt = None;
    }
    pub fn update(&mut self, dot_map: &mut DotMap, input_key: &KeyDown) {
        self.pre_pos = self.pos;
        // 撃破後、復活前
        if let Some(cnt) = self.explosion_cnt {
            if cnt < 50 {
                // 画像切り替え直後は消す
                if cnt % 5 == 0 {
                    self.erase(dot_map, self.pos);
                }
                self.array_sprite(dot_map);
            } else if cnt == 50 {
                self.erase(dot_map, self.pos);
            } else if self.const_max_explosion_cnt < cnt {
                // 一定時間経過したら復活する
                self.explosion_cnt = None;
                self.pos.x = 8;
                self.life -= 1;
                return;
            }
            // カウントを進める
            self.explosion_cnt = Some(cnt + 1);
            return;
        }

        // プレイヤー移動範囲制限
        if 7 < self.pos.x && input_key.left {
            // 左に移動
            self.pos.x -= 1;
        }
        if self.pos.x + self.width < canvas::GAME_WIDTH - 7 && input_key.right {
            // 右に移動
            self.pos.x += 1;
        }
        self.draw(dot_map);
    }
    // プレイヤーをドットマップに描画(縦方向のバイト境界はまたがない)
    fn draw(&mut self, dot_map: &mut DotMap) {
        if let Some(_) = self.explosion_cnt {
            return;
        }
        // 前回描画した部分を0で消す
        self.erase(dot_map, self.pre_pos);
        // 移動後描画する
        self.array_sprite(dot_map);
    }
    pub fn remove(&mut self, dot_map: &mut DotMap, audio: &Audio) {
        self.explosion_cnt = Some(0);
        self.erase(dot_map, self.pos);
        // 自機撃破音再生
        if let Some(sound) = &audio.player_explosion {
            audio.play_once_sound(sound);
        }
    }
}

impl ArraySprite for Player {
    fn pos(&self) -> Vec2 {
        self.pos
    }
    fn sprite(&self) -> &[u8] {
        // 爆発画像表示中
        if let Some(cnt) = self.explosion_cnt {
            // 5フレーム毎にスプライト切り替え
            if (cnt / 5) % 2 == 0 {
                &self.explosion_sprite[0]
            } else {
                &self.explosion_sprite[1]
            }
        } else {
            &self.sprite
        }
    }
}
