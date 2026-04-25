use macroquad::prelude::*;

static mut MAP: [i32; 9] = [1, 0, 0, 2, 0, 1, 1, 2, 1]; // player : 1, ai : 2

fn draw_plrs(coords: (i32, i32), w: bool) { // player : true, ai : false
    if w {
        let new_coord: (f32, f32) = (coords.0 as f32 + 20.0, coords.1 as f32 + 20.0);
        draw_line(new_coord.0, new_coord.1, new_coord.0 + 160.0, new_coord.1 + 160.0, 10.0, ORANGE);
        draw_line(new_coord.0 + 160.0, new_coord.1, new_coord.0, new_coord.1 + 160.0, 10.0, ORANGE);
    } else {
        let new_coord: (f32, f32) = (coords.0 as f32 + 100.0, coords.1 as f32 + 100.0);
        draw_circle_lines(new_coord.0, new_coord.1, 80.0, 10.0, BLUE);
    }
}

fn drawmap() {
    draw_line(215.0, 100.0, 215.0, 760.0, 30.0, BLACK);
    draw_line(445.0, 100.0, 445.0, 760.0, 30.0, BLACK);
    draw_line(0.0, 315.0, 660.0, 315.0, 30.0, BLACK);
    draw_line(0.0, 545.0, 660.0, 545.0, 30.0, BLACK);
    draw_rectangle(0.0, 0.0, 660.0, 100.0, LIGHTGRAY);
    draw_rectangle(0.0, 760.0, 660.0, 100.0, LIGHTGRAY);
    unsafe {
        let mut i: i32 = 0;
        for spot in MAP {
            match i {
                0 => {match spot {
                    0 => {},
                    1 => {draw_plrs((0, 100), true);},
                    2 => {draw_plrs((0, 100), false);},
                    _ => {println!("prblème au niveau de la MAP")}
                }},
                1 => {match spot {
                    0 => {},
                    1 => {draw_plrs((230, 100), true);},
                    2 => {draw_plrs((230, 100), false);},
                    _ => {println!("prblème au niveau de la MAP")}
                }},
                2 => {match spot {
                    0 => {},
                    1 => {draw_plrs((460, 100), true);},
                    2 => {draw_plrs((460, 100), false);},
                    _ => {println!("prblème au niveau de la MAP")}
                }},
                3 => {match spot {
                    0 => {},
                    1 => {draw_plrs((0, 330), true);},
                    2 => {draw_plrs((0, 330), false);},
                    _ => {println!("prblème au niveau de la MAP")}
                }},
                4 => {match spot {
                    0 => {},
                    1 => {draw_plrs((230, 330), true);},
                    2 => {draw_plrs((230, 330), false);},
                    _ => {println!("prblème au niveau de la MAP")}
                }},
                5 => {match spot {
                    0 => {},
                    1 => {draw_plrs((460, 330), true);},
                    2 => {draw_plrs((460, 330), false);},
                    _ => {println!("prblème au niveau de la MAP")}
                }},
                6 => {match spot {
                    0 => {},
                    1 => {draw_plrs((0, 560), true);},
                    2 => {draw_plrs((0, 560), false);},
                    _ => {println!("prblème au niveau de la MAP")}
                }},
                7 => {match spot {
                    0 => {},
                    1 => {draw_plrs((230, 560), true);},
                    2 => {draw_plrs((230, 560), false);},
                    _ => {println!("prblème au niveau de la MAP")}
                }},
                8 => {match spot {
                    0 => {},
                    1 => {draw_plrs((460, 560), true);},
                    2 => {draw_plrs((460, 560), false);},
                    _ => {println!("prblème au niveau de la MAP")}
                }},
                _ => {}
            }
            i += 1;
        }
    }
}

fn chng_map(coords: (i32, i32), plr: i32) {
    unsafe {
        match plr {
            1 => {match coords {
                (1, 1) => {if MAP[0] == 0 {MAP[0] = 1;}},
                (1, 2) => {if MAP[1] == 0 {MAP[1] = 1;}},
                (1, 3) => {if MAP[2] == 0 {MAP[2] = 1;}},
                (2, 1) => {if MAP[3] == 0 {MAP[3] = 1;}},
                (2, 2) => {if MAP[4] == 0 {MAP[4] = 1;}},
                (2, 3) => {if MAP[5] == 0 {MAP[5] = 1;}},
                (3, 1) => {if MAP[6] == 0 {MAP[6] = 1;}},
                (3, 2) => {if MAP[7] == 0 {MAP[7] = 1;}},
                (3, 3) => {if MAP[8] == 0 {MAP[8] = 1;}},
                _ => {println!("défaut d'entrée des coords")}
            }},
            2 => {match coords {
                (1, 1) => {if MAP[0] == 0 {MAP[0] = 2;}},
                (1, 2) => {if MAP[1] == 0 {MAP[1] = 2;}},
                (1, 3) => {if MAP[2] == 0 {MAP[2] = 2;}},
                (2, 1) => {if MAP[3] == 0 {MAP[3] = 2;}},
                (2, 2) => {if MAP[4] == 0 {MAP[4] = 2;}},
                (2, 3) => {if MAP[5] == 0 {MAP[5] = 2;}},
                (3, 1) => {if MAP[6] == 0 {MAP[6] = 2;}},
                (3, 2) => {if MAP[7] == 0 {MAP[7] = 2;}},
                (3, 3) => {if MAP[8] == 0 {MAP[8] = 2;}},
                _ => {println!("défaut d'entrée des coords")}
            }},
            _ => {println!("défaut d'entrée du joueur")}
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Tic Tac Toe".to_owned(),
        window_width: 660,
        window_height: 860,
        window_resizable: false,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    loop {
        // 1. Nettoyer l'écran à chaque frame (l'équivalent de screen.fill en Pygame)
        clear_background(WHITE);

        drawmap();

        // 3. Attendre la frame suivante (l'équivalent de pygame.display.flip())
        next_frame().await
    }
}