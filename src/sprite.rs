pub struct DotShape {
    pub width: i32,              // 幅[ドット]
    pub height: i32,             // 高さ[ドット]
    pub dot_map: Vec<Vec<bool>>, // 描画部分は#、非描画部分は_で表す
}

impl DotShape {
    // 真偽値で表されたスプライトを時計回りに90度回転させたVec<u8>に変換
    pub fn create_dot_map(&self) -> Vec<u8> {
        // 指定されたサイズと実際のドットマップのサイズが一致しているか確認
        if self.height as usize != self.dot_map.len() {
            panic!("指定されたスプライトの高さが実際のデータと異なります。");
        }
        if self.width as usize != self.dot_map[0].len() {
            panic!("指定されたスプライトの幅が実際のデータと異なります。");
        }
        // ドットマップの幅が異なる行が無いか確認
        let map_width = self.dot_map[0].len();
        for l in &self.dot_map {
            if l.len() != map_width {
                panic!("スプライトの形が不正です。");
            }
        }
        // 1列8ピクセルを8bitで表す
        // 元のboolの二次元配列に対し時計回りに90度回転させる
        let mut bytes: Vec<u8>;
        if self.height == 8 {
            bytes = vec![0; self.width as usize];
            for (y, line) in self.dot_map.iter().enumerate() {
                for (x, dot) in line.iter().enumerate() {
                    if *dot {
                        bytes[x] |= 1 << y;
                    }
                }
            }
        } else if self.height == 16 {
            // トーチカの場合
            bytes = vec![0; (self.width * 2) as usize];
            for y in 0..=7 {
                for x in 0..self.width as usize {
                    if self.dot_map[y][x] {
                        bytes[x] |= 1 << y;
                    }
                }
            }
            for y in 8..16 {
                for x in 0..self.width as usize {
                    if self.dot_map[y][x] {
                        bytes[x + self.width as usize] |= 1 << (y - 8);
                    }
                }
            }
        } else {
            panic!("高さは8または16でなければなりません。");
        }
        bytes
    }
}

// ドットデータを変更する際はこの中身のみ変更する
pub fn ret_dot_data(name: &str) -> DotShape {
    let player = DotShape {
        width: 16,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ _ _ _ _ # _ _ _ _ _ _ _",
            "_ _ _ _ _ _ _ # # # _ _ _ _ _ _",
            "_ _ _ _ _ _ _ # # # _ _ _ _ _ _",
            "_ _ _ # # # # # # # # # # # _ _",
            "_ _ # # # # # # # # # # # # # _",
            "_ _ # # # # # # # # # # # # # _",
            "_ _ # # # # # # # # # # # # # _",
            "_ _ # # # # # # # # # # # # # _",
        ]),
    };
    let bullet_player = DotShape {
        width: 1,
        height: 8,
        dot_map: convert_dot_map(vec!["_", "_", "_", "_", "#", "#", "#", "#"]),
    };

    let player_explosion_1 = DotShape {
        width: 16,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ # _ _ _ _ _ _ _ _ _ # _ _",
            "# _ _ _ _ _ # _ _ _ _ # # _ _ #",
            "_ _ _ # _ _ _ _ # # _ _ _ _ _ _",
            "_ _ _ _ _ _ # _ _ _ _ _ _ _ # _",
            "_ # _ _ # _ # # _ _ # # _ _ _ #",
            "_ _ # _ _ _ _ # # # _ _ _ # _ _",
            "_ _ _ # # # # # # # # # _ _ _ _",
            "_ _ # # _ # # # # # # # _ _ # _",
        ]),
    };
    let player_explosion_2 = DotShape {
        width: 16,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ _ _ # _ _ _ _ _ _ _ _ _",
            "_ _ _ _ _ _ _ _ _ _ _ # _ _ _ _",
            "_ _ _ _ _ _ # _ # _ # _ _ _ _ _",
            "_ _ _ # _ _ # _ _ _ _ _ _ _ _ _",
            "_ _ _ _ _ _ _ # # _ # # _ _ _ _",
            "_ # _ _ _ # _ # # _ # _ # _ _ _",
            "_ _ _ # # # # # # # # _ _ # _ _",
            "_ _ # # # # # # # # # # _ # _ #",
        ]),
    };

    let crab_down = DotShape {
        width: 16,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ _ # _ _ _ _ _ # _ _ _ _",
            "_ _ _ _ _ _ # _ _ _ # _ _ _ _ _",
            "_ _ _ _ _ # # # # # # # _ _ _ _",
            "_ _ _ _ # # _ # # # _ # # _ _ _",
            "_ _ _ # # # # # # # # # # # _ _",
            "_ _ _ # _ # # # # # # # _ # _ _",
            "_ _ _ # _ # _ _ _ _ _ # _ # _ _",
            "_ _ _ _ _ _ # # _ # # _ _ _ _ _",
        ]),
    };
    let crab_banzai = DotShape {
        width: 16,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ _ # _ _ _ _ _ # _ _ _ _",
            "_ _ _ # _ _ # _ _ _ # _ _ # _ _",
            "_ _ _ # _ # # # # # # # _ # _ _",
            "_ _ _ # # # _ # # # _ # # # _ _",
            "_ _ _ # # # # # # # # # # # _ _",
            "_ _ _ _ # # # # # # # # # _ _ _",
            "_ _ _ _ _ # _ _ _ _ _ # _ _ _ _",
            "_ _ _ _ # _ _ _ _ _ _ _ # _ _ _",
        ]),
    };

    let octopus_open = DotShape {
        width: 16,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ _ _ # # # # _ _ _ _ _ _",
            "_ _ _ # # # # # # # # # # _ _ _",
            "_ _ # # # # # # # # # # # # _ _",
            "_ _ # # # _ _ # # _ _ # # # _ _",
            "_ _ # # # # # # # # # # # # _ _",
            "_ _ _ _ _ # # _ _ # # _ _ _ _ _",
            "_ _ _ _ # # _ # # _ # # _ _ _ _",
            "_ _ # # _ _ _ _ _ _ _ _ # # _ _",
        ]),
    };

    let octopus_close = DotShape {
        width: 16,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ _ _ # # # # _ _ _ _ _ _",
            "_ _ _ # # # # # # # # # # _ _ _",
            "_ _ # # # # # # # # # # # # _ _",
            "_ _ # # # _ _ # # _ _ # # # _ _",
            "_ _ # # # # # # # # # # # # _ _",
            "_ _ _ _ # # # _ _ # # # _ _ _ _",
            "_ _ _ # # _ _ # # _ _ # # _ _ _",
            "_ _ _ _ # # _ _ _ _ # # _ _ _ _",
        ]),
    };
    let squid_open = DotShape {
        width: 16,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ _ _ _ # # _ _ _ _ _ _ _",
            "_ _ _ _ _ _ # # # # _ _ _ _ _ _",
            "_ _ _ _ _ # # # # # # _ _ _ _ _",
            "_ _ _ _ # # _ # # _ # # _ _ _ _",
            "_ _ _ _ # # # # # # # # _ _ _ _",
            "_ _ _ _ _ _ # _ _ # _ _ _ _ _ _",
            "_ _ _ _ _ # _ # # _ # _ _ _ _ _",
            "_ _ _ _ # _ # _ _ # _ # _ _ _ _",
        ]),
    };
    let squid_close = DotShape {
        width: 16,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ _ _ _ # # _ _ _ _ _ _ _",
            "_ _ _ _ _ _ # # # # _ _ _ _ _ _",
            "_ _ _ _ _ # # # # # # _ _ _ _ _",
            "_ _ _ _ # # _ # # _ # # _ _ _ _",
            "_ _ _ _ # # # # # # # # _ _ _ _",
            "_ _ _ _ _ # _ # # _ # _ _ _ _ _",
            "_ _ _ _ # _ _ _ _ _ _ # _ _ _ _",
            "_ _ _ _ _ # _ _ _ _ # _ _ _ _ _",
        ]),
    };

    let alien_explosion = DotShape {
        width: 16,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ _ # _ _ _ # _ _ _ _ _ _",
            "_ _ # _ _ _ # _ # _ _ _ # _ _ _",
            "_ _ _ # _ _ _ _ _ _ _ # _ _ _ _",
            "_ _ _ _ # _ _ _ _ _ # _ _ _ _ _",
            "_ # # _ _ _ _ _ _ _ _ _ # # _ _",
            "_ _ _ _ # _ _ _ _ _ # _ _ _ _ _",
            "_ _ _ # _ _ # _ # _ _ # _ _ _ _",
            "_ _ # _ _ # _ _ _ # _ _ # _ _ _",
        ]),
    };

    let player_bullet_explosion = DotShape {
        width: 8,
        height: 8,
        dot_map: convert_dot_map(vec![
            "# _ _ _ # _ _ #",
            "_ _ # _ _ _ # _",
            "_ # # # # # # _",
            "# # # # # # # #",
            "# # # # # # # #",
            "_ # # # # # # _",
            "_ _ # _ _ # _ _",
            "# _ _ # _ _ _ #",
        ]),
    };

    let alien_bullet_explosion = DotShape {
        width: 6,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ # _ _ _",
            "# _ _ _ # _",
            "_ _ # # _ #",
            "_ # # # # _",
            "# _ # # # _",
            "_ # # # # #",
            "# _ # # # _",
            "_ # _ # _ #",
        ]),
    };

    let shield = DotShape {
        width: 22,
        height: 16,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ # # # # # # # # # # # # # # _ _ _ _",
            "_ _ _ # # # # # # # # # # # # # # # # _ _ _",
            "_ _ # # # # # # # # # # # # # # # # # # _ _",
            "_ # # # # # # # # # # # # # # # # # # # # _",
            "# # # # # # # # # # # # # # # # # # # # # #",
            "# # # # # # # # # # # # # # # # # # # # # #",
            "# # # # # # # # # # # # # # # # # # # # # #",
            "# # # # # # # # # # # # # # # # # # # # # #",
            "# # # # # # # # # # # # # # # # # # # # # #",
            "# # # # # # # # # # # # # # # # # # # # # #",
            "# # # # # # # # # # # # # # # # # # # # # #",
            "# # # # # # # # # # # # # # # # # # # # # #",
            "# # # # # # # _ _ _ _ _ _ _ # # # # # # # #",
            "# # # # # # _ _ _ _ _ _ _ _ _ # # # # # # #",
            "# # # # # _ _ _ _ _ _ _ _ _ _ _ # # # # # #",
            "# # # # # _ _ _ _ _ _ _ _ _ _ _ # # # # # #",
        ]),
    };

    let ufo = DotShape {
        width: 24,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _",
            "_ _ _ _ _ _ _ _ _ # # # # # # _ _ _ _ _ _ _ _ _",
            "_ _ _ _ _ _ _ # # # # # # # # # # _ _ _ _ _ _ _",
            "_ _ _ _ _ _ # # # # # # # # # # # # _ _ _ _ _ _",
            "_ _ _ _ _ # # _ # # _ # # _ # # _ # # _ _ _ _ _",
            "_ _ _ _ # # # # # # # # # # # # # # # # _ _ _ _",
            "_ _ _ _ _ _ # # # _ _ # # _ _ # # # _ _ _ _ _ _",
            "_ _ _ _ _ _ _ # _ _ _ _ _ _ _ _ # _ _ _ _ _ _ _",
        ]),
    };

    let ufo_explosion = DotShape {
        width: 24,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ # _ _ # _ # _ _ _ _ _ _ # _ # _ _ # _ _ _",
            "_ _ _ _ # _ _ _ _ _ _ _ _ # # _ _ _ _ # _ _ _ _",
            "_ # _ # _ _ _ # # # # _ _ _ # # _ _ _ _ _ _ _ _",
            "_ _ _ _ _ _ # # # # # # # _ _ # # # _ _ # _ _ _",
            "_ _ _ _ _ # # # _ # _ # _ # _ _ # # # _ _ # _ _",
            "_ _ _ # _ _ _ # # # # # _ _ _ # # _ _ _ _ _ _ _",
            "_ # _ _ _ _ _ _ # _ # _ _ _ # # _ _ _ # _ _ _ _",
            "_ _ _ # _ _ _ # _ _ _ # _ _ _ _ # _ _ _ _ _ _ _",
        ]),
    };
    match name {
        "player" => player,
        "bullet_player" => bullet_player,
        "crab_down" => crab_down,
        "crab_banzai" => crab_banzai,
        "octopus_open" => octopus_open,
        "octopus_close" => octopus_close,
        "squid_open" => squid_open,
        "squid_close" => squid_close,
        "alien_explosion" => alien_explosion,
        "alien_bullet_explosion" => alien_bullet_explosion,
        "player_bullet_explosion" => player_bullet_explosion,
        "shield" => shield,
        "ufo" => ufo,
        "ufo_explosion" => ufo_explosion,
        "player_explosion_1" => player_explosion_1,
        "player_explosion_2" => player_explosion_2,
        _ => panic!(
            "{}のドットマップ取得に失敗しました。プログラムを終了します。",
            name
        ), // ドットマップ取得失敗
    }
}
// 文字サイズ(8bit x 8bit)のドットマップを返す
pub fn char_dot_data() -> Vec<DotShape> {
    let mut num = Vec::new();
    // 0
    num.push(DotShape {
        width: 8,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ _ _ _ _",
            "_ _ # # # _ _ _",
            "_ # _ _ _ # _ _",
            "_ # _ _ # # _ _",
            "_ # _ # _ # _ _",
            "_ # # _ _ # _ _",
            "_ # _ _ _ # _ _",
            "_ _ # # # _ _ _",
        ]),
    });
    // 1
    num.push(DotShape {
        width: 8,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ _ _ _ _",
            "_ _ _ # _ _ _ _",
            "_ _ # # _ _ _ _",
            "_ _ _ # _ _ _ _",
            "_ _ _ # _ _ _ _",
            "_ _ _ # _ _ _ _",
            "_ _ _ # _ _ _ _",
            "_ _ # # # _ _ _",
        ]),
    });
    // 2
    num.push(DotShape {
        width: 8,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ _ _ _ _",
            "_ _ # # # _ _ _",
            "_ # _ _ _ # _ _",
            "_ _ _ _ _ # _ _",
            "_ _ _ # # _ _ _",
            "_ _ # _ _ _ _ _",
            "_ # _ _ _ _ _ _",
            "_ # # # # # _ _",
        ]),
    });
    // 3
    num.push(DotShape {
        width: 8,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ _ _ _ _",
            "_ # # # # # _ _",
            "_ _ _ _ _ # _ _",
            "_ _ _ _ # _ _ _",
            "_ _ _ # # _ _ _",
            "_ _ _ _ _ # _ _",
            "_ # _ _ _ # _ _",
            "_ _ # # # _ _ _",
        ]),
    });
    // 4
    num.push(DotShape {
        width: 8,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ _ _ _ _",
            "_ _ _ _ # _ _ _",
            "_ _ _ # # _ _ _",
            "_ _ # _ # _ _ _",
            "_ # _ _ # _ _ _",
            "_ # # # # # _ _",
            "_ _ _ _ # _ _ _",
            "_ _ _ _ # _ _ _",
        ]),
    });
    // 5
    num.push(DotShape {
        width: 8,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ _ _ _ _",
            "_ # # # # # _ _",
            "_ # _ _ _ _ _ _",
            "_ # # # # _ _ _",
            "_ _ _ _ _ # _ _",
            "_ _ _ _ _ # _ _",
            "_ # _ _ _ # _ _",
            "_ _ # # # _ _ _",
        ]),
    });
    // 6
    num.push(DotShape {
        width: 8,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ _ _ _ _",
            "_ _ _ # # # _ _",
            "_ _ # _ _ _ _ _",
            "_ # _ _ _ _ _ _",
            "_ # # # # _ _ _",
            "_ # _ _ _ # _ _",
            "_ # _ _ _ # _ _",
            "_ _ # # # _ _ _",
        ]),
    });
    // 7
    num.push(DotShape {
        width: 8,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ _ _ _ _",
            "_ # # # # # _ _",
            "_ _ _ _ _ # _ _",
            "_ _ _ _ # _ _ _",
            "_ _ _ # _ _ _ _",
            "_ _ # _ _ _ _ _",
            "_ _ # _ _ _ _ _",
            "_ _ # _ _ _ _ _",
        ]),
    });
    // 8
    num.push(DotShape {
        width: 8,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ _ _ _ _",
            "_ _ # # # _ _ _",
            "_ # _ _ _ # _ _",
            "_ # _ _ _ # _ _",
            "_ _ # # # _ _ _",
            "_ # _ _ _ # _ _",
            "_ # _ _ _ # _ _",
            "_ _ # # # _ _ _",
        ]),
    });
    // 9
    num.push(DotShape {
        width: 8,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ _ _ _ _",
            "_ _ # # # _ _ _",
            "_ # _ _ _ # _ _",
            "_ # _ _ _ # _ _",
            "_ _ # # # # _ _",
            "_ _ _ _ _ # _ _",
            "_ _ _ _ # _ _ _",
            "_ # # # _ _ _ _",
        ]),
    });
    num
}

// 描画部分を真、非描画部分を偽とするドットマップを返す
fn convert_dot_map(dot_map: Vec<&str>) -> Vec<Vec<bool>> {
    let mut bool_map = Vec::new();
    for line in dot_map {
        let mut bool_line = Vec::new();
        // 空白をすべて削除
        let space_removed = line.replace(' ', "");
        for c in space_removed.chars() {
            if c == '#' {
                bool_line.push(true);
            } else if c == '_' {
                bool_line.push(false);
            }
        }
        bool_map.push(bool_line);
    }
    bool_map
}
