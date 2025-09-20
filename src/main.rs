use statrs::distribution::{StudentsT, ContinuousCDF};
use std::collections::BTreeMap;



fn main() {
    let mut pre_test_data = BTreeMap::new();
    pre_test_data.insert("user_1".to_string(), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0]);
    pre_test_data.insert("user_2".to_string(), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0]);

    let mut post_test_data = BTreeMap::new();
    post_test_data.insert("user_1".to_string(), vec![0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1]);
    post_test_data.insert("user_2".to_string(), vec![0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0]);

    let session = KirkpatrickModel::new(pre_test_data, post_test_data);

    match session.summary() {
        Ok(summary) => {
            println!("{:#?}", summary);
        }
        Err(e) => {
            eprintln!("Failed to compute stats: {}", e);
        }
    }
}


// ==== ===== ==== ===== ==== ===== ==== ===== ==== ===== ==== ===== ==== ===== 



struct KirkpatrickModel {
    pre_test_data: BTreeMap<String, Vec<f64>>,
    post_test_data: BTreeMap<String, Vec<f64>>,
}

#[derive(Debug)]
struct SessionSummary {
    n_users: usize,
    mean_improvement: f64,
    std_improvement: f64,
    t_stat: f64,
    p_value: f64,
    cohens_d: f64,
    significant_05: bool,
    significant_01: bool,
}

impl KirkpatrickModel {

    // Constructor
        fn new(pre_test_data: BTreeMap<String, Vec<i32>>, post_test_data: BTreeMap<String, Vec<i32>>) -> Self {            
            
            let pre_test_data = pre_test_data
            .into_iter()
            .map(|(key, values)| {
                let float_values: Vec<f64> = values
                .into_iter()
                .map(|x| x as f64)
                .collect();
                (key,float_values)
            })
            .collect();
            
            
            let post_test_data = post_test_data
            .into_iter()
            .map(|(key, values)| {
                let float_values: Vec<f64> = values
                .into_iter()
                .map(|x| x as f64)
                .collect();
                (key,float_values)
            })
            .collect();
            
            Self { pre_test_data, post_test_data }
        }


         // Methods
        fn p_value(&self) -> f64 {

            let n_of_users = self.pre_test_data.len();
            let t_stat = self.t_stat();
    
            let df = (n_of_users - 1) as f64;
            let t_disc = StudentsT::new(0.0, 1.0, df).unwrap();
    
            let cdf_val = t_disc.cdf(t_stat.abs());
    
            return 2.0 * (1.0 - cdf_val)
        }


         // Method
        fn t_stat(&self) -> f64 {

            let mean_diff = self.mean_diff();
            let std_diff = self.std_diff();
            let n_of_users = self.pre_test_data.len();
    
            let t_stat = mean_diff / (std_diff/(n_of_users as f64).sqrt());
            return t_stat
        }            

        fn cohens_d(&self) -> f64 {

            let mean_diff = self.mean_diff();
            let std_diff = self.std_diff();

            return mean_diff / std_diff  // standardized effect size
        }


        //     // // Method
        fn std_diff(&self) -> f64 {
            
            let diff_vector = self.total_score_differences(); // Vec<f64> with one difference per user
            let mean_diff = self.mean_diff(); 
            
            let mut sum: f64 = 0.0;
            for diff in &diff_vector {
                sum += (diff - mean_diff) * (diff - mean_diff);
            }
            
            let std = (sum / (diff_vector.len() as f64 - 1.0)).sqrt();
            std
        }
        

        // // Method
        fn mean_diff(&self) -> f64{

            let avg_vectors = self.total_score_differences();
            
            return avg_vectors.iter().sum::<f64>() / avg_vectors.len() as f64
        }
        

        // // Method
        fn total_score_differences(&self) -> Vec<f64> {

            let mut diff_vectors = vec![];

            for key in self.pre_test_data.keys() {
                let mut diff = 0.0;

                for i in 0..(&self.pre_test_data[key]).len() {
                    diff += (self.post_test_data[key][i] - self.pre_test_data[key][i]);    
                
                }
                diff_vectors.push(diff);

            }
            return diff_vectors
        }    

        fn summary(&self) -> Result<SessionSummary, String> {
            let n = self.pre_test_data.len();
            let mean_diff = self.mean_diff();
            let std_diff = self.std_diff();
            let t_stat = self.t_stat();
            let p_val = self.p_value();
            let d = self.cohens_d();

            Ok(SessionSummary {
                n_users: n,
                mean_improvement: mean_diff,
                std_improvement: std_diff,
                t_stat,
                p_value: p_val,
                cohens_d: d,
                significant_05: p_val < 0.05,
                significant_01: p_val < 0.01,
            })
        }



}

