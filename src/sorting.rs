pub mod bubble_sort_no_flag
{
    pub fn sort(vector: Vec<i64>) -> (Vec<i64>, u64)
    {
        let vector_len = vector.len();
        let mut sorted_vec: Vec<i64> = vector;
        let mut iterations: u64 = 0;

        for i in 0..vector_len
        {
            for j in 0..(vector_len - 1)
            {
                if sorted_vec[i] < sorted_vec[j]
                {
                    let temp = sorted_vec[j + 1];
                    sorted_vec[j + 1] = sorted_vec[j];
                    sorted_vec[j] = temp;                    
                }

                iterations += 1;
            }
        }

        (sorted_vec, iterations)
    }
}

pub mod bubble_sort_with_flag
{
    pub fn sort(vector: Vec<i64>) -> (Vec<i64>, u64)
    {
        let vector_len = vector.len();
        let mut sorted_vec: Vec<i64> = vector;
        let mut changed: bool;
        let mut iterations: u64 = 0;

        for _ in 0..vector_len
        {
            changed = false;

            for j in 0..(vector_len - 1)
            {
                if sorted_vec[j] > sorted_vec[j + 1]
                {
                    let temp = sorted_vec[j + 1];
                    sorted_vec[j + 1] = sorted_vec[j];
                    sorted_vec[j] = temp;

                    changed = true;
                }

                iterations += 1;
            }

            if !changed 
            {
                break;
            }
        }

        (sorted_vec, iterations)
    }
}
