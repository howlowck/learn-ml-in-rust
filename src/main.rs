fn neural_network(inputs: &[f32], weights: &[f32]) -> f32 {
    let mut i: usize = 0;
    let multiplied: Vec<f32> = inputs
            .iter()
            .map(|&num| { let result = num* weights[i]; i += 1; result })
            .collect();

    for number in &multiplied
    {
        println!("multiplied number {:?}", number);
    }

    multiplied.iter().fold(0.0, |total, next| total + next)
}

fn main() {
    let toes: Vec<f32> = vec![8.5, 9.5, 9.9, 9.0];
    let winloss: Vec<f32> = vec![0.65, 0.8, 0.8, 0.9];
    let fans: Vec<f32> = vec![1.2, 1.3, 0.5, 1.0];

    let weights: Vec<Vec<f32>> = vec![
        vec![0.3, 0.1, 0.4],
        vec![0.1, 0.2, 0.0],
        vec![0.4, 0.4, 0.2]
    ];

    let inputs = vec![toes[0], winloss[0], fans[0]];
    let pred = neural_network(&inputs, &weights[1]);
    println!("sum! {:?}", pred);
}