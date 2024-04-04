mod type_resolver;
mod jar_analyzer;

use std::env;
use spinners::{Spinner, Spinners};
use crate::jar_analyzer::JarAnalyzer;

fn main() {
    let file_path = env::args().nth(1).expect("No file path provided");

    let mut spinner = Spinner::new(Spinners::Point, "Extracting .jar file".into());

    let jar_analyzer = JarAnalyzer::new(file_path);
    spinner.stop_with_symbol(" ✅ ");


    let mut spinner = Spinner::new(Spinners::Point, "Gathering java class infos".into());

    let classes = jar_analyzer.get_classes();
    spinner.stop_with_symbol(" ✅ ");
}
