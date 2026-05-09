use std::fs::File;
use std::io::Write;

use clap::Parser;

use linfa::prelude::*;
use linfa_trees::{DecisionTree, SplitQuality};
use ndarray::prelude::*;
use ndarray::{Array2, array};

fn categorize_happiness(score: i32) -> &'static str {
    match score {
        i32::MIN..=4 => "sad",
        5..=7 => "ok",
        8_i32..=i32::MAX => "happy",
    }
}

fn get_mock_data() -> Array2<f32> {
    array!(
        [1., 1., 1000., 1., 10.],
        [1., 1., 0., 1., 6.],
        [1., 0., 0., 1., 6.],
        [1., 0., 0., 1., 6.],
        [1., 0., 800., 1., 8.],
        [1., 0., 0., 0., 5.],
        [1., 0., 0., 1., 5.],
        [1., 0., 0., 0., 5.],
        [1., 1., 0., 1., 5.],
        [1., 1., 500., 1., 40.],
        [1., 0., 0., 0., 0.],
        [1., 1., 0., 0., 0.],
        [1., 1., 0., 0., 0.],
    )
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Output path for the generated TikZ file
    #[arg(short, long, default_value = "dt.tex")]
    output: String,
}

fn train_model<R: ndarray::Data<Elem = f32>, S: ndarray::Data<Elem = &'static str>>(
    dataset: &DatasetBase<ArrayBase<R, ndarray::Ix2>, ArrayBase<S, ndarray::Ix1>>,
) -> anyhow::Result<DecisionTree<f32, &'static str>> {
    let model = DecisionTree::params()
        .split_quality(SplitQuality::Gini)
        .fit(dataset)?;
    Ok(model)
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let original_data: Array2<f32> = get_mock_data();

    let feature_names = vec!["watched tv", "pet cat", "rust LOC", "ate pizza"];

    let num_features = original_data.len_of(Axis(1)) - 1;
    let features = original_data.slice(s![.., 0..num_features]).to_owned();
    let labels = original_data.column(num_features).to_owned();

    let linfa_dataset = Dataset::new(features, labels)
        .map_targets(|x| categorize_happiness(x.to_owned() as i32))
        .with_feature_names(feature_names);

    let (train, test) = linfa_dataset.split_with_ratio(0.8);

    let model = train_model(&train)?;

    let predictions = model.predict(&test);
    let cm = predictions.confusion_matrix(&test)?;
    println!("{:?}", cm);
    println!("Test accuracy: {:.2}%", 100.0 * cm.accuracy());

    File::create(&cli.output)?
        .write_all(model.export_to_tikz().with_legend().to_string().as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_categorize_happiness() {
        assert_eq!(categorize_happiness(4), "sad");
        assert_eq!(categorize_happiness(-10), "sad");
        assert_eq!(categorize_happiness(5), "ok");
        assert_eq!(categorize_happiness(7), "ok");
        assert_eq!(categorize_happiness(8), "happy");
        assert_eq!(categorize_happiness(100), "happy");
    }
}
