mod style_sheet_test;
mod style_test;
fn main() {
    println!("========================Running Tests for style macro=======================");
    style_test::run_tests();
    println!(
        "\n\n========================Running Tests for style sheet macro======================="
    );
    style_sheet_test::run_tests();
}
