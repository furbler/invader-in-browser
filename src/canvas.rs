// 画面の幅(文字単位)
const CHAR_WIDTH: i32 = 28;
// 画面の上部分（スコアなどの表示用）のドット単位の大きさ(28文字x4文字)
pub const TOP_WIDTH: i32 = 8 * CHAR_WIDTH;
pub const TOP_HEIGHT: i32 = 8 * 4;
// メインのゲーム画面のドット単位の大きさ(28文字x26文字)
pub const GAME_WIDTH: i32 = 8 * CHAR_WIDTH;
pub const GAME_HEIGHT: i32 = 8 * 26;
// 画面の上部分（スコアなどの表示用）のドット単位の大きさ(28文字x4文字)
pub const BOTTOM_WIDTH: i32 = 8 * CHAR_WIDTH;
pub const BOTTOM_HEIGHT: i32 = 8 * 2;
// キャンバス全体のドット単位の大きさ
pub const ALL_WIDTH: i32 = TOP_WIDTH;
pub const ALL_HEIGHT: i32 = TOP_HEIGHT + GAME_HEIGHT + BOTTOM_HEIGHT;

// 1ドットを何ピクセル四方で表示するか(pixel / dot)
pub const SCALE: i32 = 3;
