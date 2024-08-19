pub mod bubble_sort_no_flag
{
    pub fn sort(vector: &mut Vec<i64>) -> u64
    {
        let vector_len = vector.len();
        let mut iterations: u64 = 0;

        for _ in 0..vector_len
        {
            for j in 0..(vector_len - 1)
            {
                if vector[j] > vector[j + 1]
                {
                    let temp = vector[j + 1];
                    vector[j + 1] = vector[j];
                    vector[j] = temp;                    
                }

                iterations += 1;
            }
        }

        iterations
    }
}

pub mod bubble_sort_with_flag
{
    pub fn sort(vector: &mut Vec<i64>) -> u64
    {
        let vector_len = vector.len();
        let mut changed: bool;
        let mut iterations: u64 = 0;

        for _ in 0..vector_len
        {
            changed = false;

            for j in 0..(vector_len - 1)
            {
                if vector[j] > vector[j + 1]
                {
                    let temp = vector[j + 1];
                    vector[j + 1] = vector[j];
                    vector[j] = temp;

                    changed = true;
                }

                iterations += 1;
            }

            if !changed 
            {
                break;
            }
        }

        iterations
    }
}

pub mod quick_sort
{
    static mut ITERATIONS: u64 = 0;

    fn partition(vector: &mut Vec<i64>, low: isize, high: isize) -> isize
    {
        let pivot = vector[high as usize];
        let mut index: isize;

        index = low as isize - 1;


        for j in low..=high - 1
        {
            if vector[j as usize] <= pivot
            {
                index += 1;
                let temp = vector[index as usize];
                vector[index as usize] = vector[j as usize];
                vector[j as usize] = temp;
            }
        }

        let temp = vector[(index + 1) as usize];
        vector[(index + 1) as usize] = vector[high as usize];
        vector[high as usize] = temp;

        index + 1
    }

    pub fn sort(vector: &mut Vec<i64>, low: isize, high: isize) -> u64
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

pub mod selection_sort
{
    pub fn sort(vector: &mut Vec<i64>) -> u64
    {
        let mut min_ind: usize;
        let mut iterations: u64 = 0;

        for i in 0..vector.len()
        {
            min_ind = i;

            for j in (i + 1)..vector.len()
            {
                if vector[j] < vector[min_ind]
                {
                    min_ind = j;
                }

                iterations += 1;
            }

            let temp = vector[i];
            vector[i] = vector[min_ind];
            vector[min_ind] = temp;
        }

        iterations
    }
}

pub mod bogo_sort
{
    use rand::prelude::*;
    use super::quick_sort;

    pub fn sort(vector: &mut Vec<i64>) -> u64
    {
        let mut iterations: u64 = 0;
        let mut vector_clone_compare: Vec<i64> = vector.clone();
        let vec_size = vector.len();
        let mut rng = rand::thread_rng();
        let mut rand_index1;
        let mut rand_index2;

        quick_sort::sort(&mut vector_clone_compare, 0, (vec_size - 1) as isize);

        while !compare_eq(vector.clone(), vector_clone_compare.clone())
        {
            rand_index1 = rng.gen_range(0..vec_size);
            rand_index2 = rng.gen_range(0..vec_size);

            let temp = vector[rand_index1];
            vector[rand_index1] = vector[rand_index2];
            vector[rand_index2] = temp;

            iterations += 1;
        }

        iterations
    }

    fn compare_eq(vec1: Vec<i64>, vec2: Vec<i64>) -> bool
    {
        for i in 0..vec1.len()
        {
            if vec1[i] != vec2[i]
            {
                return false;
            }
        }

        true
    }
}

pub mod radix_sort
{
    static mut ITERATIONS: u64 = 0;

    pub fn sort(vector: &mut Vec<i64>) -> u64
    {
        let max = get_max(vector.to_vec());
        let mut exp: usize = 1;

        while max / exp as i64 > 0
        {
            count_sort(vector, exp);

            exp = exp * 10;
        }

        unsafe 
        {
            ITERATIONS
        }
    }

    fn count_sort(vector: &mut Vec<i64>, exp: usize) -> ()
    {
        let limit = vector.len();
        let mut temp_vector: Vec<i64> = vec![0; limit];
        let mut count_vector: Vec<usize> = vec![0; limit];
        let mut iter_vec: Vec<i64> = vec![0; 3];

        for i in 0..vector.len()
        {
            iter_vec[0] = iter_vec[0] + 1;
            count_vector[((vector[i] / exp as i64) % 10) as usize] = count_vector[((vector[i] / exp as i64) % 10) as usize] + 1;
        }

        for i in 1..limit
        {
            iter_vec[1] = iter_vec[1] + 1;
            count_vector[i] = count_vector[i] + count_vector[i - 1];
        }

        for i in (0..=(vector.len() - 1)).rev()
        {
            iter_vec[2] = iter_vec[2] + 1;
            let ind = ((vector[i] / exp as i64) % 10) as usize;
            temp_vector[count_vector[ind] - 1] = vector[i];
            count_vector[ind] = count_vector[ind] - 1;  
        }

        for i in 0..limit
        {
            vector[i] = temp_vector[i];
        }

        unsafe 
        {
            ITERATIONS = ITERATIONS + get_max(iter_vec) as u64;
        }
    }

    fn get_max(vector: Vec<i64>) -> i64
    {
        *vector.iter().max().unwrap()
    }
}