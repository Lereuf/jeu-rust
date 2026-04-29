use macroquad::prelude::*;

mod tictactoe;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tic Tac Toe".to_owned(),
        window_width: 660,
        window_height: 860,
        window_resizable: false,
        ..Default::default()
    }
}

fn draw_menu()
{
    clear_background(WHITE);
    draw_rectangle_lines(20.0, 120.0, 300.0, 200.0, 10.0, SKYBLUE);
    draw_text("Tic Tac Toe", 40.0, 170.0, 50.0, BLUE);
    draw_rectangle_lines(340.0, 120.0, 300.0, 200.0, 10.0, SKYBLUE);
    draw_rectangle_lines(20.0, 340.0, 300.0, 200.0, 10.0, SKYBLUE);
    draw_rectangle_lines(340.0, 340.0, 300.0, 200.0, 10.0, SKYBLUE);
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        draw_menu();
        if is_mouse_button_pressed(MouseButton::Left)
        {
            if mouse_position().0 > 20.0 && mouse_position().0 < 320.0 && mouse_position().1 > 120.0 && mouse_position().1 < 320.0 {
                tictactoe::run_game().await;
            }
        }
        next_frame().await
    }
}