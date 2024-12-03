fn count_safe_reports(reports: &[Vec<i32>]) {
    let safe_report_count: u32 = reports
        .iter()
        .map(|report| {
            if report.len() <= 1 {
                return 0;
            }

            let differences: Vec<i32> = report.windows(2).map(|a| a[1] - a[0]).collect();
            let gradient_check = if differences[0] < 0 {
                |x: i32| x < 0
            } else {
                |x: i32| x > 0
            };
            u32::from(
                differences
                    .iter()
                    .all(|&x| gradient_check(x) && (1..4).contains(&x.abs())),
            )
        })
        .sum();

    println!("Number of safe reports: {safe_report_count}");
}

fn count_safe_reports_dampener(reports: &[Vec<i32>]) {
    let safe_report_count: u32 = reports
        .iter()
        .map(|report| {
            let validate = |test_report: &Vec<i32>| {
                if test_report.len() <= 1 {
                    return false;
                }

                let differences: Vec<i32> = test_report.windows(2).map(|a| a[1] - a[0]).collect();
                let gradient_check = if differences[0] < 0 {
                    |x: i32| x < 0
                } else {
                    |x: i32| x > 0
                };
                differences
                    .iter()
                    .all(|&x| gradient_check(x) && (1..4).contains(&x.abs()))
            };

            u32::from(
                validate(report)
                    || (0..report.len()).any(|skip_index| {
                        let test_report: Vec<i32> = report[..skip_index]
                            .iter()
                            .chain(report[skip_index + 1..].iter())
                            .copied()
                            .collect();
                        validate(&test_report)
                    }),
            )
        })
        .sum();

    println!("Number of safe reports with dampener: {safe_report_count}");
}

fn main() {
    let input = include_str!("input.txt");
    let lines = input.lines();

    let reports: Vec<Vec<i32>> = lines
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            assert!(
                parts.len() >= 2,
                "Invalid input, expected more than 1 number"
            );

            parts
                .into_iter()
                .map(|s| s.parse().expect("Failed to parse an integer"))
                .collect()
        })
        .collect();

    count_safe_reports(&reports);
    count_safe_reports_dampener(&reports);
}
