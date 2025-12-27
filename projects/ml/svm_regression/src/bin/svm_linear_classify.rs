use linfa::prelude::*;
use linfa_preprocessing::linear_scaling::LinearScaler;
use linfa_svm::Svm;
use ndarray::array;

fn main() {
    let train_features = array![
        [1.0, 1.0],
        [2.0, 2.0],
        [3.0, 3.0],
        [4.0, 4.0],
        [5.0, 5.0],
        [6.0, 6.0],
    ];
    let train_labels = array![false, false, false, true, true, true];

    let train_dataset = DatasetBase::new(train_features, train_labels);
    let train_scaler = LinearScaler::<f64>::standard().fit(&train_dataset).unwrap();
    let train_dataset_scaled = train_scaler.transform(train_dataset);
    println!("== Scaled training data ==\n{:?}\n", train_dataset_scaled);
    let model = Svm::<f64, bool>::params()
        .pos_neg_weights(0.2, 0.2)
        .fit(&train_dataset_scaled)
        .expect("Training failed");

    println!("== SVM Model ==\n{:?}\n", model);

    // Prediction
    let valid_data = array![[2., 2.], [3., 3.], [4., 4.], [5., 5.]];
    let valid_labels_dummy = array![false, false, false, false]; // Dummy labels for dataset creation
    let valid_dataset = DatasetBase::new(valid_data, valid_labels_dummy);
    let valid_dataset_scaled = train_scaler.transform(valid_dataset);

    let prediction = model.predict(&valid_dataset_scaled);

    println!("Prediction results: {:?}", prediction);
}
