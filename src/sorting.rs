pub mod bubble_sort_no_flag
{
    pub fn sort(vector: Vec<i64>) -> (Vec<i64>, u64)
    {
        let vector_len = vector.len();
        let mut sorted_vec: Vec<i64> = vector;
        let mut iterations: u64 = 0;

        for _ in 0..vector_len
        {
            for j in 0..(vector_len - 1)
            {
                if sorted_vec[j] > sorted_vec[j + 1]
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

pub mod quick_sort
{
    static mut ITERATIONS: u64 = 0;

    fn partition(vector: &mut Vec<i64>, low: usize, high: usize) -> usize
    {
        let pivot = vector[high];
        let mut index;
        let low_i: isize = low.try_into().unwrap();
        let mut iter0 = false;

        if low_i - 1 < 0
        {
            index = 0;
            iter0 = true;
        }
        
        else 
        {
            index = low - 1;
        }

        for j in low..high
        {
            if vector[j] < pivot
            {
                if !iter0 
                {
                    index += 1;
                }

                let temp = vector[index];
                vector[index] = vector[j];
                vector[j] = temp;
            }
        }

        let temp = vector[index + 1];
        vector[index + 1] = vector[high];
        vector[high] = temp;

        index + 1
    }

    pub fn sort(vector: &mut Vec<i64>, low: usize, high: usize) -> u64
    {
        if low < high
        {
            let partition_ret = partition(vector, low, high);

            unsafe 
            {
                ITERATIONS += 1;
            }

            sort(vector, low, partition_ret - 1);
            sort(vector, partition_ret + 1, high);
        }

        unsafe 
        {
            return ITERATIONS
        }
    }
}
