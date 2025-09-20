use statrs::distribution::{StudentsT, ContinuousCDF};
use std::collections::BTreeMap;


//     [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0]

//     [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1]
fn main() {

    let mut pre_test_data = BTreeMap::new();
    pre_test_data.insert("user_1".to_string(), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0]);
    pre_test_data.insert("user_2".to_string(), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0]);

    
    let mut post_test_data = BTreeMap::new();
    post_test_data.insert("user_1".to_string(), vec![0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1]);
    post_test_data.insert("user_2".to_string(), vec![0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0]);

    
   let session = KirkpatrickModel::new(pre_test_data, post_test_data);

    // println!("avg {:?}", session.p_value());
    // println!("avg {:?}", session.p_value());
    // println!("avg {:?}", session.p_value());

    println!("{:?}", session.p_value());

}


// ==== ===== ==== ===== ==== ===== ==== ===== ==== ===== ==== ===== ==== ===== 



struct KirkpatrickModel {
    pre_test_data: BTreeMap<String, Vec<f64>>,
    post_test_data: BTreeMap<String, Vec<f64>>,
}


impl KirkpatrickModel {

        // fn test(&self) 
        // }
    
    // Constructor
        fn new(pre_test_data: BTreeMap<String, Vec<i32>>, post_test_data: BTreeMap<String, Vec<i32>>) -> Self {

            //            let pre_test_data = pre_test_data.into_iter().map(|val| val.into()).collect();
            
            
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
        
        fn p_value(&self) -> Vec<f64>{

            let t_stat = self.t_stat();
            let mut all_p_values = vec![];
            let n_of_questions = self.pre_test_data.values().next().unwrap().len();
            let df = (n_of_questions - 1) as f64;
            let t_disc = StudentsT::new(0.0, 1.0, df).unwrap();

            for i in 0..t_stat.len() {
                let cdf_val = t_disc.cdf(t_stat[i].abs());
                all_p_values.push(2.0 * (1.0 - cdf_val));
            }
            all_p_values
        }


        
        // // Method
        fn t_stat(&self) -> Vec<f64>{

            let mean_diff = self.mean_diff();
            let std_diff = self.std_diff();
            let n_of_questions = self.pre_test_data.values().next().unwrap().len();

            let mut all_t_stats = vec![];

            for i in 0..mean_diff.len(){

                let t_stat = mean_diff[i] / (std_diff[i]/(n_of_questions as f64).sqrt());
                
                all_t_stats.push(t_stat);
            }
            return all_t_stats

        }
        

        // // Method
        fn std_diff(&self) -> Vec<f64> {
        
            let mean_diffs = self.mean_diff();
            let diff_vectors = self.score_diff(); 
            let mut all_stds = vec![];            

            for i in 0..mean_diffs.len() {

                let mut sum: f64 = 0.0;
                
                for diff in &diff_vectors[i] {
                    sum += (diff - mean_diffs[i]) * (diff - mean_diffs[i]);

                }
                let std = (sum/(diff_vectors[i].len() as f64 - 1.0)).sqrt();
                all_stds.push(std);                
            }

            return all_stds
    }
        
        // // Method
        fn mean_diff(&self) -> Vec<f64>{

            let avg_vectors = self.score_diff();
            let mut averages = vec![];

            for i in 0..avg_vectors.len() {
                
                let sum: f64 = avg_vectors[i].iter().sum();
                let avg = sum/avg_vectors[i].len() as f64;

                averages.push(avg);

            }
            return averages
        }
        

        // // Method
        fn score_diff(&self) -> Vec<Vec<f64>> {

            let mut diff_vectors = vec![];

            for key in self.pre_test_data.keys() {
                let mut diff = vec![];

                for i in 0..(&self.pre_test_data[key]).len() {
                    diff.push(self.post_test_data[key][i] - self.pre_test_data[key][i]);    
                
                }
                diff_vectors.push(diff)

            }
            return diff_vectors
        }    
}

