fn count_safe_reports(reports: Vec<Vec<i32>>) {
    let safe_report_count: u32 = reports
        .iter()
        .map(|report| {
            let differences: Vec<i32> = report.windows(2).map(|a| a[1] - a[0]).collect();
            let gradient_check = if differences[0] < 0 {
                |x: i32| x < 0
            } else {
                |x: i32| x > 0
            };
            differences
                .iter()
                .all(|&x| gradient_check(x) && (x.abs() >= 1 && x.abs() <= 3)) as u32
        })
        .sum();

    println!("Number of safe reports: {safe_report_count}");
}

fn count_safe_reports_dampener(reports: Vec<Vec<i32>>) {
    let safe_report_count: u32 = reports
        .iter()
        .map(|report| {
            let validate = |test_report: &Vec<i32>| {
                let differences: Vec<i32> = test_report.windows(2).map(|a| a[1] - a[0]).collect();
                let gradient_check = if differences[0] < 0 {
                    |x: i32| x < 0
                } else {
                    |x: i32| x > 0
                };
                differences
                    .iter()
                    .all(|&x| gradient_check(x) && (x.abs() >= 1 && x.abs() <= 3))
            };

            (validate(report)
                || (0..report.len()).into_iter().any(|skip_index| {
                    let test_report: Vec<i32> = report[..skip_index]
                        .iter()
                        .chain(report[skip_index + 1..].iter())
                        .cloned()
                        .collect();
                    validate(&test_report)
                })) as u32
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
            if parts.len() < 2 {
                panic!("Invalid input, expected more than 1 number");
            }

            parts
                .into_iter()
                .map(|s| s.parse().expect("Failed to parse an integer"))
                .collect()
        })
        .collect();

    count_safe_reports(reports.clone());
    count_safe_reports_dampener(reports.clone());
}
