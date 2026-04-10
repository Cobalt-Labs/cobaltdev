use std::fs::File;
use std::i32;
use std::io::Write;

use linfa::prelude::*;
use linfa_trees::{DecisionTree, SplitQuality};
use ndarray::prelude::*;
use ndarray::{Array2, array};

fn main() {
    let original_data: Array2<f32> = array!(
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
    );

    let feature_names = vec!["watched tv", "pet cat", "rust LOC", "ate pizza"];

    let num_features = original_data.len_of(Axis(1)) - 1;
    let features = original_data.slice(s![.., 0..num_features]).to_owned();
    let labels = original_data.column(num_features).to_owned();

    let linfa_dataset = Dataset::new(features, labels)
        .map_targets(|x| match x.to_owned() as i32 {
            i32::MIN..=4 => "sad",
            5..=7 => "ok",
            8_i32..=i32::MAX => "happy",
        })
        .with_feature_names(feature_names);

    let model = DecisionTree::params()
        .split_quality(SplitQuality::Gini)
        .fit(&linfa_dataset)
        .unwrap();

    File::create("dt.tex")
        .unwrap()
        .write_all(model.export_to_tikz().with_legend().to_string().as_bytes())
        .unwrap();
}
