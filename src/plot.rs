use serde::Deserialize;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use csv::Reader;
use plotters::prelude::*;

#[derive(Debug, Deserialize)]
pub struct GameState {
    time: f64,
    team: Option<u32>,
    player_name: String,
    location_x: f64,
    location_y: f64,
    location_z: f64,
    rotation_x: f64,
    rotation_y: f64,
    rotation_z: f64,
    rotation_w: f64,
    angular_velocity_x: f64,
    angular_velocity_y: f64,
    angular_velocity_z: f64,
    linear_velocity_x: f64,
    linear_velocity_y: f64,
    linear_velocity_z: f64,
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

    if teams.len() >= 2 {
        let orange_heatmap = format!("{}_orange_team_heatmap.png", file_path);
        plot_team_heatmap(&data, &orange_heatmap, Some(teams[0]), "Orange Team Heatmap", RGBColor(255, 165, 0))?;

        let blue_heatmap = format!("{}_blue_team_heatmap.png", file_path);
        plot_team_heatmap(&data, &blue_heatmap, Some(teams[1]), "Blue Team Heatmap", RGBColor(0, 0, 255))?;
    } else {
        println!("Not enough teams found in the dataset to plot heatmaps.");
    }

    if let Ok(player_name) = env::var("PLAYER_NAME") {
        let player_plot = format!("{}_{}.png", file_path, player_name);
        plot_player(&data, &player_plot, &player_name)?;
    }

    println!("Heatmaps generated.");
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

fn plot_team_heatmap(
    data: &[GameState],
    output_file: &str,
    team_id: Option<u32>,
    caption: &str,
    color: RGBColor,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(output_file, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let (min_x, max_x, min_y, max_y) = calculate_bounds(data);

    // Define the heatmap grid size
    let grid_size_x = 50;
    let grid_size_y = 50;

    let cell_width = (max_x - min_x) / grid_size_x as f64;
    let cell_height = (max_y - min_y) / grid_size_y as f64;

    // Initialize a 2D grid to hold densities
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

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    chart.configure_mesh().draw()?;

    // Draw the heatmap by filling grid cells
    for x in 0..grid_size_x {
        for y in 0..grid_size_y {
            if grid[x][y] > 0 {
                let intensity = grid[x][y] as f64 / max_density as f64;
                let cell_color = color.mix(intensity);

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

    root.present()?;
    Ok(())
}


fn plot_player(
    data: &[GameState],
    output_file: &str,
    player_name: &str,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(output_file, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let (min_x, max_x, min_y, max_y) = calculate_bounds(data);

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("Game for Player: {}", player_name), ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    chart.configure_mesh().draw()?;

    // Filter data for the specific player
    let player_positions: Vec<(f64, f64)> = data
        .iter()
        .filter(|s| s.player_name == player_name)
        .map(|state| (state.location_x, state.location_y))
        .collect();

    // Use a heatmap-like representation by plotting dense points
    chart.draw_series(
        player_positions
            .iter()
            .map(|&(x, y)| Circle::new((x, y), 1, RGBColor(255, 0, 0).mix(0.2).filled())),
    )?;

    root.present()?;
    Ok(())
}
