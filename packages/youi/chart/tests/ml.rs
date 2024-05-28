use linfa::Dataset;
use linfa::dataset::AsSingleTargets;
use linfa::traits::{Fit, Predict};
use linfa_linear::LinearRegression;
use ndarray::{ ArrayBase, OwnedRepr};
use polars_core::prelude::{DataFrame, DataType, Float64Type, IndexOrder, NamedFromOwned, Series};
use polars_io::SerReader;
use youi_test::find_real_path;


#[test]
pub fn test1()->Result<(), Box<dyn std::error::Error>> {

    let path = find_real_path("chart","xy.csv");

    let df = polars_io::prelude::CsvReader::from_path(&path)?
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    let y_series = df.column("y")?.cast(&DataType::Float64)?;
    let target = y_series.f64()?;

    let mut features = df.drop("y")?;

    // 遍历列并将每个列强制转换为Float64
    for col_name in features.get_column_names_owned() {
        let casted_col = df
            .column(&col_name)?
            .cast(&DataType::Float64)
            .expect("Failed to cast column");

        features.with_column(casted_col)?;
    }

    let features_ndarray: ArrayBase<OwnedRepr<_>, _> =
        features.to_ndarray::<Float64Type>(IndexOrder::C)?;
    let target_ndarray = target.to_ndarray()?.to_owned();
    let dataset = Dataset::new(features_ndarray, target_ndarray);

    // 训练模型
    let model = LinearRegression::default().fit(&dataset)?;

    // 预测
    let pred = model.predict(&dataset);
    let py_series  = Series::from_vec("py",pred.as_single_targets().to_vec());

    features.replace_or_add("py",py_series);

    println!("{:?}",features.get_columns());
    //pred.r2(&dataset_validation)?;
    //pred.s()
    //println!("y - {:?}",y_series);
    // println!("py - {:?}",pred.map(|f|f).into_iter().collect::<Vec<f64>>());
    //println!("x: {:?}",features.get_columns());

    //DataFrame::

    // let array = array![[0., 0.], [1., 1.]];
    //
    // let dataset2 = Dataset::from(array);
    // println!("{:?}",dataset2);
    //
    // let dataset = linfa_datasets::diabetes();
    // println!("{:?}",dataset);
    // //
    // let model = LinearRegression::default().fit(&dataset).unwrap();
    // let pred = model.predict(&dataset);
    // let r2 = pred.r2(&dataset).unwrap();
    //
    // println!("r2 from prediction: {}", r2);
    // println!("r2 from prediction: {:?}", pred);

    Ok(())
}
