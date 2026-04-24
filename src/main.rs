use macroquad::prelude::*;

static mut MAP: [i32; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0]; // player : 1, ai : 2

fn drawmap() {
    draw_line(215.0, 100.0, 215.0, 730.0, 30.0, BLACK);
    draw_line(415.0, 100.0, 415.0, 730.0, 30.0, BLACK);
    draw_line(0.0, 315.0, 630.0, 315.0, 30.0, BLACK);
    draw_line(0.0, 515.0, 630.0, 515.0, 30.0, BLACK);
    
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
        window_width: 630,
        window_height: 830,
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