//https://mp.weixin.qq.com/s/GOG4VdUy5R8G4COaAevBzQ 为什么选择Polars进行高性能数据分析？
// https://docs.pola.rs/user-guide/getting-started/#reading-writing
// 1、Polars采用了列存储格式（Arrow Arrays），这种设计不仅使得数据访问更加高效，还能显著降低内存的使用。与传统的基于行的数据处理方法相比，Polars的列存储结构让其在处理大数据集时展现出无与伦比的性能优势。
// 2、与行式存储相比，列式存储不仅提升了数据的访问效率，还使得Polars在内存管理上更加出色。在处理大规模数据时，Polars只会加载和操作所需的部分数据，极大减少了不必要的内存占用。
// 3、在现代计算中，多核处理器已经成为标配，而Polars通过内建的多线程支持，将这一优势发挥得淋漓尽致。无论是数据聚合、连接操作，还是复杂的计算任务，Polars都能利用多核并行处理进行加速。它能够大幅减少计算时间，尤其是在处理大数据集时，执行速度上与单核处理相比提高了数倍。
// 4、 Polars作为Rust的原生库，其与Rust生态的兼容性极高，意味着Rust开发者可以轻松将Polars集成到现有的项目中。Rust语言的所有权机制和内存安全特性，使得Polars既高效又安全，是Rust开发者进行数据处理的天然选择。
#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use polars::prelude::*;
    #[test]
    fn chunked_array_test() {
        use polars::prelude::*;

        // use iterators
        let _ca: UInt32Chunked = (0..10).map(Some).collect();
        // from slices
        let _ca = UInt32Chunked::new("foo".into(), &[1, 2, 3]);

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
        let mut df: DataFrame = df!(
            "name" => ["Alice Archer", "Ben Brown", "Chloe Cooper", "Daniel Donovan"],
            "birthdate" => [
                NaiveDate::from_ymd_opt(1997, 1, 10).unwrap(),
                NaiveDate::from_ymd_opt(1985, 2, 15).unwrap(),
                NaiveDate::from_ymd_opt(1983, 3, 22).unwrap(),
                NaiveDate::from_ymd_opt(1981, 4, 30).unwrap(),
            ],
            "weight" => [57.9, 72.5, 53.6, 83.1],  // (kg)
            "height" => [1.56, 1.77, 1.65, 1.75],  // (m)
        )?;
        println!("{:?}", df);
        // from a Vec<Column>
        let c1 = Column::new("names".into(), &["a", "b", "c"]);
        let c2 = Column::new("values".into(), &[Some(1), None, Some(3)]);
        let df1 = DataFrame::new(vec![c1, c2])?;
        println!("{:?}", df1);

        Ok(())
    }

    #[test]
    fn test_reading_writing() -> Result<(), Box<dyn std::error::Error>> {
        use std::fs::File;

        let mut df: DataFrame = df!(
            "name" => ["Alice Archer", "Ben Brown", "Chloe Cooper", "Daniel Donovan"],
            "birthdate" => [
                NaiveDate::from_ymd_opt(1997, 1, 10).unwrap(),
                NaiveDate::from_ymd_opt(1985, 2, 15).unwrap(),
                NaiveDate::from_ymd_opt(1983, 3, 22).unwrap(),
                NaiveDate::from_ymd_opt(1981, 4, 30).unwrap(),
            ],
            "weight" => [57.9, 72.5, 53.6, 83.1],  // (kg)
            "height" => [1.56, 1.77, 1.65, 1.75],  // (m)
        )
        .unwrap();
        println!("{}", df);

        let fileName = "polars_data_test.csv";
        let mut file = File::create(fileName).expect("could not create file");
        CsvWriter::new(&mut file)
            .include_header(true)
            .with_separator(b',')
            .finish(&mut df)?;
        let df_csv = CsvReadOptions::default()
            .with_infer_schema_length(None)
            .with_has_header(true)
            .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
            .try_into_reader_with_file_path(Some(fileName.into()))?
            .finish()?;
        println!("{}", df_csv);

        let result = df
            .clone()
            .lazy()
            .select([
                col("name"),
                col("birthdate").dt().year().alias("birth_year"),
                (col("weight") / col("height").pow(2)).alias("bmi"),
            ])
            .collect()?;
        println!("{}", result);

        let result = df
            .clone()
            .lazy()
            .select([
                col("name"),
                (cols(["weight", "height"]) * lit(0.95))
                    .round(2)
                    .name()
                    .suffix("-5%"),
            ])
            .collect()?;
        println!("{}", result);

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
