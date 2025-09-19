
use statrs::distribution::{StudentsT, ContinuousCDF};

fn main() {

    let pre_test= vec![4, 4, 4, 4, 4];
    let post_test = vec![7, 6, 5, 8, 9];
    
    let session = KirkpatrickModel::new(pre_test, post_test);

    println!("avg {:?}", session.mean_diff());
}


// ==== ===== ==== ===== ==== ===== ==== ===== ==== ===== ==== ===== ==== ===== 

struct KirkpatrickModel {
    pre_test_data: Vec<f64>,
    post_test_data: Vec<f64>,
}


impl KirkpatrickModel {

        // Constructor
        fn new<T, U>(pre_test_data: Vec<T>, post_test_data: Vec<U>) -> Self 
        where 
            T: Into<f64> + Clone,
            U: Into<f64> + Clone,

        {
            let pre_test_data = pre_test_data.into_iter().map(|val| val.into()).collect();
            let post_test_data = post_test_data.into_iter().map(|val| val.into()).collect();

            Self { pre_test_data, post_test_data }
        }
    

        // Method
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
        

        // Method
        fn std_diff(&self) -> f64 {
            

            let mean_diff_vector = self.mean_diff();
            let diff_vector = self.score_diff();
            let mut sum: f64 = 0.0;
            
            for diff in &diff_vector {
                sum += (diff - mean_diff_vector) * (diff - mean_diff_vector);
            }
            
            let std = (sum/(diff_vector.len() as f64 - 1.0)).sqrt();
            return std    
        }
        

        // Method
        fn mean_diff(&self) -> f64 {

            let avg_vector = self.score_diff();
            let sum: f64 = avg_vector.iter().sum();
            let avg = sum/avg_vector.len() as f64;
            
            return avg
        }
        

        // Method
        fn score_diff(&self) -> Vec<f64>{
    
            let mut i = 0;
            let mut diff_vector: Vec<f64> = Vec::new();
    
            while i <self.pre_test_data.len(){
                let diff = self.post_test_data[i] - self.pre_test_data[i];
                diff_vector.push(diff);
                i += 1;
    
            }
            return diff_vector
        }
}    
