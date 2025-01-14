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
        let df: DataFrame = df!(
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

        let file_name = "polars_data_test.csv";
        let mut file = File::create(file_name).expect("could not create file");
        CsvWriter::new(&mut file)
            .include_header(true)
            .with_separator(b',')
            .finish(&mut df)?;
        let df_csv = CsvReadOptions::default()
            .with_infer_schema_length(None)
            .with_has_header(true)
            .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
            .try_into_reader_with_file_path(Some(file_name.into()))?
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

        let result = df
            .clone()
            .lazy()
            .filter(col("birthdate").dt().year().lt(lit(1990)))
            .collect()?;
        println!("{}", result);

        // Use `group_by_stable` if you want the Python behaviour of `maintain_order=True`.
        let result = df
            .clone()
            .lazy()
            .group_by([(col("birthdate").dt().year() / lit(10) * lit(10)).alias("decade")])
            .agg([len()])
            .collect()?;
        println!("{}", result);

        let result = df
            .clone()
            .lazy()
            .group_by([(col("birthdate").dt().year() / lit(10) * lit(10)).alias("decade")])
            .agg([
                len().alias("sample_size"),
                col("weight").mean().round(2).alias("avg_weight"),
                col("height").max().alias("tallest"),
            ])
            .collect()?;
        println!("{}", result);

        let result = df
            .clone()
            .lazy()
            .with_columns([
                (col("birthdate").dt().year() / lit(10) * lit(10)).alias("decade"),
                col("name").str().split(lit(" ")).list().first(),
            ])
            .select([all().exclude(["birthdate"])])
            .group_by([col("decade")])
            .agg([
                col("name"),
                cols(["weight", "height"])
                    .mean()
                    .round(2)
                    .name()
                    .prefix("avg_"),
            ])
            .collect()?;
        println!("{}", result);

        let df2: DataFrame = df!(
            "name" => ["Ben Brown", "Daniel Donovan", "Alice Archer", "Chloe Cooper"],
            "parent" => [true, false, false, false],
            "siblings" => [1, 2, 3, 4],
        )
        .unwrap();

        let result = df
            .clone()
            .lazy()
            .join(
                df2.clone().lazy(),
                [col("name")],
                [col("name")],
                JoinArgs::new(JoinType::Left),
            )
            .collect()?;

        println!("{}", result);

        let df3: DataFrame = df!(
            "name" => ["Ethan Edwards", "Fiona Foster", "Grace Gibson", "Henry Harris"],
            "birthdate" => [
                NaiveDate::from_ymd_opt(1977, 5, 10).unwrap(),
                NaiveDate::from_ymd_opt(1975, 6, 23).unwrap(),
                NaiveDate::from_ymd_opt(1973, 7, 22).unwrap(),
                NaiveDate::from_ymd_opt(1971, 8, 3).unwrap(),
            ],
            "weight" => [67.9, 72.5, 57.6, 93.1],  // (kg)
            "height" => [1.76, 1.6, 1.66, 1.8],  // (m)
        )
        .unwrap();

        let result = concat(
            [df.clone().lazy(), df3.clone().lazy()],
            UnionArgs::default(),
        )?
        .collect()?;
        println!("{}", result);

        // use polars_ops::series::*;
        let result = df
            .clone()
            .lazy()
            .filter(
                col("birthdate")
                    .is_between(
                        lit(NaiveDate::from_ymd_opt(1982, 12, 31).unwrap()),
                        lit(NaiveDate::from_ymd_opt(1996, 1, 1).unwrap()),
                        ClosedInterval::Both,
                    )
                    .and(col("height").gt(lit(1.7))),
            )
            .collect()?;
        println!("{}", result);

        Ok(())
    }

    #[test]
    fn test_reader() {
        let df = CsvReadOptions::default()
            .try_into_reader_with_file_path(Some("test_data.csv".into()))
            .unwrap()
            .finish()
            .unwrap();
        println!("{}", df.head(Some(10)));
        let df_small = df
            .clone()
            .lazy()
            .filter(
                col("type").gt(10001).or(col("warehouse_id")
                    .is_in(lit(Series::new("".into(), &[6567548941112i64, 6549184383297i64])))),
            )
            .select([
                col("warehouse_id"),
                col("product_id"),
                col("type"),
                col("cnt"),
                col("cnt2"),
            ])
            .group_by([col("type")])
            .agg([len(), col("cnt").sum(), col("cnt2").sum()])
            .collect();
        println!("{:?}", df_small);
    }
}
