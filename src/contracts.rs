use crate::police::PoliceState;
use crate::prelude::sprites::buttons;
use crate::prelude::*;

pub const MIN_ACCEPT_DISTANCE: f32 = 256.0;
pub const MIN_DELIVERY_DISTANCE: f32 = 256.0;

pub const MIN_DELAY_BETWEEN_NEW_CONTRACTS: f32 = 5000.0;
pub const MAX_CONTRACTS: usize = 3;

#[derive(Clone, Copy)]
pub struct Contract {
    pub src_planet: usize,
    pub dst_planet: usize,
    pub cargo: Cargo,
    pub reward: u32,
    pub wanted: f32,
    pub expires_at: f32,
}

#[derive(Clone, Copy)]
pub enum Cargo {
    Passengers,
    Crabs,
    Bananas,
}

impl Cargo {
    pub fn sprite(&self) -> u32 {
        match self {
            Cargo::Passengers => 352,
            Cargo::Bananas => 354,
            Cargo::Crabs => 356,
        }
    }
}

pub fn tic(
    rng: &mut dyn RngCore,
    camera: &mut Camera,
    game: &mut Game,
    player: &Player,
    planets: &[Planet],
    police: &mut PoliceState,
) {
    let mo = mouse();

    // Spawn new contracts
    #[allow(clippy::collapsible_if)]
    if game.contracts.len() < MAX_CONTRACTS {
        if game.time - game.time_of_last_contract_spawned
            > MIN_DELAY_BETWEEN_NEW_CONTRACTS
        {
            if rng.gen::<f32>() > 0.5 {
                let mut tries = 0;
                let mut spawned = false;

                while tries < 64 {
                    tries += 1;

                    // ---

                    let src_planet = rng.gen_range(1..planets.len());
                    let dst_planet = rng.gen_range(1..planets.len());

                    if dst_planet == src_planet {
                        continue;
                    }

                    if game.contracts.iter().any(|c| c.src_planet == src_planet)
                    {
                        continue;
                    }

                    let reward;
                    let wanted;
                    let cargo;

                    if rng.gen_bool(0.15) {
                        reward = rng.gen_range(8..=12);
                        wanted = 0.7;
                        cargo = Cargo::Crabs;
                    } else {
                        reward = rng.gen_range(1..=5);
                        wanted = ((reward as f32) / 5.0 * 0.3).clamp(0.1, 0.33);

                        cargo = if rng.gen_bool(0.5) {
                            Cargo::Passengers
                        } else {
                            Cargo::Bananas
                        };
                    }

                    game.contracts.push(Contract {
                        src_planet,
                        dst_planet,
                        cargo,
                        reward,
                        wanted,
                        expires_at: game.time
                            + rng.gen_range(20.0..45.0) * 1000.0,
                    });

                    spawned = true;
                    break;
                }

                if spawned {
                    audio::play(sounds::NEW_CONTRACT);
                    msgs::add("New contract available!");
                }

                game.time_of_last_contract_spawned = game.time;
            }
        }
    }

    // Remove contracts that went stale
    game.contracts
        .extract_if(|contract| game.time >= contract.expires_at)
        .for_each(drop);

    // Draw available unselected contracts
    for (idx, contract) in game.contracts.iter().enumerate() {
        let src_planet = &planets[contract.src_planet];
        let dst_planet = &planets[contract.dst_planet];
        let src_pos = camera.world_to_screen(src_planet.pos);
        let blink = blink();

        if blink {
            Img::sprite_idx_with_size(320, uvec2(2, 2))
                .at(src_pos + vec2(64.0, -64.0) * camera.scale)
                .scale((4.0 * camera.scale).max(0.3))
                .draw();
        }

        Localizator::contract(src_pos).draw();

        let ship_to_planet_distance =
            (player.ship.pos - src_planet.pos).length();

        if camera.scale < 0.15 && blink && !game.manouver_mode {
            circb(src_pos.x as i32, src_pos.y as i32, 8, 3);
        }

        if ship_to_planet_distance < src_planet.radius + MIN_ACCEPT_DISTANCE
            && !camera.is_animating()
            && !game.is_paused()
            && game.cargo_hold.iter().any(|c| c.is_none())
        {
            game.selected_contract = Some(idx);
            game.manouver_mode = false;
            game.speed = GameSpeed::Paused;

            let target_pos = (src_planet.pos + dst_planet.pos) * 0.5;

            let target_scale = {
                let mut scale = 1.0;

                // ugh, this can surely be calculated analytically, but I don't
                // have much time now
                while scale > 0.001 {
                    let camera = Camera {
                        scale,
                        ..Default::default()
                    };

                    let src = camera.world_to_screen(src_planet.pos);
                    let dst = camera.world_to_screen(dst_planet.pos);

                    if camera.contains(src) && camera.contains(dst) {
                        break;
                    }

                    scale -= 0.0001;
                }

                scale
            };

            camera.animate_to(target_pos.extend(target_scale));
        }
    }

    if game.manouver_mode || !game.is_paused() {
        game.selected_contract = None;
    }

    if game.selected_contract.is_some() && mouse_right_pressed() {
        game.selected_contract = None;
    }

    // Draw selected contract ui
    if let Some(selected_contract) = game.selected_contract {
        let contract = &game.contracts[selected_contract];

        let src_planet = &planets[contract.src_planet];
        let dst_planet = &planets[contract.dst_planet];

        let src_pos = camera.world_to_screen(src_planet.pos);
        let dst_pos = camera.world_to_screen(dst_planet.pos);

        Arrow::new(src_pos, dst_pos, dst_planet.color)
            .margin(5.0)
            .draw();

        let mpos = vec2(mo.x as f32, mo.y as f32);
        let tooltip_pos = (src_pos + dst_pos) * 0.5 + vec2(0.0, -14.0);

        let txt_width = Text::new(format!("+${}k - Accept?", contract.reward))
            .at(tooltip_pos)
            .draw() as f32
            + 1.0;

        let btn_accept_pos = tooltip_pos + vec2(txt_width - 3.0 * 8.0, 14.0);
        let btn_reject_pos = tooltip_pos + vec2(txt_width - 1.0 * 8.0, 14.0);

        let btn_accept_hover = (mpos - btn_accept_pos).length() < 8.0;
        let btn_reject_hover = (mpos - btn_reject_pos).length() < 8.0;

        let btn_accept_sprite = if btn_accept_hover {
            buttons::highlighted::YES
        } else {
            buttons::inactive::YES
        };

        let btn_reject_sprite = if btn_reject_hover {
            buttons::highlighted::NO
        } else {
            buttons::inactive::NO
        };

        Img::sprite_idx_with_size(btn_accept_sprite as u32, uvec2(2, 2))
            .at(btn_accept_pos)
            .draw();

        Img::sprite_idx_with_size(btn_reject_sprite as u32, uvec2(2, 2))
            .at(btn_reject_pos)
            .draw();

        if mouse_left_pressed() {
            if btn_accept_hover {
                game.selected_contract = None;
                game.speed = GameSpeed::Normal;

                camera.animate_back();

                for hold in &mut game.cargo_hold {
                    if hold.is_none() {
                        *hold = Some(*contract);
                        break;
                    }
                }

                audio::play(sounds::COIN);
                police.increment_wanted_level(contract.wanted);
                game.contracts.remove(selected_contract);
            } else if btn_reject_hover {
                game.selected_contract = None;
                game.speed = GameSpeed::Normal;
                game.contracts.remove(selected_contract);

                camera.animate_back();
            }
        }
    }

    // If mouse is over cargo hold, show arrows to destinations
    let mpos = vec2(mo.x as f32, mo.y as f32);

    let cargo_hold_height = 3.0 * 16.0;

    let cargo_hold_bounds = (
        vec2(16.0, HEIGHT as f32 - cargo_hold_height),
        vec2(16.0, cargo_hold_height),
    );

    if mpos.x > cargo_hold_bounds.0.x
        && mpos.x < cargo_hold_bounds.0.x + cargo_hold_bounds.1.x
        && mpos.y > cargo_hold_bounds.0.y
        && mpos.y < cargo_hold_bounds.0.y + cargo_hold_bounds.1.y
    {
        for contract in game.cargo_hold.iter().flatten() {
            let dst_planet = &planets[contract.dst_planet];
            let dst_pos = camera.world_to_screen(dst_planet.pos);

            Arrow::new(mpos, dst_pos, dst_planet.color)
                .margin(5.0)
                .draw();
        }
    }

    // Deliveries
    let mut deliveries_to_clear = vec![];

    for (idx, contract) in game.cargo_hold.iter().enumerate() {
        if let Some(contract) = contract {
            let dst_planet = &planets[contract.dst_planet];

            let ship_to_planet_distance =
                (player.ship.pos - dst_planet.pos).length();

            if ship_to_planet_distance
                < dst_planet.radius + MIN_DELIVERY_DISTANCE
            {
                game.credits += contract.reward;
                game.total_credits += contract.reward;

                audio::play(sounds::COIN);
                msgs::add("Delivery complete!");

                deliveries_to_clear.push(idx);
            }
        }
    }

    for idx in deliveries_to_clear {
        game.cargo_hold[idx] = None;
    }
}
