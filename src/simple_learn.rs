use crate::colors;

fn error_fn(goal: &f32, prediction: &f32) -> f32 {
    (prediction - goal).powi(2)
}

pub fn simplest_learn() {
    let mut weight: f32 = 0.5;
    let input: f32 = 0.5;
    let goal: f32 = 0.8;

    let step_amount: f32 = 0.001;
    let mut prediction: f32 = 0.0;
    let mut error: f32 = 0.0;

    for i in 0..1101 {
        prediction = input * weight;
        error = error_fn(&goal, &prediction);
        println!("---- {} ----", i);
        println!("Error: {}", error);
        println!("Prediction: {}", prediction);
        let up_prediction: f32 = input * (weight + step_amount);
        let up_error: f32 = error_fn(&goal, &up_prediction);
        let down_prediction: f32 = input * (weight - step_amount);
        let down_error: f32 = error_fn(&goal, &down_prediction);
        weight = if down_error < up_error {
            weight - step_amount
        } else if down_error > up_error {
            weight + step_amount
        } else {
            weight
        }
    }

    println!("\n =================");
    println!("Final Error: {:?}", colors::result().apply_to(error));
    println!("Final Prediction: {:?}", colors::result().apply_to(prediction));
}