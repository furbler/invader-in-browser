use crate::array_sprite::ArraySprite;
use crate::audio::Audio;
use crate::canvas;
use crate::dot_map::DotMap;
use crate::math::Vec2;
use crate::player::Player;

enum BulletType {
    Squiggly, // ジグザグ型
    Plunger,  // 十字架型(ピストン型)
    Rolling,  // ねじ型
}

struct Bullet {
    pos: Vec2,
    btype: BulletType,
    live: bool,      // 弾が画面上にある場合は真
    flying_cnt: i32, // 弾が発射されてからの経過カウント
    speed: i32,      // 移動速度(移動量)
    // 爆発エフェクトの残り表示カウント
    explosion_cnt: Option<i32>,
    // 種類によらず、サイズは3x8ドット
    sprite: [u8; 3],
    // 爆発エフェクトスプライト
    explosion_sprite: Vec<u8>,
}

impl Bullet {
    fn new(btype: BulletType, explosion_sprite: Vec<u8>) -> Self {
        Bullet {
            btype,
            pos: Vec2::new(0, 0),
            live: false,
            flying_cnt: 0,
            speed: 0,
            explosion_cnt: None,
            sprite: [0; 3],
            explosion_sprite,
        }
    }
    fn fire(&mut self, alien_pos: Vec2, speed: i32) {
        // 発射するエイリアンより少し下から発射する
        self.pos = Vec2::new(alien_pos.x + 8, alien_pos.y + 16);
        self.live = true;
        self.flying_cnt = 0;
        self.speed = speed;
    }
    fn update(&mut self, dot_map: &mut DotMap, player: &mut Player, audio: &Audio) {
        if self.live {
            // 弾が飛翔中
            self.flying_cnt += 1;
        } else {
            // 爆発エフェクト表示中
            if let Some(cnt) = self.explosion_cnt {
                self.explosion_cnt = if cnt < 0 {
                    // カウント終了したら爆発エフェクトを消す
                    self.erase_shifted(dot_map, self.pos);
                    None
                } else {
                    // カウントを進める
                    Some(cnt - 1)
                }
            }
            return;
        }
        // 前回の描画を消す
        self.erase_shifted(dot_map, self.pos);
        // 移動
        self.pos.y += self.speed;
        // スプライトを更新
        match self.btype {
            BulletType::Squiggly => self.update_squiggly_sprite(self.pos.y),
            BulletType::Plunger => self.update_plunger_sprite(self.pos.y),
            BulletType::Rolling => self.update_rolling_sprite(self.pos.y),
        }
        // 赤線に着弾
        if canvas::GAME_HEIGHT - 1 <= self.pos.y + 7 {
            // はみださないようにする
            self.pos.y = canvas::GAME_HEIGHT - 8;
            self.pos.x -= 3;
            self.create_explosion_effect(dot_map);
            return;
        }
        // 何かに衝突した場合
        if self.is_collide(dot_map) {
            // プレイヤーのいる高さの範囲内に弾が入っている
            if canvas::GAME_HEIGHT - 8 * 3 < self.pos.y + 8
                && self.pos.y < canvas::GAME_HEIGHT - 8 * 2
            {
                // プレイヤーが爆発中でなければ
                if player.explosion_cnt == None {
                    // プレイヤーを破壊する
                    player.remove(dot_map, audio);
                }
            }
            self.pos.x -= 3;
            self.pos.y += 3;
            self.create_explosion_effect(dot_map);
            return;
        }
    }
    fn draw(&self, dot_map: &mut DotMap) {
        if !self.live || self.explosion_cnt != None {
            return;
        }
        self.array_shifted_sprite(dot_map);
    }
    // エフェクトを設置する
    fn create_explosion_effect(&mut self, dot_map: &mut DotMap) {
        self.live = false;
        self.explosion_cnt = Some(15);
        self.array_shifted_sprite(dot_map);
    }
    fn update_rolling_sprite(&mut self, pos_y: i32) {
        // 真ん中は常に描く
        self.sprite[0] = 0;
        self.sprite[1] = 0b1111_1111;
        self.sprite[2] = 0;

        // i = 0..20
        let i = (pos_y as usize % (20 * 3)) / 3;

        if i < 8 {
            // スラッシュ
            self.sprite[0] = 0b10010000 >> i;
            self.sprite[2] = 0b01001000 >> i;
        } else if 12 <= i {
            // バックスラッシュ
            self.sprite[0] = 0b01001000 >> (i - 12);
            self.sprite[2] = 0b10010000 >> (i - 12);
        }
    }
    fn update_plunger_sprite(&mut self, pos_y: i32) {
        // 真ん中は常に描く
        self.sprite[0] = 0;
        self.sprite[1] = 0b1111_1111;
        self.sprite[2] = 0;
        // i = 0..8
        let i = (pos_y as usize % 24) / 3;

        self.sprite[0] |= 1 << (7 - i);
        self.sprite[2] |= 1 << (7 - i);
    }
    fn update_squiggly_sprite(&mut self, pos_y: i32) {
        // 0クリア
        for i in 0..3 {
            self.sprite[i] = 0;
        }
        // i = 0..4
        let mut i = (pos_y as usize % 12) / 3;
        let table = [2, 1, 0, 1];
        for y in 1..8 {
            self.sprite[table[i]] |= 1 << y;
            i = (i + 1) % 4;
        }
    }
}
impl ArraySprite for Bullet {
    fn pos(&self) -> Vec2 {
        self.pos
    }
    fn sprite(&self) -> &[u8] {
        // 描画するのが弾か爆発エフェクトか
        if self.explosion_cnt == None {
            &self.sprite
        } else {
            &self.explosion_sprite
        }
    }
}

struct TableManage {
    // 次に利用すべき値のインデックス番号
    i: usize,
    table: Vec<usize>,
}
impl TableManage {
    // 前回利用した表の値の次の値を返す
    fn take(&mut self) -> usize {
        let value = self.table[self.i];
        self.i = if self.table.len() <= self.i + 1 {
            0
        } else {
            self.i + 1
        };
        value
    }
}

pub struct BulletManage {
    // ジグザグ型
    // bulets[0]: squiggly
    // 十字架型(ピストン型)
    // bulets[1]: plunger
    // ねじ型(プレイヤーを狙う)
    // bulets[2]: rolling
    bullets: Vec<Bullet>,
    // 発射列表
    plunger_shot_column_table: TableManage,
    squiggly_shot_column_table: TableManage,
    // 弾を発射して、その弾が消える前に別の弾の発射許可を出す時間間隔の最低値
    reload_cnt: i32,
    // 弾の移動速度(移動量)
    speed: i32,
    // 射撃禁止状態の残りカウント
    ban_fire_cnt: Option<i32>,
}
impl BulletManage {
    pub fn new(explosion_sprite: Vec<u8>) -> Self {
        let mut bullets = Vec::new();
        bullets.push(Bullet::new(BulletType::Rolling, explosion_sprite.clone()));
        bullets.push(Bullet::new(BulletType::Plunger, explosion_sprite.clone()));
        bullets.push(Bullet::new(BulletType::Squiggly, explosion_sprite.clone()));

        BulletManage {
            bullets,
            plunger_shot_column_table: TableManage {
                i: 0,
                table: vec![1, 7, 1, 1, 1, 4, 11, 1, 6, 3, 1, 1, 11, 9, 2, 8]
                    .iter()
                    .map(|x| x - 1)
                    .collect(),
            },
            squiggly_shot_column_table: TableManage {
                i: 0,
                table: vec![11, 1, 6, 3, 1, 1, 11, 9, 2, 8, 2, 11, 4, 7, 10]
                    .iter()
                    .map(|x| x - 1)
                    .collect(),
            },
            reload_cnt: 48, // 0x30 * 1.5
            speed: 1,
            ban_fire_cnt: None,
        }
    }
    pub fn reset(&mut self) {
        // 開始から一定時間は発射しない
        self.ban_fire_cnt = Some(120);
        for b in self.bullets.iter_mut() {
            b.live = false;
        }
    }
    pub fn update(
        &mut self,
        dot_map: &mut DotMap,
        player: &mut Player,
        alien: &Alien,
        score: i32,
        audio: &Audio,
    ) {
        // 獲得点数に応じて発射頻度を変える
        self.set_reload_cnt(score);
        if let Some(cnt) = self.ban_fire_cnt {
            if cnt < 0 {
                self.ban_fire_cnt = None;
            } else {
                self.ban_fire_cnt = Some(cnt - 1);
            }
        } else {
            // プレイヤーが爆発していたら
            if let Some(cnt) = player.explosion_cnt {
                // プレイヤーの爆発が終わってから一定時間は発射しない
                self.ban_fire_cnt = Some(cnt + 80);
            } else if alien.explosion.effect_cnt == None {
                // プレイヤーとエイリアンのどちらも爆発中でない場合に発射処理を行う
                self.which_fire(player, alien);
            }
        }
        for i in 0..self.bullets.len() {
            self.bullets[i].update(dot_map, player, audio);
        }
        self.draw(dot_map);
    }
    fn draw(&self, dot_map: &mut DotMap) {
        for i in 0..self.bullets.len() {
            self.bullets[i].draw(dot_map);
        }
    }
    // どのエイリアンがどの種類の弾を撃つか決める
    fn which_fire(&mut self, player: &Player, alien: &Alien) {
        let seed = (player.pos.x + alien.ref_alien_pos.x).abs() as usize % 3;
        // 自身が画面上に無く、かつ他2種の弾が発射してから一定時間経過した後
        // rolling shot(自機を狙う弾)
        if seed == 0 && !self.bullets[seed].live && self.bullets[seed].explosion_cnt == None {
            if (!self.bullets[1].live || self.reload_cnt < self.bullets[1].flying_cnt)
                && (!self.bullets[2].live || self.reload_cnt < self.bullets[2].flying_cnt)
            {
                // プレイヤーに近い列のエイリアンに生き残りがいたら
                if let Some(i) = alien.alien_index_near_x(player.pos.x) {
                    // そのエイリアンからrolling shot(自機を狙う)発射
                    self.bullets[seed].fire(alien.index2pos(i), self.speed);
                }
            }
        } else if seed == 1 && !self.bullets[seed].live && self.bullets[seed].explosion_cnt == None
        {
            // 自身が画面上に無く、かつ他2種の弾が発射してから一定時間経過した後
            // plunger shot(十字架、ピストン弾)
            if (!self.bullets[0].live || self.reload_cnt < self.bullets[0].flying_cnt)
                && (!self.bullets[2].live || self.reload_cnt < self.bullets[2].flying_cnt)
            {
                if let Some(i) = alien.column2index(self.plunger_shot_column_table.take()) {
                    // plunger shot発射
                    self.bullets[seed].fire(alien.index2pos(i), self.speed);
                }
            }
        } else if !self.bullets[seed].live && self.bullets[seed].explosion_cnt == None {
            // 自身が画面上に無く、かつ他2種の弾が発射してから一定時間経過した後
            if (!self.bullets[0].live || self.reload_cnt < self.bullets[0].flying_cnt)
                && (!self.bullets[1].live || self.reload_cnt < self.bullets[1].flying_cnt)
            {
                if let Some(i) = alien.column2index(self.squiggly_shot_column_table.take()) {
                    // squiggly shot発射
                    self.bullets[seed].fire(alien.index2pos(i), self.speed);
                }
            }
        }
    }
    fn set_reload_cnt(&mut self, score: i32) {
        self.reload_cnt = match score {
            0..=200 => 48,
            201..=1600 => 32,  // 16 x 2
            1601..=3200 => 22, // 11 x 2
            3201..=4800 => 16, // 8 x 2
            4801.. => 14,      // 7 x 2
            _ => 0,
        }
    }
}

struct Explosion {
    pos: Vec2,
    // 爆発エフェクトのスプライト
    sprite: Vec<u8>,
    //エフェクト表示中はSome(カウント)
    effect_cnt: Option<i32>,
}

impl Explosion {
    // エフェクトを設置する
    fn create_effect(&mut self, dot_map: &mut DotMap, pos: Vec2) {
        self.pos = pos;
        self.effect_cnt = Some(15);
        self.array_sprite(dot_map);
    }
    fn update(&mut self, dot_map: &mut DotMap) {
        // エフェクトが表示されていたら
        if let Some(cnt) = self.effect_cnt {
            // カウントが終わったら
            if cnt < 0 {
                // エフェクト削除
                self.effect_cnt = None;
                self.erase(dot_map, self.pos);
            } else {
                self.effect_cnt = Some(cnt - 1);
            }
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

pub struct Alien {
    // リファレンスエイリアンの座標
    pub ref_alien_pos: Vec2,
    // リファレンスエイリアンの現在位置へ動く一つ前の位置
    pre_ref_alien_pos: Vec2,
    // 描画するスプライト
    show_sprite: bool,
    // スプライトのリスト
    sprite_list: Vec<Vec<u8>>,
    explosion: Explosion,
    // 描画処理対象のインデックス番号
    i_cursor_alien: usize,
    // エイリアンの移動量
    speed: Vec2,
    // エイリアンの生存状態
    live: Vec<bool>,
    // 生きているエイリアンの数
    pub live_num: i32,
    // ステージ2から9までのリファレンスエイリアンの位置
    table_init_pos_y: Vec<i32>,
    // 再生するサウンドのインデックス番号
    se_index: usize,
    // 前回音を再生してからのtick数
    se_interval: i32,
}

impl Alien {
    pub fn new(
        // 下2列のエイリアンのスプライト
        low_sprite0: Vec<u8>,
        low_sprite1: Vec<u8>,
        // 中2列のエイリアンのスプライト
        middle_sprite0: Vec<u8>,
        middle_sprite1: Vec<u8>,
        // 上1列のエイリアンのスプライト
        high_sprite0: Vec<u8>,
        high_sprite1: Vec<u8>,
        // 爆発エフェクトのスプライト
        explosion_sprite: Vec<u8>,
    ) -> Self {
        let mut sprite_list = Vec::new();
        sprite_list.push(low_sprite0);
        sprite_list.push(low_sprite1);
        sprite_list.push(middle_sprite0);
        sprite_list.push(middle_sprite1);
        sprite_list.push(high_sprite0);
        sprite_list.push(high_sprite1);
        let table_init_pos_y: Vec<i32> = vec![88, 72, 64, 64, 64, 56, 56, 56]
            .iter()
            .map(|y| canvas::GAME_HEIGHT - y)
            .collect();
        if table_init_pos_y.len() != 8 {
            panic!("エイリアンの初期位置表の要素数は8にしてください。");
        }
        Alien {
            ref_alien_pos: Vec2::new(0, 0),
            pre_ref_alien_pos: Vec2::new(0, 0),
            show_sprite: true,
            sprite_list,
            explosion: Explosion {
                pos: Vec2::new(0, 0),
                sprite: explosion_sprite,
                effect_cnt: None,
            },
            i_cursor_alien: 0,
            speed: Vec2::new(2, 0),
            live: vec![true; 55],
            live_num: 55,
            table_init_pos_y,
            se_index: 0,
            se_interval: 0,
        }
    }
    // エイリアンを初期化する
    pub fn reset(&mut self, stage: usize) {
        self.live = vec![true; 55];
        self.live_num = 55;
        self.show_sprite = true;
        self.i_cursor_alien = 0;
        self.speed = Vec2::new(2, 0);
        self.se_index = 0;
        self.se_interval = 0;

        // ステージ数によって初期位置が決まる
        self.ref_alien_pos.x = 24;
        self.ref_alien_pos.y = if 1 < stage {
            self.table_init_pos_y[(stage - 2) % 8]
        } else {
            canvas::GAME_HEIGHT - 112
        };
        self.pre_ref_alien_pos = self.ref_alien_pos;
    }
    pub fn update(&mut self, dot_map: &mut DotMap, player_exploding: bool, audio: &Audio) {
        // プレイヤーまたはエイリアンが爆発中はすべてのエイリアンを停止させる
        if player_exploding || self.explosion.effect_cnt != None {
            self.explosion.update(dot_map);
            return;
        }
        if self.live_num <= 0 {
            return;
        }

        if self.live[self.i_cursor_alien] {
            // カーソルエイリアンの移動後の真上の（16 x 8）ドットの範囲を消す
            let mut alien_top_pos = self.index2pos(self.i_cursor_alien);
            alien_top_pos.y -= 8;
            self.erase(dot_map, alien_top_pos);
            // 移動後を描画する
            self.array_sprite(dot_map);
        }

        // 処理対象カーソルを進める
        self.i_cursor_alien += 1;
        while self.i_cursor_alien < 55 {
            if self.live[self.i_cursor_alien] {
                break;
            }
            self.i_cursor_alien += 1;
        }

        if self.i_cursor_alien == 55 {
            self.i_cursor_alien = 0;
            // もう一巡、処理対象カーソルを進める
            while self.i_cursor_alien < 55 {
                if self.live[self.i_cursor_alien] {
                    break;
                }
                self.i_cursor_alien += 1;
            }
            if self.i_cursor_alien == 55 {
                println!("エイリアンは全滅した。");
                return;
            }
            // 一巡後、エイリアンのどれかが両側の折り返し地点に到達していたら反転する
            self.check_bump_side(dot_map);

            // 一巡したら描画するスプライトを切り替える
            self.show_sprite = !self.show_sprite;
            // 移動前のリファレンスエイリアンの座標を保存する
            self.pre_ref_alien_pos = self.ref_alien_pos;
            // リファレンスエイリアンを移動させる
            self.ref_alien_pos += self.speed;
            // カーソルエイリアン(に一番近い個体)が動いた時に侵攻音再生
            // 音が保存されており、前回の再生から5tick経過していた時のみ再生
            if 0 < audio.invader_move.len() && 4 < self.se_interval {
                audio.play_once_sound(&audio.invader_move[self.se_index]);
                self.se_index = (self.se_index + 1) % 4;
                self.se_interval = 0;
            }
        }
        self.se_interval += 1;
    }
    // 一番下のエイリアンがプレイヤーの高さまで侵攻したら真を返す
    pub fn invaded(&self) -> bool {
        canvas::GAME_HEIGHT - 24 <= self.index2pos(self.i_cursor_alien).y
    }
    // 何かの物体が両側の折り返し地点に到達していたら真を返す
    fn check_bump_side(&mut self, dot_map: &DotMap) {
        // 判定する壁の高さはUFOの下からプレイヤーの上まで
        for char_y in 2..23 {
            // 右の壁のドットに何かが存在したら
            if dot_map.map[char_y][213] != 0 {
                self.speed = Vec2::new(-2, 8);
                return;
            }
        }
        for char_y in 2..23 {
            // 左の壁のドットに何かが存在したら
            if dot_map.map[char_y][9] != 0 {
                // エイリアンが最後の1匹のときは速度を上げる
                self.speed = if self.live_num == 1 {
                    Vec2::new(3, 8)
                } else {
                    Vec2::new(2, 8)
                };
                return;
            }
        }
        // 折り返しでなければdyは0にする
        self.speed.y = 0;
    }
    // インデックス番号で指定されたエイリアンを消す
    pub fn remove(&mut self, dot_map: &mut DotMap, i: usize, audio: &Audio) {
        self.live[i] = false;
        let width = self.sprite_list[2 * Alien::ret_alien_type(i)].len();
        let alien_pos = self.index2pos(i);

        let char_y = (alien_pos.y / 8) as usize;
        for dx in 0..width {
            dot_map.map[char_y][alien_pos.x as usize + dx] = 0;
        }

        // 爆発エフェクト描画
        self.explosion.create_effect(dot_map, alien_pos);
        self.live_num -= 1;
        // 爆発音再生
        if let Some(sound) = &audio.invader_explosion {
            audio.play_once_sound(sound);
        }
    }
    // プレイヤーの弾の座標を引数として、エイリアンに当たった場合はそのエイリアンのインデックス番号を返す
    pub fn pos2index(&self, mut pos: Vec2) -> Option<usize> {
        let mut ref_pos = self.ref_alien_pos;
        // リファレンスエイリアン移動時のずれを考慮し、左に2ドットずらす
        ref_pos.x -= 2;
        ref_pos.y += 4;
        // リファレンスエイリアンより左側の場合
        if pos.x < ref_pos.x {
            // エイリアンには当たっていない
            return None;
        }
        // 計算を簡単にするため左下座標にする
        ref_pos.y += 8;
        pos.y += 8;

        let row = (ref_pos.y - pos.y) / 16;
        let column = (ref_pos.x - pos.x).abs() / 16;
        // エイリアンの隊列の外側の場合
        if 11 <= column || 5 <= row {
            // エイリアンには当たっていない
            return None;
        }
        let index = (row * 11 + column) as usize;
        // 該当エイリアンの生存判定
        if self.live[index] {
            Some((row * 11 + column) as usize)
        } else {
            None
        }
    }
    // 指定したx座標に一番近い列の一番下のエイリアンのインデックス番号を、全滅していたらNoneを返す
    fn alien_index_near_x(&self, pos_x: i32) -> Option<usize> {
        // リファレンスエイリアンより左側の場合
        if pos_x < self.ref_alien_pos.x {
            return self.column2index(0);
        }
        let mut column = (pos_x - self.ref_alien_pos.x) as usize / 16;
        if 10 < column {
            column = 10
        };
        self.column2index(column)
    }
    // 列番号(0..11)のエイリアンが存在していたら一番下の個体のインデックス番号を、全滅していたらNoneを返す
    fn column2index(&self, column: usize) -> Option<usize> {
        let mut i = column;

        while i < 55 {
            if self.live[i] {
                return Some(i);
            }
            i += 11;
        }
        None
    }

    // エイリアンのインデックス番号から座標を返す
    fn index2pos(&self, i: usize) -> Vec2 {
        let dx = i as i32 % 11;
        let dy = i as i32 / 11;
        // リファレンスエイリアンと同期済
        if i <= self.i_cursor_alien {
            Vec2::new(
                self.ref_alien_pos.x + 16 * dx,
                self.ref_alien_pos.y - 16 * dy,
            )
        } else {
            // リファレンスエイリアンとずれている
            Vec2::new(
                self.pre_ref_alien_pos.x + 16 * dx,
                self.pre_ref_alien_pos.y - 16 * dy,
            )
        }
    }
    pub fn index2score(i: usize) -> i32 {
        match Alien::ret_alien_type(i) {
            0 => 10,
            1 => 20,
            2 => 30,
            _ => panic!("エイリアンを指すインデックス番号が不正です。"),
        }
    }
    // インデックス番号から、下2段は0、中2段は1、上1段は2を返す
    fn ret_alien_type(i: usize) -> usize {
        let row = i / 11;
        match row {
            0 | 1 => 0,
            2 | 3 => 1,
            4 => 2,
            _ => panic!("エイリアンを指すインデックス番号が不正です。"),
        }
    }
}

impl ArraySprite for Alien {
    fn pos(&self) -> Vec2 {
        // カーソルエイリアンの座標
        self.index2pos(self.i_cursor_alien)
    }
    // カーソルエイリアンのスプライト
    fn sprite(&self) -> &[u8] {
        // 2つの状態のスプライトのどちらを描画するか
        let sprite_type: usize = if self.show_sprite { 0 } else { 1 };
        &self.sprite_list[2 * Alien::ret_alien_type(self.i_cursor_alien) + sprite_type]
    }
}
