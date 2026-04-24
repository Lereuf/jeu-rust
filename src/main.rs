use macroquad::prelude::*;

#[macroquad::main("Mon Premier Jeu")]
async fn main() {
    loop {
        // 1. Nettoyer l'écran à chaque frame (l'équivalent de screen.fill en Pygame)
        clear_background(LIGHTGRAY);

        // 2. Dessiner des formes ou du texte
        draw_circle(screen_width() / 2.0, screen_height() / 2.0, 50.0, YELLOW);
        draw_text("Salut depuis Macroquad !", 20.0, 20.0, 30.0, DARKBLUE);

        // 3. Attendre la frame suivante (l'équivalent de pygame.display.flip())
        next_frame().await
    }
}