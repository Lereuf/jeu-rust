use macroquad::prelude::*;

static mut MAP: [i32; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0]; // player : 1, ai : 2

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

// Renvoie true si le joueur a cliqué sur une case valide, false sinon
fn plr_play(coords: (f32, f32)) -> bool {
    if coords.0 > 0.0 && coords.0 < 200.0 && coords.1 > 100.0 && coords.1 < 300.0 { return chng_map((1, 1), 1); }
    if coords.0 > 230.0 && coords.0 < 430.0 && coords.1 > 100.0 && coords.1 < 300.0 { return chng_map((1, 2), 1); }
    if coords.0 > 460.0 && coords.0 < 660.0 && coords.1 > 100.0 && coords.1 < 300.0 { return chng_map((1, 3), 1); }
    if coords.0 > 0.0 && coords.0 < 200.0 && coords.1 > 330.0 && coords.1 < 530.0 { return chng_map((2, 1), 1); }
    if coords.0 > 230.0 && coords.0 < 430.0 && coords.1 > 330.0 && coords.1 < 530.0 { return chng_map((2, 2), 1); }
    if coords.0 > 460.0 && coords.0 < 660.0 && coords.1 > 330.0 && coords.1 < 530.0 { return chng_map((2, 3), 1); }
    if coords.0 > 0.0 && coords.0 < 200.0 && coords.1 > 560.0 && coords.1 < 760.0 { return chng_map((3, 1), 1); }
    if coords.0 > 230.0 && coords.0 < 430.0 && coords.1 > 560.0 && coords.1 < 760.0 { return chng_map((3, 2), 1); }
    if coords.0 > 460.0 && coords.0 < 660.0 && coords.1 > 560.0 && coords.1 < 760.0 { return chng_map((3, 3), 1); }
    
    false // Si on a cliqué en dehors des cases
}

fn in_game_print(what: String) {
    draw_text(&what, 50.0, 75.0, 40.0, BLACK);
}

fn rnd_ai_play() {
    unsafe {
        loop {
            // Tire un index entre 0 inclus et 9 exclu (donc de 0 à 8)
            let idx = rand::gen_range(0, 9); 
            if MAP[idx] == 0 {
                MAP[idx] = 2;
                break; // Très important : on sort de la boucle une fois le pion posé !
            }
        }
    }
}

// Renvoie true si l'IA a posé un pion
fn ai_check_around(i: i32) -> bool {
    unsafe {
        match i {
            0 => {
                if MAP[1] == 0 { return chng_map((2, 1), 2); } 
                else if MAP[3] == 0 { return chng_map((3, 1), 2); } 
                else if MAP[4] == 0 { return chng_map((2, 2), 2); }
            }
            1 => {
                if MAP[0] == 0 { return chng_map((1, 1), 2); } 
                else if MAP[2] == 0 { return chng_map((1, 3), 2); } 
                else if MAP[4] == 0 { return chng_map((2, 2), 2); }
            }
            2 => {
                if MAP[1] == 0 { return chng_map((2, 1), 2); } 
                else if MAP[4] == 0 { return chng_map((2, 2), 2); } 
                else if MAP[5] == 0 { return chng_map((3, 2), 2); }
            }
            3 => {
                if MAP[0] == 0 { return chng_map((1, 1), 2); } 
                else if MAP[4] == 0 { return chng_map((2, 2), 2); } 
                else if MAP[6] == 0 { return chng_map((1, 3), 2); }
            }
            4 => {
                if MAP[1] == 0 { return chng_map((2, 1), 2); } 
                else if MAP[3] == 0 { return chng_map((1, 2), 2); } 
                else if MAP[5] == 0 { return chng_map((3, 2), 2); } 
                else if MAP[7] == 0 { return chng_map((2, 3), 2); }
            }
            5 => {
                if MAP[2] == 0 { return chng_map((1, 3), 2); } 
                else if MAP[4] == 0 { return chng_map((2, 2), 2); } 
                else if MAP[8] == 0 { return chng_map((3, 3), 2); }
            }
            6 => {
                if MAP[3] == 0 { return chng_map((1, 2), 2); } 
                else if MAP[4] == 0 { return chng_map((2, 2), 2); } 
                else if MAP[7] == 0 { return chng_map((2, 3), 2); }
            }
            7 => {
                if MAP[4] == 0 { return chng_map((2, 2), 2); } 
                else if MAP[6] == 0 { return chng_map((1, 3), 2); } 
                else if MAP[8] == 0 { return chng_map((3, 3), 2); }
            }
            8 => {
                if MAP[5] == 0 { return chng_map((3, 2), 2); } 
                else if MAP[7] == 0 { return chng_map((2, 3), 2); } 
                else if MAP[4] == 0 { return chng_map((2, 2), 2); }
            }
            _ => { println!("problème au niveau de i (ia avancée)"); }
        }
    }
    false
}

fn adv_ai_play() {
    unsafe {
        let mut already_played: bool = false;
        for pos in MAP {
            if pos == 2 {
                already_played = true;
                break;
            }
        }
        
        if already_played {
            let mut i: i32 = 0;
            for _pos in MAP {
                // L'IA vérifie uniquement autour de SES pions, pas les cases vides !
                if MAP[i as usize] == 2 {
                    if ai_check_around(i) {
                        return; // Dès qu'un pion est posé, on arrête le tour.
                    }
                }
                i += 1;
            }
            // Si l'IA n'a pas trouvé de case autour de ses pions, elle joue au hasard
            rnd_ai_play();
        } else {
            rnd_ai_play();
        }
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
                0 => {match spot { 1 => {draw_plrs((0, 100), true);}, 2 => {draw_plrs((0, 100), false);}, _ => {}}},
                1 => {match spot { 1 => {draw_plrs((230, 100), true);}, 2 => {draw_plrs((230, 100), false);}, _ => {}}},
                2 => {match spot { 1 => {draw_plrs((460, 100), true);}, 2 => {draw_plrs((460, 100), false);}, _ => {}}},
                3 => {match spot { 1 => {draw_plrs((0, 330), true);}, 2 => {draw_plrs((0, 330), false);}, _ => {}}},
                4 => {match spot { 1 => {draw_plrs((230, 330), true);}, 2 => {draw_plrs((230, 330), false);}, _ => {}}},
                5 => {match spot { 1 => {draw_plrs((460, 330), true);}, 2 => {draw_plrs((460, 330), false);}, _ => {}}},
                6 => {match spot { 1 => {draw_plrs((0, 560), true);}, 2 => {draw_plrs((0, 560), false);}, _ => {}}},
                7 => {match spot { 1 => {draw_plrs((230, 560), true);}, 2 => {draw_plrs((230, 560), false);}, _ => {}}},
                8 => {match spot { 1 => {draw_plrs((460, 560), true);}, 2 => {draw_plrs((460, 560), false);}, _ => {}}},
                _ => {}
            }
            i += 1;
        }
    }
}

// Renvoie true si un pion a bien été placé, false si la case était déjà occupée
fn chng_map(coords: (i32, i32), plr: i32) -> bool {
    let mut played = false;
    unsafe {
        match plr {
            1 | 2 => { // On peut regrouper le match de plr pour gagner de la place !
                match coords {
                    (1, 1) => { if MAP[0] == 0 { MAP[0] = plr; played = true; } },
                    (1, 2) => { if MAP[1] == 0 { MAP[1] = plr; played = true; } },
                    (1, 3) => { if MAP[2] == 0 { MAP[2] = plr; played = true; } },
                    (2, 1) => { if MAP[3] == 0 { MAP[3] = plr; played = true; } },
                    (2, 2) => { if MAP[4] == 0 { MAP[4] = plr; played = true; } },
                    (2, 3) => { if MAP[5] == 0 { MAP[5] = plr; played = true; } },
                    (3, 1) => { if MAP[6] == 0 { MAP[6] = plr; played = true; } },
                    (3, 2) => { if MAP[7] == 0 { MAP[7] = plr; played = true; } },
                    (3, 3) => { if MAP[8] == 0 { MAP[8] = plr; played = true; } },
                    _ => { println!("défaut d'entrée des coords"); }
                }
            },
            _ => { println!("défaut d'entrée du joueur"); }
        }
    }
    played
}

fn check_full() -> bool {
    let mut empty_slots_exist = false;
    unsafe {
        for i in MAP {
            if i == 0 {
                empty_slots_exist = true;
                break;
            }
        }
    }
    empty_slots_exist
}

pub async fn run_game() {
    // Initialiser l'aléatoire pour de vrai !
    rand::srand(macroquad::miniquad::date::now() as u64);
    
    loop {
        clear_background(WHITE);

        if check_full() {
            if is_mouse_button_pressed(MouseButton::Left) {
                // Le joueur essaie de jouer
                let joueur_a_joue = plr_play(mouse_position());
                
                // L'IA ne joue QUE SI le joueur a réussi à placer son pion, 
                // et qu'il reste de la place sur le plateau !
                if joueur_a_joue && check_full() {
                    adv_ai_play();
                }
            }
            drawmap();
        } else {
            drawmap();
            in_game_print("match nul : la grille est pleine.".to_string());
        }

        next_frame().await
    }
}