mod style;
mod style_sheet;
fn main() {
    println!("========================Running Tests for style macro=======================");
    style::run_tests();
    println!(
        "\n\n========================Running Tests for style sheet macro======================="
    );
    style_sheet::run_tests();
}
