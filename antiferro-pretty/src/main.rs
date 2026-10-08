use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    use tenferro_runtime::Tensor;

    use antiferro_pretty::Pretty;

    let tensor = Tensor::from_vec_col_major(vec![20, 10, 2], (0..400).map(|x| x as f64).collect())?;
    tensor.show()?;

    Ok(())
}
