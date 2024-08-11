pub mod bubble_sort
{
    pub fn sort(vector: Vec<i64>) -> Vec<i64>
    {
        let vector_len = vector.len();
        let mut sorted_vec: Vec<i64> = vector;

        for i in 0..vector_len
        {
            for j in 0..vector_len
            {
                if sorted_vec[i] < sorted_vec[j]
                {
                    let temp = sorted_vec[j];
                    sorted_vec[j] = sorted_vec[i];
                    sorted_vec[i] = temp;
                }
            }
        }

        sorted_vec
    }
}