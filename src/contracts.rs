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

pub fn insert_into_empty_cargo<const N: usize>(
    contract: Contract,
    hold: &mut [Option<Contract>; N],
) -> bool {
    for n in 0..N {
        if hold[n].is_none() {
            hold[n] = Some(contract);
            return true;
        }
    }

    false
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
    if game.contracts.len() < MAX_CONTRACTS {
        if game.time - game.time_of_last_contract_spawned
            > MIN_DELAY_BETWEEN_NEW_CONTRACTS
        {
            if rng.gen::<f32>() > 0.5 {
                // Spawn new contract

                music(
                    tracks::NEW_CONTRACT_SOUND,
                    MusicOptions {
                        repeat: false,
                        ..Default::default()
                    },
                );

                msgs::add("New contract available!");

                game.time_of_last_contract_spawned = game.time;

                let src_planet = rng.gen_range(1..planets.len());
                let mut dst_planet = rng.gen_range(1..planets.len());

                while dst_planet == src_planet {
                    dst_planet = rng.gen_range(1..planets.len());
                }

                let cargo = match rng.gen_range(0..3) {
                    0 => Cargo::Passengers,
                    1 => Cargo::Crabs,
                    2 => Cargo::Bananas,
                    _ => unreachable!(),
                };

                game.contracts.push(Contract {
                    src_planet,
                    dst_planet,
                    cargo,
                    reward: rng.gen_range(5..=25),
                });
            }
        }
    }

    // Draw available unselected contracts
    for (idx, contract) in game.contracts.iter().enumerate() {
        let src_planet = &planets[contract.src_planet];
        let dst_planet = &planets[contract.dst_planet];
        let src_pos = camera.world_to_screen(src_planet.pos);
        let flash_indicator = (time() / 1000.0) as i32 % 2 == 0;

        if flash_indicator {
            Img::sprite_idx_with_size(320, uvec2(2, 2))
                .at(src_pos + vec2(64.0, -64.0) * camera.scale)
                .scale((4.0 * camera.scale).max(0.3))
                .draw();
        }

        let ship_to_planet_distance =
            (player.ship.pos - src_planet.pos).length();

        if camera.scale < 0.15 && flash_indicator && !game.manouver_mode {
            circb(src_pos.x as i32, src_pos.y as i32, 8, 3);
        }

        if ship_to_planet_distance < src_planet.radius + MIN_ACCEPT_DISTANCE
            && !camera.is_animating()
            && !game.is_paused()
        {
            game.selected_contract = Some(idx);
            game.manouver_mode = false;
            game.speed = GameSpeed::Paused;

            let target_pos = (src_planet.pos + dst_planet.pos) * 0.5;

            let target_scale = Camera::size().min_element()
                / (src_planet.pos.distance(dst_planet.pos) * 1.2);

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

        let tooltip_pos = dst_pos + vec2(8.0, -14.0);

        let txt_width =
            Text::new("Accept?").at(tooltip_pos).draw() as f32 + 1.0;

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

                if !insert_into_empty_cargo(*contract, &mut game.cargo_hold) {
                    msgs::add("Complete your current contracts first!");
                } else {
                    music(
                        tracks::COIN_SOUND,
                        MusicOptions {
                            repeat: false,
                            ..Default::default()
                        },
                    );

                    game.contracts.remove(selected_contract);
                    police.increment_wanted_level();
                }
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
        for cargo in &game.cargo_hold {
            if let Some(contract) = cargo {
                let dst_planet = &planets[contract.dst_planet];
                let dst_pos = camera.world_to_screen(dst_planet.pos);

                Arrow::new(mpos, dst_pos, dst_planet.color)
                    .margin(5.0)
                    .draw();
            }
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

                music(
                    tracks::COIN_SOUND,
                    MusicOptions {
                        repeat: false,
                        ..Default::default()
                    },
                );

                msgs::add("Delivery complete!");

                deliveries_to_clear.push(idx);
            }
        }
    }

    for idx in deliveries_to_clear {
        game.cargo_hold[idx] = None;
    }
}
