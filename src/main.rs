
use statrs::distribution::{StudentsT, ContinuousCDF};

fn main() {

    let pre_test = vec![4.0, 4.0, 4.0, 4.0, 4.0];
    let post_test = vec![7.0, 6.0, 5.0, 8.0, 9.0];

    let diff = score_diff(&pre_test, &post_test);
    let mean_diff = mean_diff(&diff);
    let std_diff = std_diff(&diff, &mean_diff);
    let t_stat_res = t_stat(&mean_diff, &std_diff, &(pre_test.len() as f64));
    let p_value = p_value(&t_stat_res, pre_test.len());

    println!("{}", p_value); 

}


fn p_value(t_stat: &f64, n_of_users: usize) -> f64 {

    let df = (n_of_users - 1) as f64;
    let t_disc = StudentsT::new(0.0, 1.0, df).unwrap();

    let cdf_val = t_disc.cdf(t_stat.abs());

    return 2.0 * (1.0 - cdf_val)


}

fn t_stat(mean_diff: &f64, std_diff: &f64, n_of_users: &f64) -> f64 {

    let t_stat = mean_diff / (std_diff/n_of_users.sqrt());
    return t_stat

}


fn mean_diff(avg_vector: &[f64]) -> f64 {

    let mut sum: f64 = 0.0;
//    let float_avgs: Vec<f64> = avgs.iter().map(|&x| x as f64).collect();

    for avg in avg_vector {
        sum += avg;
    }

    let avg = sum/avg_vector.len() as f64;
    return avg

}

fn score_diff(pre_test: &[f64], post_test: &[f64]) -> Vec<f64>{

    let mut i = 0;
    let mut diff_vector: Vec<f64> = Vec::new();

    while i < pre_test.len(){
        let diff = post_test[i] - pre_test[i];
        diff_vector.push(diff);
        i += 1;

    }
    return diff_vector

}

fn std_diff(diff_vector: &[f64], mean_diff: &f64) -> f64 {

    let mut sum: f64 = 0.0;

    for diff in diff_vector {
        sum += (diff - mean_diff) * (diff - mean_diff);
    }

    let std = (sum/(diff_vector.len() as f64 - 1.0)).sqrt();
    return std

}

