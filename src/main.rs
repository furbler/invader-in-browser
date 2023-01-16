use alien::Alien;
use audio::Audio;
use bottom_area::BottomArea;
use dot_map::DotMap;
use pause::Pause;
use player::{Bullet, Player};
use std::cell::RefCell;
use std::rc::Rc;
use top_area::TopArea;
use ufo::Ufo;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

mod alien;
mod array_sprite;
mod audio;
mod bottom_area;
mod canvas;
mod dot_map;
mod input;
mod math;
mod pause;
mod player;
mod sprite;
mod top_area;
mod ufo;

#[derive(PartialEq)]
enum Scene {
    Title,
    Play,
    Pause,
    LaunchGame(i32),
    LaunchStage(i32),
    ResetStage,
    Gameover(i32),
}

pub enum Msg {
    RetAudio,
    RegisterAudio(Audio),
    AudioVolumeUp,
    AudioVolumeDown,
    AudioVolumeReset,
    ResetCanvas,
    Initialize,
    MainLoop,
}

struct GameCanvas {
    canvas: NodeRef,
    map: DotMap,
    top: TopArea,
    bottom: BottomArea,
    player: Player,
    player_bullet: Bullet,
    alien: Alien,
    alien_bullets: alien::BulletManage,
    ufo: Ufo,
    shield: Vec<u8>,
    stage: usize, // 最初は1、最終は9
    audio: Audio,
    callback: Closure<dyn FnMut()>,
    input_key: Rc<RefCell<input::KeyDown>>,
    // 真の場合、画面全体を赤色にする
    player_exploding: bool,
    pause: Pause,
    scene: Scene,
}

impl Component for GameCanvas {
    type Properties = ();
    type Message = Msg;
    fn create(ctx: &Context<Self>) -> Self {
        let player_data = sprite::ret_dot_data("player");
        let bullet_player_data = sprite::ret_dot_data("bullet_player");
        if bullet_player_data.width != 1 {
            panic!("プレイヤーの弾の幅は1以外は不正です。");
        }
        let player_explosion_1_data = sprite::ret_dot_data("player_explosion_1");
        let player_explosion_2_data = sprite::ret_dot_data("player_explosion_2");
        let player_bullet_explosion_data = sprite::ret_dot_data("player_bullet_explosion");
        let ufo_data = sprite::ret_dot_data("ufo");
        let ufo_explosion_data = sprite::ret_dot_data("ufo_explosion");
        let shield_data = sprite::ret_dot_data("shield");
        let octopus_open_data = sprite::ret_dot_data("octopus_open");
        let octopus_close_data = sprite::ret_dot_data("octopus_close");
        let crab_banzai_data = sprite::ret_dot_data("crab_banzai");
        let crab_down_data = sprite::ret_dot_data("crab_down");
        let squid_open_data = sprite::ret_dot_data("squid_open");
        let squid_close_data = sprite::ret_dot_data("squid_close");
        let alien_explosion_data = sprite::ret_dot_data("alien_explosion");
        let alien_bullet_explosion_data = sprite::ret_dot_data("alien_bullet_explosion");
        let num_data = sprite::char_dot_data();

        // 各構造体初期化
        let player_sprite = player_data.create_dot_map();

        let num_list: Vec<Vec<u8>> = num_data.iter().map(|n| n.create_dot_map()).collect();
        // 画面上部
        let top = top_area::TopArea::new(num_list.clone());
        // メインのゲーム画面
        let map = DotMap::new();
        // 画面下部
        let bottom = bottom_area::BottomArea::new(num_list.clone(), player_sprite.clone());
        let player = Player::new(
            player_sprite.clone(),
            player_explosion_1_data.create_dot_map(),
            player_explosion_2_data.create_dot_map(),
        );
        let player_bullet = Bullet::new(
            bullet_player_data.create_dot_map(),
            player_bullet_explosion_data.create_dot_map(),
        );

        let ufo = Ufo::new(
            ufo_data.create_dot_map(),
            ufo_explosion_data.create_dot_map(),
            num_list.clone(),
        );
        let shield = shield_data.create_dot_map();

        let alien = Alien::new(
            octopus_open_data.create_dot_map(),
            octopus_close_data.create_dot_map(),
            crab_banzai_data.create_dot_map(),
            crab_down_data.create_dot_map(),
            squid_open_data.create_dot_map(),
            squid_close_data.create_dot_map(),
            alien_explosion_data.create_dot_map(),
        );
        let alien_bullets = alien::BulletManage::new(alien_bullet_explosion_data.create_dot_map());

        let comp_ctx = ctx.link().clone();
        let callback = Closure::wrap(
            Box::new(move || comp_ctx.send_message(Msg::MainLoop)) as Box<dyn FnMut()>
        );

        Self {
            canvas: NodeRef::default(),
            top,
            map,
            bottom,
            player,
            player_bullet,
            alien,
            alien_bullets,
            shield,
            ufo,
            audio: Audio::new(),
            input_key: Rc::new(RefCell::new(input::KeyDown {
                left: false,
                right: false,
                shot: false,
                pause: false,
            })),
            stage: 1,
            player_exploding: false,
            scene: Scene::Title,
            pause: Pause::new(),
            callback,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ResetCanvas => {
                // Click Thisボタンを削除
                let document = window().unwrap().document().unwrap();
                let audio_enable_button_element =
                    document.get_element_by_id("parent-audio-button").unwrap();
                audio_enable_button_element.remove();

                ctx.link().send_message(Msg::Initialize);
                true
            }
            // 初期化
            Msg::Initialize => {
                // エイリアンの初期化
                self.alien.reset(self.stage);
                // プレイヤーの初期化
                self.player.reset_all();
                self.ufo.reset();
                // キー入力情報初期化
                input::input_setup(&self.input_key);

                ctx.link().send_message(Msg::RetAudio);
                true
            }
            // 音データを取得
            Msg::RetAudio => {
                if self.audio.invader_move.len() == 0 {
                    ctx.link()
                        .send_future(async { Msg::RegisterAudio(audio::ret_audio().await) });
                }
                false
            }
            // 音データを保存
            Msg::RegisterAudio(audio) => {
                self.audio = audio;
                ctx.link().send_message(Msg::MainLoop);
                false
            }
            Msg::AudioVolumeUp => {
                self.audio.all_volume_up();
                false
            }
            Msg::AudioVolumeDown => {
                self.audio.all_volume_down();
                false
            }
            Msg::AudioVolumeReset => {
                self.audio.reset_volume();
                false
            }
            // ループ
            Msg::MainLoop => {
                self.main_loop();
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let canvas_width = (canvas::ALL_WIDTH * canvas::SCALE).to_string();
        let canvas_height = (canvas::ALL_HEIGHT * canvas::SCALE).to_string();
        html! {
            <div>
                <div id="parent-audio-button">
                    <button id="audio-button" onclick={ctx.link().callback(|_| Msg::ResetCanvas)}>{ "Click This" }</button>
                </div>
            // キャンバスのサイズはここで指定
                <canvas
                    id="canvas"
                    width={canvas_width}
                    height={canvas_height}
                    ref={self.canvas.clone()}/>
                <div class="volume-buttons-list">
                    <button class="volume-button" onclick={ctx.link().callback(|_| Msg::AudioVolumeUp)}>{ "Volume Up" }</button>
                    <button class="volume-button" onclick={ctx.link().callback(|_| Msg::AudioVolumeReset)}>{ "Reset Volume" }</button>
                    <button class="volume-button" onclick={ctx.link().callback(|_| Msg::AudioVolumeDown)}>{ "Volume Down" }</button>
                </div>
            </div>
        }
    }
}

impl GameCanvas {
    fn main_loop(&mut self) {
        let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
        let ctx: CanvasRenderingContext2d =
            canvas.get_context("2d").unwrap().unwrap().unchecked_into();
        // 画像のぼやけを防ぐ
        ctx.set_image_smoothing_enabled(false);
        // 画面全体を背景色(黒)でクリア
        ctx.set_global_alpha(1.);
        ctx.set_fill_style(&JsValue::from("rgb(0,0,0)"));
        ctx.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

        let (top_imagedata, _top_unused) = self.top.dot_map2imagedata(self.player_exploding);
        let (game_imagedata, _game_unused) = self.map.dot_map2imagedata(self.player_exploding);
        let (bottom_imagedata, _bottom_unused) =
            self.bottom.dot_map2imagedata(self.player_exploding);

        // ctx.put_image_data(&top_imagedata, 0., 0.).unwrap();
        ctx.put_image_data(&game_imagedata, 0., canvas::TOP_HEIGHT as _)
            .unwrap();
        ctx.put_image_data(
            &bottom_imagedata,
            0.,
            (canvas::TOP_HEIGHT + canvas::GAME_HEIGHT) as _,
        )
        .unwrap();

        // 得点表示
        self.top.draw_score(self.player_bullet.score);
        // 残機表示
        self.bottom.draw(self.player.life);

        match self.scene {
            Scene::Title => {
                // ショットボタンが押されたら
                if self.input_key.borrow().shot {
                    self.scene = Scene::LaunchGame(10);
                    // 前回のドットマップをすべて消す
                    self.top.all_clear();
                    self.map.all_clear();
                    self.bottom.all_clear();
                }
                // 画像のぼやけを防ぐ
                ctx.set_image_smoothing_enabled(false);
                // 画面全体を背景色(黒)でクリア
                ctx.set_global_alpha(1.);
                ctx.set_fill_style(&JsValue::from("rgb(0,0,0)"));
                ctx.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
                draw_title(&ctx);
            }
            Scene::Play => {
                // Escキーが押されていたらポーズ
                if self.pause.toggle_pause(self.input_key.borrow().pause) {
                    self.scene = Scene::Pause;
                }
                // 更新処理
                self.ufo.update(
                    &mut self.map,
                    self.player_bullet.fire_cnt,
                    self.alien.live_num,
                    &self.audio,
                );

                self.alien
                    .update(&mut self.map, self.player_exploding, &self.audio);
                self.alien_bullets.update(
                    &mut self.map,
                    &mut self.player,
                    &mut self.alien,
                    self.player_bullet.score,
                    &self.audio,
                );

                self.player
                    .update(&mut self.map, &mut self.input_key.borrow());
                self.player_bullet.update(
                    &mut self.map,
                    &mut self.player,
                    &mut self.ufo,
                    &mut self.alien,
                    &self.input_key.borrow(),
                    &self.audio,
                );

                // エイリアンが全滅したら
                if self.alien.live_num <= 0 {
                    // 次のステージへ進む
                    self.scene = Scene::LaunchStage(120);
                }
                // プレイヤーの残機が0またはエイリアンがプレイヤーの高さまで侵攻したら
                if self.player.life <= 0 || self.alien.invaded() {
                    // ゲームオーバー
                    self.scene = Scene::Gameover(120);
                    // 音を止める
                    self.ufo.reset();
                    if self.alien.invaded() {
                        // プレイヤーの高さに降りてきた個体を描く
                        self.alien
                            .update(&mut self.map, self.player_exploding, &self.audio);
                        // エイリアンに侵攻されていたら爆発を起こす
                        self.player.remove(&mut self.map, &self.audio);
                    };
                }
                // プレイヤーが爆発中は画面全体を赤にする
                self.player_exploding = if self.player.explosion_cnt == None {
                    false
                } else {
                    true
                };
            }
            Scene::ResetStage => {
                // ゲーム開始、ステージ開始時共通
                self.scene = Scene::Play;
                // すべて消す
                self.map.all_clear();
                self.top.all_clear();
                self.bottom.all_clear();
                // プレイヤーの下の横線
                self.map.draw_holizon_line(canvas::GAME_HEIGHT - 1);
                // シールド配置
                let shield_width = self.shield.len() / 2;
                for i in 0..4 {
                    let gap = (shield_width + 23) * i;
                    for dx in 0..shield_width {
                        self.map.map[20][gap + 33 + dx] = self.shield[dx];
                    }
                    for dx in 0..shield_width {
                        self.map.map[21][gap + 33 + dx] = self.shield[shield_width + dx];
                    }
                }
                self.alien.reset(self.stage);
                self.alien_bullets.reset();
                self.ufo.reset();
            }
            Scene::LaunchGame(cnt) => {
                // 一定時間経過したらゲーム開始
                if cnt < 0 {
                    self.scene = Scene::ResetStage;

                    self.stage = 1;
                    self.player.reset_all();
                    self.player_bullet.reset_all();
                } else {
                    self.scene = Scene::LaunchGame(cnt - 1);
                }
            }
            Scene::LaunchStage(cnt) => {
                // 一定時間経過したら次のステージ開始
                if cnt < 0 {
                    self.scene = Scene::ResetStage;

                    self.stage += 1;
                    self.player.reset_stage();
                    self.player_bullet.reset_stage();
                } else {
                    self.scene = Scene::LaunchStage(cnt - 1);
                }
            }
            Scene::Gameover(cnt) => {
                // 一定時間経過したらタイトル画面に戻る
                if cnt < 0 {
                    self.scene = Scene::Title;
                    self.player_bullet.score = 0;
                } else {
                    self.scene = Scene::Gameover(cnt - 1);
                    // プレイヤーを爆発させる
                    if let Some(cnt) = self.player.explosion_cnt {
                        if cnt <= self.player.const_max_explosion_cnt {
                            self.player
                                .update(&mut self.map, &mut self.input_key.borrow());
                        }
                    }
                }
                draw_gameover_message(&ctx);
            }
            Scene::Pause => {
                // Escキーが押されていたらポーズ解除
                if self.pause.toggle_pause(self.input_key.borrow().pause) {
                    self.scene = Scene::Play;
                }
                draw_pause(&ctx);
            }
        }

        window()
            .unwrap()
            .request_animation_frame(self.callback.as_ref().unchecked_ref())
            .unwrap();
    }
}

fn draw_title(ctx: &CanvasRenderingContext2d) {
    let ref_pos_x = (canvas::ALL_WIDTH * canvas::SCALE) as f64 / 2.;
    let ref_pos_y = (canvas::ALL_WIDTH * canvas::SCALE) as f64 / 4.;
    ctx.set_font("90px monospace");
    ctx.set_fill_style(&JsValue::from("rgba(200, 10, 10)"));
    ctx.fill_text("Invader", ref_pos_x - 170., ref_pos_y)
        .unwrap();

    ctx.set_font("40px monospace");
    ctx.fill_text("Press Enter", ref_pos_x - 120., ref_pos_y + 80.)
        .unwrap();
}

fn draw_pause(ctx: &CanvasRenderingContext2d) {
    let ref_pos_x = (canvas::ALL_WIDTH * canvas::SCALE) as f64 / 2.;
    let ref_pos_y = (canvas::ALL_WIDTH * canvas::SCALE) as f64 / 4.;
    ctx.set_font("90px monospace");
    ctx.set_fill_style(&JsValue::from("rgba(200, 10, 10)"));
    ctx.fill_text("Pause", ref_pos_x - 170., ref_pos_y).unwrap();

    ctx.set_font("40px monospace");
    ctx.fill_text("Press Escape", ref_pos_x - 120., ref_pos_y + 80.)
        .unwrap();
}
fn draw_gameover_message(ctx: &CanvasRenderingContext2d) {
    let ref_pos_x = (canvas::ALL_WIDTH * canvas::SCALE) as f64 / 2.;
    let ref_pos_y = (canvas::ALL_WIDTH * canvas::SCALE) as f64 / 4.;
    ctx.set_font("90px monospace");
    ctx.set_fill_style(&JsValue::from("rgba(200, 10, 10)"));
    ctx.fill_text("Game over", ref_pos_x - 170., ref_pos_y)
        .unwrap();
}

#[function_component(App)]
fn app_body() -> Html {
    html! {
        <>
            <GameCanvas />
        </>
    }
}

fn main() {
    // デバッグ出力用
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
