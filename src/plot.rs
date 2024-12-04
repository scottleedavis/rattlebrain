use serde::Deserialize;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use csv::Reader;
use plotters::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct GameState {
    // time: f64,
    team: Option<u32>,
    player_name: String,
    location_x: f64,
    location_y: f64,
    // location_z: f64,
    // rotation_x: f64,
    // rotation_y: f64,
    // rotation_z: f64,
    // rotation_w: f64,
    // angular_velocity_x: f64,
    // angular_velocity_y: f64,
    // angular_velocity_z: f64,
    // linear_velocity_x: f64,
    // linear_velocity_y: f64,
    // linear_velocity_z: f64,
}

pub fn plot_csv(file_path: &str) -> Result<Vec<GameState>, Box<dyn Error>> {
    let mut reader = Reader::from_path(file_path)?;
    let mut data = Vec::new();

    for result in reader.deserialize() {
        let record: GameState = result?;
        data.push(record);
    }

    // Discover unique team numbers
    let mut teams: Vec<u32> = data
        .iter()
        .filter_map(|s| s.team)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    teams.sort(); // Ensure consistent order

    let mut team_colors = HashMap::new();
    if teams.len() >= 2 {
        team_colors.insert(Some(teams[0]), RGBColor(255, 165, 0)); // Orange team
        team_colors.insert(Some(teams[1]), RGBColor(0, 0, 255));   // Blue team
    } else {
        return Err("Not enough teams found in the dataset to plot heatmaps.".into());
    }

    if let Ok(player_name) = env::var("PLAYER_NAME") {
        let player_plot_combo = format!("{}.png", file_path);
        plot_combined(&data, &player_plot_combo, &player_name, &team_colors)?;
    } else {
        let combined_heatmap = format!("{}.png", file_path);
        plot_combined_heatmap(&data, &combined_heatmap, &team_colors)?;
    }

    let ball_plot = format!("{}_ball.png", file_path);
    plot_ball(&data, &ball_plot)?;

    println!("Plots generated.");
    Ok(data)
}

fn calculate_bounds(data: &[GameState]) -> (f64, f64, f64, f64) {
    let buffer = 256.0;
    let min_x = data.iter().map(|s| s.location_x).fold(f64::INFINITY, f64::min)-buffer;
    let max_x = data.iter().map(|s| s.location_x).fold(f64::NEG_INFINITY, f64::max)+buffer;
    let min_y = data.iter().map(|s| s.location_y).fold(f64::INFINITY, f64::min)-buffer;
    let max_y = data.iter().map(|s| s.location_y).fold(f64::NEG_INFINITY, f64::max)+buffer;
    (min_x, max_x, min_y, max_y)
}

fn plot_combined_heatmap(
    data: &[GameState],
    output_file: &str,
    team_colors: &HashMap<Option<u32>, RGBColor>,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(output_file, (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let (min_x, max_x, min_y, max_y) = calculate_bounds(data);

    let mut chart = ChartBuilder::on(&root)
        .caption("Combined Team Heatmaps", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    chart.configure_mesh().draw()?;

    // Define heatmap grid size
    let grid_size_x = 50;
    let grid_size_y = 50;

    let cell_width = (max_x - min_x) / grid_size_x as f64;
    let cell_height = (max_y - min_y) / grid_size_y as f64;

    // Draw heatmaps for each team
    for &team_id in team_colors.keys() {
        let mut grid = vec![vec![0; grid_size_y]; grid_size_x];

        // Populate the grid with point densities
        data.iter()
            .filter(|s| s.team == team_id)
            .for_each(|state| {
                let x_index = ((state.location_x - min_x) / cell_width) as usize;
                let y_index = ((state.location_y - min_y) / cell_height) as usize;

                if x_index < grid_size_x && y_index < grid_size_y {
                    grid[x_index][y_index] += 1;
                }
            });

        // Determine the maximum density for normalization
        let max_density = grid.iter().flatten().copied().max().unwrap_or(1);

        let team_color = team_colors.get(&team_id).unwrap_or(&RGBColor(128, 128, 128)); // Default to gray

        // Draw the heatmap by filling grid cells
        for x in 0..grid_size_x {
            for y in 0..grid_size_y {
                if grid[x][y] > 0 {
                    let intensity = grid[x][y] as f64 / max_density as f64;
                    let cell_color = team_color.mix(intensity);

                    let x_start = min_x + x as f64 * cell_width;
                    let x_end = x_start + cell_width;
                    let y_start = min_y + y as f64 * cell_height;
                    let y_end = y_start + cell_height;

                    chart.draw_series([Rectangle::new(
                        [(x_start, y_start), (x_end, y_end)],
                        cell_color.filled(),
                    )])?;
                }
            }
        }
    }

    root.present()?;
    Ok(())
}


fn plot_ball(data: &[GameState], output_file: &str) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(output_file, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let (min_x, max_x, min_y, max_y) = calculate_bounds(data);

    let mut chart = ChartBuilder::on(&root)
        .caption("Ball Movement", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    chart.configure_mesh().draw()?;

    // Filter data for the ball (player_name is empty or "ball")
    let ball_positions: Vec<(f64, f64)> = data
        .iter()
        .filter(|s| s.player_name.is_empty() || s.player_name.to_lowercase() == "_ball_")
        .map(|state| (state.location_x, state.location_y))
        .collect();

    // Draw the ball's positions
    chart.draw_series(
        ball_positions
            .iter()
            .map(|&(x, y)| Circle::new((x, y), 1, RGBColor(0, 0, 0).filled())),
    )?;

    root.present()?;
    Ok(())
}

fn plot_combined(
    data: &[GameState],
    output_file: &str,
    player_name: &str,
    team_colors: &HashMap<Option<u32>, RGBColor>,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(output_file, (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let (min_x, max_x, min_y, max_y) = calculate_bounds(data);

    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("Combined Heatmaps and Player Movement: {}", player_name),
            ("sans-serif", 30),
        )
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    chart.configure_mesh().draw()?;

    // Define heatmap grid size
    let grid_size_x = 50;
    let grid_size_y = 50;

    let cell_width = (max_x - min_x) / grid_size_x as f64;
    let cell_height = (max_y - min_y) / grid_size_y as f64;

    // Draw heatmaps for each team
    for &team_id in team_colors.keys() {
        let mut grid = vec![vec![0; grid_size_y]; grid_size_x];

        // Populate the grid with point densities
        data.iter()
            .filter(|s| s.team == team_id)
            .for_each(|state| {
                let x_index = ((state.location_x - min_x) / cell_width) as usize;
                let y_index = ((state.location_y - min_y) / cell_height) as usize;

                if x_index < grid_size_x && y_index < grid_size_y {
                    grid[x_index][y_index] += 1;
                }
            });

        // Determine the maximum density for normalization
        let max_density = grid.iter().flatten().copied().max().unwrap_or(1);

        let team_color = team_colors.get(&team_id).unwrap_or(&RGBColor(128, 128, 128)); // Default to gray

        // Draw the heatmap by filling grid cells
        for x in 0..grid_size_x {
            for y in 0..grid_size_y {
                if grid[x][y] > 0 {
                    let intensity = grid[x][y] as f64 / max_density as f64;
                    let cell_color = team_color.mix(intensity);

                    let x_start = min_x + x as f64 * cell_width;
                    let x_end = x_start + cell_width;
                    let y_start = min_y + y as f64 * cell_height;
                    let y_end = y_start + cell_height;

                    chart.draw_series([Rectangle::new(
                        [(x_start, y_start), (x_end, y_end)],
                        cell_color.filled(),
                    )])?;
                }
            }
        }
    }

    // Overlay player movement
    let player_data: Vec<&GameState> = data
        .iter()
        .filter(|s| s.player_name == player_name)
        .collect();

    if player_data.is_empty() {
        println!("No data found for player: {}", player_name);
        return Ok(());
    }

    let player_team = player_data[0].team;
    let player_color = team_colors
        .get(&player_team)
        .unwrap_or(&RGBColor(0, 0, 0)); // Default to black if no team color found

    let player_positions: Vec<(f64, f64)> = player_data
        .iter()
        .map(|state| (state.location_x, state.location_y))
        .collect();

    chart.draw_series(
        player_positions
            .iter()
            .map(|&(x, y)| Circle::new((x, y), 1, player_color.mix(1.0).filled())),
    )?;

    root.present()?;
    Ok(())
}
