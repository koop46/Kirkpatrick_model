
#[warn(unused_mut)]

fn main() {

    let pre_test = vec![4.0, 4.0, 4.0, 4.0, 4.0];
    let post_test = vec![7.0, 6.0, 5.0, 8.0, 9.0];
    let avgs = vec![2.0, 3.0, 4.0, 5.0, 6.0];

    let diff = score_diff(&pre_test, &post_test);
    let mean = mean_diff(&avgs);
    let std = std_diff(&diff, &mean);

    // println!("{}", mean); 
    // println!("{:?}", diff); 
    println!("{}", std); 


}

fn mean_diff(avg_vector: &[f32]) -> f32 {

    let mut sum: f32 = 0.0;

    for avg in avg_vector {
        sum += avg;
    }

    let avg = sum/avg_vector.len() as f32;
    return avg

}

fn score_diff(pre_test: &[f32], post_test: &[f32]) -> Vec<f32>{

    let mut i = 0;
    let mut diff_vector: Vec<f32> = Vec::new();

    while i < pre_test.len(){
        let mut diff = post_test[i] - pre_test[i];
        diff_vector.push(diff);
        i += 1;

    }
    return diff_vector

}

fn std_diff(diff_vector: &[f32], mean_diff: &f32) -> f32 {

    let mut sum: f32 = 0.0;

    for diff in diff_vector {
        sum += (diff - mean_diff) * (diff - mean_diff);
    }

    let std = (sum/(diff_vector.len() as f32 -1.0)).sqrt();
    return std

}

fn t_stat(avg_diff: &f32, std_diff: &f32) {

    

}