pub fn run() {
    let items = vec![vec![1, 2], vec![3, 2], vec![2, 4], vec![5, 6], vec![3, 5]];
    let queries = vec![1, 2, 3, 4, 5, 6];
    println!("Res: {:?}", Solution::maximum_beauty(items, queries));
}

struct Solution {}

impl Solution {
    pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        // (index_in_original, query_price)
        let mut sorted_queries: Vec<(usize, i32)> = queries
            .iter()
            .enumerate()
            .map(|(index_in_orig, &price_query)| (index_in_orig, price_query))
            .collect();
        // sort by query_price
        sorted_queries.sort_by_key(|(_, price_query)| *price_query);

        // sort by price
        let mut sorted_items = items.clone();
        sorted_items.sort_by_key(|elem| elem[0]);

        let mut sol = vec![0; queries.len()];

        let mut max_beauty_so_far = 0;
        let mut previous_price = 0;
        let mut i = 0_usize;
        let mut j = 0_usize;

        while j < sorted_queries.len() {
            if i != sorted_items.len() && previous_price <= sorted_queries[j].1 {
                let current_price = sorted_items[i][0];

                if current_price > sorted_queries[j].1 {
                    sol[sorted_queries[j].0] = max_beauty_so_far;
                    j += 1;
                } else {
                    previous_price = current_price;
                    if max_beauty_so_far < sorted_items[i][1] {
                        max_beauty_so_far = sorted_items[i][1];
                    }
                    i += 1;
                }
            } else {
                sol[sorted_queries[j].0] = max_beauty_so_far;
                j += 1;
            }
        }

        sol
    }
}
