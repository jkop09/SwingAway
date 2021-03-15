pub fn dist(launch_angle: &f32, exit_velo: &f32, starting_height: &f32) -> f32 {
    let s: f32 = *launch_angle;
    let temp = exit_velo * s.sin();
    let f = f32::powf(temp, 2.0);
    let t = f - (4.0 * -16.0 * starting_height);
    let time = -1.0*(temp + t.sqrt())/ (2.0 * -16.0);
    let dist = exit_velo * s.cos() * time;
    {
        dist
    }
}