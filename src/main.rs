mod data;

use macroquad::prelude::*;
use crate::data::Card;

fn mouse_in(x: f32, y: f32, width: f32, height: f32) -> bool{
    let (mx, my) = mouse_position();
    return mx >= x && mx < (x + width) && my >= y && my < (y + height);
}

fn mouse_in_text(text: &str, x: f32, y: f32) -> bool{
    let area = measure_text(text, None, 30, 1.);
    return mouse_in(x, y - area.offset_y, area.width, area.height);
}

#[macroquad::main("Scoundrel")]
async fn main(){
    let mut deck = data::gen_deck();
    let mut health = 20;
    let mut weapon_strength = 0;
    let mut weapon_max = 20;
    let mut ran = false;
    let mut room: Vec<Card> = Vec::new();

    loop{
        clear_background(WHITE);

        // display your stats
        draw_text(&format!("Health: {health}/20"), 10., 30., 30., RED);
        if weapon_strength > 0{
            draw_text(&format!("Weapon: {weapon_strength} (capped at {weapon_max})"), 10., 60., 30., RED);
        }else{
            draw_text("No weapon", 10., 60., 30., RED);
        }
        draw_text(&format!("{} remain", deck.len()), 10., 90., 30., ORANGE);
        if ran{
            draw_text("Already ran", 10., 120., 30., ORANGE);
        }

        // display the room
        for i in (0..room.len()).rev(){
            let card = room[i];
            let text = &card.describe();
            let mut x = 100.;
            let y = 200. + (35 * i) as f32;
            if mouse_in_text(text, x, y){
                x += 20.;
                if is_mouse_button_pressed(MouseButton::Left){
                    room.remove(i);
                    match card{
                        Card::Weapon(str) => {
                            weapon_strength = str;
                            weapon_max = 20;
                        }
                        Card::Health(str) => {
                            health += str;
                            health = health.min(20);
                        }
                        Card::Monster(_, str) => {
                            if str < weapon_max{
                                health = health.saturating_sub(str.saturating_sub(weapon_strength));
                                weapon_max = str;
                            }else{
                                health = health.saturating_sub(str);
                            }
                        }
                    }
                }
            }
            draw_text(text, x, y, 30., card.colour());
        }

        // running
        let mut just_ran = false;
        let mut run_x = 100.;
        if mouse_in_text("Run!", run_x, 380.) && !ran{
            run_x += 20.;
            if is_mouse_button_pressed(MouseButton::Left){
                ran = true;
                just_ran = true;
                room.drain(..).for_each(|it| deck.push_back(it));
            }
        }
        draw_text("Run!", run_x, 380., 30., if ran { GRAY } else { ORANGE });

        // win condition: deck and room are empty
        if deck.is_empty() && room.is_empty(){
            draw_text("Won!", 300., 20., 30., GREEN);
        }
        // lose condition: health is 0
        if health <= 0{
            draw_text("Lost!", 300., 20., 30., BLACK);
        }

        // add to room if necessary
        if room.len() <= 1{
            if !just_ran{
                ran = false;
            }
            while room.len() < 4 && !deck.is_empty(){
                room.push(deck.pop_front().unwrap());
            }
        }

        //

        next_frame().await
    }
}
