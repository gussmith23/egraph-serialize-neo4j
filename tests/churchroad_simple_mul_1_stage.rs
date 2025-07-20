#[test]
fn simple_mul_1_stage() {
    // Ensure NEO4J_URI, NEO4J_USER, and NEO4J_PASS are set in the environment.
    let uri = std::env::var("NEO4J_URI").expect("NEO4J_URI not set");
    let user = std::env::var("NEO4J_USERNAME").expect("NEO4J_USERNAME not set");
    let pass = std::env::var("NEO4J_PASSWORD").expect("NEO4J_PASSWORD not set");

    // JSON path relative to CARGO_MANIFEST_DIR (which is the root of the repo).
    let json_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("churchroad_simple_mul_1_stage.json");
    // Run the binary on the test.
    let output = std::process::Command::new("cargo")
        .args([
            "run",
            "--bin",
            "egraph-serialize-neo4j",
            "--",
            "--json",
            json_path.to_str().unwrap(),
            "--uri",
            &uri,
            "--username",
            &user,
            "--password",
            &pass,
        ])
        .output()
        .expect("failed to execute process");

    assert!(
        output.status.success(),
        "Process failed with status: {}\nstdout: {}\nstderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
