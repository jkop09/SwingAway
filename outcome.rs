pub fn hit<'a>(exit_velo: &f32, prob: &f32, distance: &f32, feild: &str) -> &'a str {
    let mut hit = "";
    if distance >= &355.00 && feild == "LF" {
        hit = "HR";
    }
    else if distance >= &370.00 && feild == "LC" {
        hit = "HR";
    }
    else if distance >= &408.00 && feild == "CF" {
        hit = "HR";
    }
    else if distance >= &375.00 && feild == "RC" {
        hit = "HR";
    }
    else if distance >= &350.00 && feild == "RF" {
        hit = "HR";
    }
    else if exit_velo >= &73.00 || exit_velo <= &88.00 {
        if prob <= &14.82 {
            hit = "single"
        }
        else if prob <= &15.5 {
            hit = "double"
        }
        else {
            hit = "out"
        }
    }
    else if exit_velo >= &88.01 || exit_velo <= &102.66 {
        if prob <= &19.46 {
            hit = "single"
        }
        else if prob <= &20.49 {
            hit = "double"
        }
        else {
            hit = "out"
        }
    }
    else if exit_velo >= &102.67 || exit_velo <= &117.33 {
        if prob <= &20.67 {
            hit = "single"
        }
        else if prob <= &22.63 {
            hit = "double"
        }
        else if prob <= &22.68 {
            hit = "triple"
        }
        else {
            hit = "out"
        }
    }
    else if exit_velo >= &117.34 || exit_velo <= &132.00 {
        if prob <= &17.62 {
            hit = "single"
        }
        else if prob <= &20.74 {
            hit = "double"
        }
        else if prob <= &20.99 {
            hit = "triple"
        }
        else {
            hit = "out"
        }
    }
    else if exit_velo >= &132.01 || exit_velo <= &146.66 {
        if prob <= &17.80 {
            hit = "single"
        }
        else if prob <= &25.48 {
            hit = "double"
        }
        else if prob <= &26.16 {
            hit = "triple"
        }
        else {
            hit = "out"
        }
    }
    else if exit_velo >= &146.67 || exit_velo <= &161.33 {
        if prob <= &28.97 {
            hit = "single"
        }
        else if prob <= &43.15 {
            hit = "double"
        }
        else if prob <= &44.39 {
            hit = "triple"
        }
        else {
            hit = "out"
        }
    }
    else if exit_velo >= &161.34 || exit_velo <= &173.50668 {
        if prob <= &22.31 {
            hit = "single"
        }
        else if prob <= &47.86 {
            hit = "double"
        }
        else if prob <= &48.18 {
            hit = "triple"
        }
        else {
            hit = "out"
        }
    }
    {
        hit
    }
}