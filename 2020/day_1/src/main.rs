use std::collections::HashSet;

use itertools::Itertools;

fn find_2020() -> anyhow::Result<i64> {
    let (a, b) = include_str!("input.txt")
        .lines()
        .into_iter()
        .map(str::trim)
        .filter(|&line| line != "")
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .tuple_combinations()
        .find(|(a, b)| a + b == 2020)
        .expect("No pair had a sum of 2020");

    Ok(a * b)
}

fn find_2020_opt() -> anyhow::Result<i64> {
    let nums = include_str!("input.txt")
        .lines()
        .into_iter()
        .map(str::trim)
        .filter(|&line| line != "")
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter();

    let mut cache = HashSet::new();
    for num in nums {
        let b = 2020 - num;
        if cache.contains(&b) {
            return Ok(num * b);
        }
        cache.insert(num);
    }

    anyhow::bail!("No pair had a sum of 2020")
}

fn find_2020_triple_opt() -> anyhow::Result<i64> {
    let mut nums = include_str!("input.txt")
        .lines()
        .into_iter()
        .map(str::trim)
        .filter(|&line| line != "")
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?;

    nums.sort();

    for i in 0..nums.len() - 2 {
        let mut l = i + 1;
        let mut r = nums.len() - 1;
        while l < r {
            if nums[i] + nums[l] + nums[r] == 2020 {
                return Ok(nums[i] * nums[l] * nums[r]);
            } else if nums[l] + nums[r] < (2020 - nums[i]) {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }

    anyhow::bail!("No triple had a sum of 2020")
}

fn main() -> anyhow::Result<()> {
    dbg!(find_2020()?);
    dbg!(find_2020_opt()?);
    dbg!(find_2020_triple_opt()?);

    Ok(())
}
