//https://mp.weixin.qq.com/s/GOG4VdUy5R8G4COaAevBzQ 为什么选择Polars进行高性能数据分析？
// https://docs.rs/polars/0.27.2/polars/#dataframe
// 1、Polars采用了列存储格式（Arrow Arrays），这种设计不仅使得数据访问更加高效，还能显著降低内存的使用。与传统的基于行的数据处理方法相比，Polars的列存储结构让其在处理大数据集时展现出无与伦比的性能优势。
// 2、与行式存储相比，列式存储不仅提升了数据的访问效率，还使得Polars在内存管理上更加出色。在处理大规模数据时，Polars只会加载和操作所需的部分数据，极大减少了不必要的内存占用。
// 3、在现代计算中，多核处理器已经成为标配，而Polars通过内建的多线程支持，将这一优势发挥得淋漓尽致。无论是数据聚合、连接操作，还是复杂的计算任务，Polars都能利用多核并行处理进行加速。它能够大幅减少计算时间，尤其是在处理大数据集时，执行速度上与单核处理相比提高了数倍。
// 4、 Polars作为Rust的原生库，其与Rust生态的兼容性极高，意味着Rust开发者可以轻松将Polars集成到现有的项目中。Rust语言的所有权机制和内存安全特性，使得Polars既高效又安全，是Rust开发者进行数据处理的天然选择。
#[cfg(test)]
mod tests {
    use futures::future::ok;
    use polars::prelude::*;
    #[test]
    fn chunked_array_test() {
        use polars::prelude::*;

        // use iterators
        let ca: UInt32Chunked = (0..10).map(Some).collect();
        // from slices
        let ca = UInt32Chunked::new("foo".into(), &[1, 2, 3]);

        // use builders
        let mut builder = PrimitiveChunkedBuilder::<UInt32Type>::new("foo".into(), 10);
        for value in 0..10 {
            builder.append_value(value);
        }
        let ca = builder.finish();
        println!("{:?}", ca);

        let s = ca.into_series();
        // into a Column
        let s = s.into_column();
        println!("{:?}", s);
    }

    #[test]
    fn data_frame_test() -> Result<(), Box<dyn std::error::Error>> {
        use polars::df;
        use polars::prelude::*;
        // use macro
        let df = df! [
            "names" => ["a", "b", "c"],
            "values" => [1, 2, 3, 4],
            "values_nulls" => [Some(1), None, Some(3)]
        ]?;
        println!("{:?}", df);
        println!("{:?}", df["values"]);
        println!("{:?}", df["values_nulls"]);
        println!("{:?}", df.gt(&Column::new("names".into(), 22)));
        // from a Vec<Column>
        let c1 = Column::new("names".into(), &["a", "b", "c"]);
        let c2 = Column::new("values".into(), &[Some(1), None, Some(3)]);
        let df = DataFrame::new(vec![c1, c2])?;

        Ok(())
    }

    // fn data_frame() -> Result<(),()> {
    //     // 创建包含日期和温度的简单DataFrame
    //     let dates = vec!["2021-01-01", "2021-01-02", "2021-01-03"];
    //     let temperatures = vec![23, 25, 22];
    //
    //     let date_series = Series::new("date", dates);
    //     let temp_series = Series::new("temperature", temperatures);
    //
    //     let df = DataFrame::new(vec![date_series, temp_series])?;
    //     println!("{:?}", df);
    //
    //     // 筛选温度大于23的行
    //     let filtered = df.filter(&df["temperature"].gt(23)?)?;
    //     println!("{:?}", filtered);
    //
    //     // 按温度降序排序
    //     let sorted = df.sort("temperature", false)?;
    //     println!("{:?}", sorted);
    //
    //     Ok(())
    // }
}
