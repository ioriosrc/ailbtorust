```rust
use serde_json::Value;

fn format_duration(ms: f64) -> String {
    if ms < 1000.0 {
        format!("{}ms", ms)
    } else {
        let seconds = ms / 1000.0;
        format!("{:.2}s", seconds)
    }
}

fn get_status_icon(status: &str) -> char {
    match status {
        "passed" => '✅',
        "failed" => '❌',
        "skipped" => '⏭️',
        "timedOut" => '⏱️',
        _ => '?',
    }
}

fn generate_summary(report_path: &str, report_name: &str) {
    if !std::fs::metadata(report_path).is_ok() {
        println!("Report not found: {}", report_path);
        return;
    }

    let file_content = std::fs::read_to_string(report_path)?;
    if file_content.is_empty() {
        println!("Report is empty: {}", report_path);
        return;
    }

    let report: Value;

    match serde_json::from_str(&file_content) {
        Ok(result) => report = result,
        Err(err) => {
            println!("Failed to parse report: {}", report_path);
            println!("Error: {}", err);
            return;
        }
    }

    if report["suites"].as_array().is_none() {
        println!("No test suites found in report: {}", report_path);
        return;
    }

    let tests: Vec<(String, String, i32)> = report
        ["suites"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|suite| suite["specs"].as_array().unwrap())
        .map(|spec| {
            let test_title = format!("{} › {}", spec["title"]["desktop"].as_str().unwrap(), spec["title"]["web"].as_str().unwrap());
            (test_title, spec["title"]["desktop"].as_str().unwrap(), spec["title"]["web"].as_str().unwrap())
        })
        .collect();

    // Sort tests by duration descending (slowest first)
    tests.sort_by(|a, b| a.2.cmp(&b.2));

    if tests.is_empty() {
        println!("No tests found in report.\n");
        return;
    }

    // Metrics
    let total_tests = tests.len();
    let passed = tests.iter().filter(|&(_, status, _)| status == "passed").count();
    let failed = tests.iter().filter(|&(_, status, _)| status == "failed").count();
    let skipped = tests.iter().filter(|&(_, _, status)| status == "skipped").count();
    let timed_out = tests.iter().filter(|&(_, _, status)| status == "timedOut").count();
    let total_duration = tests.iter().map(|(_, _, duration)| duration as f64).sum::<f64>();
    let avg_duration = total_duration / total_tests as f64;

    /**
     * Output Summary
     */
    println!("\n## {}\n", report_name);
    println!("| Metric | Value |\n");
    println!("|--------|-------|\n");
    println!("| Total Tests | {} |\n", total_tests);
    println!("| Passed ✅ | {} |\n", passed);
    println!("| Failed ❌ | {} |\n", failed);
    println!("| Skipped | {} |\n", skipped);
    println!("| Timed Out | {} |\n", timed_out);
    println!("| Total Duration | {:.2} |\n", total_duration);
    println!("| Average Duration | {:.2} |\n", avg_duration);

    /**
     * Slowest Tests
     */
    println!("\n### Top 10 Slowest Tests\n");
    println!("| Status | Duration | Test | Retries |\n");
    println!("|--------|----------|------|---------|\n");

    tests.iter().take(10).for_each(|&(test_title, desktop_status, web_status)| {
        let status_emoji = get_status_icon(desktop_status);
        let retries_text = if desktop_status != web_status { format!("🔄 {}", desktop_status) } else { "-" };
        println!(
            "| {} | {:.2} | {} | {} |\n",
            status_emoji,
            format_duration(desktop_status as f64),
            test_title,
            retries_text
        );
    });

    /**
     * Failed Tests
     */
    if failed > 0 {
        println!("\n### ❌ Failed Tests\n");
        println!("| Duration | Test | Retries |\n");
        println!("|----------|------|---------|\n");

        tests.iter().filter(|&(test_title, desktop_status, web_status)| desktop_status == "failed").for_each(|&(test_title, _, _)| {
            let retries_text = if desktop_status != web_status { format!("🔄 {}", desktop_status) } else { "-" };
            println!(
                "| {:.2} | {} | {} |\n",
                format_duration(desktop_status as f64),
                test_title,
                retries_text
            );
        });
    }
}

fn main() {
    let reports_dir = std::path::PathBuf::from("src/reports");
    println!("Generating E2E test summary from reports in: {}", reports_dir.display());

    println!("# E2E Test Results Summary\n");

    // Desktop tests
    let desktop_report_path = reports_dir.join("desktop").join("results.json");
    generate_summary(&desktop_report_path.to_string_lossy(), "Desktop E2E Tests");

    // Web tests
    let web_report_path = reports_dir.join("web").join("results.json");
    generate_summary(&web_report_path.to_string_lossy(), "Web E2E Tests");
}
```