use super::client::{ClientId, ClientState};
use crate::command::{ActiveCommand, Command};
use crate::constants::KO_RESPONSE;
use crate::game::GameState;
use crate::gui;
use std::time::{Duration, Instant};

enum StartResult {
    Continue,
    Stop,
}

pub fn process_timers(game: &mut GameState, now: Instant) {
    for _ in 0..1024 {
        let mut progress = 0;

        progress += complete_finished_commands(game, now);
        progress += game.process_food(now);
        progress += game.process_resource_respawn(now);
        progress += start_pending_commands(game, now);
        progress += complete_finished_commands(game, now);

        if progress == 0 {
            break;
        }
    }
}

pub fn next_timeout(game: &GameState, now: Instant) -> Option<Duration> {
    game.next_deadline()
        .map(|deadline| deadline.saturating_duration_since(now))
}

fn start_pending_commands(game: &mut GameState, now: Instant) -> usize {
    let client_ids = game.clients.keys().copied().collect::<Vec<_>>();

    client_ids
        .into_iter()
        .map(|client_id| start_client_commands(game, client_id, now))
        .sum()
}

fn start_client_commands(game: &mut GameState, client_id: ClientId, now: Instant) -> usize {
    let mut started = 0;

    while let Some((player_id, command)) = take_next_command(game, client_id) {
        started += 1;

        match start_command(game, client_id, player_id, command, now) {
            StartResult::Continue => continue,
            StartResult::Stop => break,
        }
    }

    started
}

fn take_next_command(game: &mut GameState, client_id: ClientId) -> Option<(usize, Command)> {
    let player_id = match game.clients.get(&client_id) {
        Some(client)
            if client.state == ClientState::Ai
                && client.active.is_none()
                && !client.close_after_flush =>
        {
            client.player_id?
        }
        _ => return None,
    };

    if game.is_player_frozen(player_id) {
        return None;
    }

    game.clients
        .get_mut(&client_id)?
        .queue
        .pop_front()
        .map(|command| (player_id, command))
}

fn start_command(
    game: &mut GameState,
    client_id: ClientId,
    player_id: usize,
    command: Command,
    now: Instant,
) -> StartResult {
    match command {
        Command::Incantation => {
            if game.begin_incantation(client_id, player_id, now) {
                StartResult::Stop
            } else {
                StartResult::Continue
            }
        }
        Command::Fork => {
            game.broadcast_to_guis(&gui::pfk(player_id));
            schedule_command(game, client_id, Command::Fork, now);
            StartResult::Stop
        }
        command => {
            schedule_command(game, client_id, command, now);
            StartResult::Stop
        }
    }
}

fn schedule_command(game: &mut GameState, client_id: ClientId, command: Command, now: Instant) {
    let finishes_at = now + game.duration(command.time_units());

    if let Some(client) = game.clients.get_mut(&client_id) {
        client.active = Some(ActiveCommand::new(command, finishes_at));
    }
}

fn complete_finished_commands(game: &mut GameState, now: Instant) -> usize {
    let due_client_ids = game
        .clients
        .iter()
        .filter_map(|(client_id, client)| finished_client_id(game, *client_id, client, now))
        .collect::<Vec<_>>();

    due_client_ids
        .into_iter()
        .filter(|client_id| complete_client_command(game, *client_id))
        .count()
}

fn finished_client_id(
    game: &GameState,
    client_id: ClientId,
    client: &super::client::Client,
    now: Instant,
) -> Option<ClientId> {
    let active = client.active.as_ref()?;

    if active.finishes_at > now {
        return None;
    }

    let player_is_frozen = client
        .player_id
        .is_some_and(|player_id| game.is_player_frozen(player_id));

    (!player_is_frozen || active.command == Command::Incantation).then_some(client_id)
}

fn complete_client_command(game: &mut GameState, client_id: ClientId) -> bool {
    let active = game
        .clients
        .get_mut(&client_id)
        .and_then(|client| client.active.take());

    match active {
        Some(active) => {
            execute_command(game, client_id, active);
            true
        }
        None => false,
    }
}

fn execute_command(game: &mut GameState, client_id: ClientId, active: ActiveCommand) {
    let Some(player_id) = game
        .clients
        .get(&client_id)
        .and_then(|client| client.player_id)
    else {
        return;
    };

    match active.command {
        Command::Forward => {
            game.execute_forward(client_id, player_id);
        }
        Command::Right => {
            game.execute_right(client_id, player_id);
        }
        Command::Left => {
            game.execute_left(client_id, player_id);
        }
        Command::Look => {
            execute_look(game, client_id, player_id);
        }
        Command::Inventory => {
            execute_inventory(game, client_id, player_id);
        }
        Command::Broadcast(message) => {
            game.execute_broadcast(client_id, player_id, &message);
        }
        Command::ConnectNbr => {
            execute_connect_nbr(game, client_id, player_id);
        }
        Command::Fork => {
            game.finish_fork(client_id, player_id);
        }
        Command::Eject => {
            game.execute_eject(client_id, player_id);
        }
        Command::Take(resource) => {
            game.execute_take(client_id, player_id, resource);
        }
        Command::Set(resource) => {
            game.execute_set(client_id, player_id, resource);
        }
        Command::Incantation => {
            finish_incantation(game, active);
        }
        Command::Unknown => {
            game.queue_to_client(client_id, KO_RESPONSE);
        }
    }
}

fn execute_look(game: &mut GameState, client_id: ClientId, player_id: usize) {
    let response = game
        .look_response(player_id)
        .unwrap_or_else(|| KO_RESPONSE.to_string());

    game.queue_to_client(client_id, &response);
}

fn execute_inventory(game: &mut GameState, client_id: ClientId, player_id: usize) {
    let response = game
        .players
        .get(&player_id)
        .map(|player| player.inventory.ai_response())
        .unwrap_or_else(|| KO_RESPONSE.to_string());

    game.queue_to_client(client_id, &response);
}

fn execute_connect_nbr(game: &mut GameState, client_id: ClientId, player_id: usize) {
    let available_slots = game
        .players
        .get(&player_id)
        .map(|player| game.available_egg_count(&player.team_name))
        .unwrap_or(0);

    game.queue_to_client(client_id, &format!("{available_slots}\n"));
}

fn finish_incantation(game: &mut GameState, active: ActiveCommand) {
    if let Some(incantation_id) = active.incantation_id {
        game.finish_incantation(incantation_id);
    }
}
