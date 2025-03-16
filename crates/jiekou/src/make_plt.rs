

use plotters::prelude::*;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // 创建图像文件
    let root = BitMapBackend::new("D:/一列柱状图.png", (400, 300)).into_drawing_area();
    root.fill(&WHITE)?;

    // 创建绘图区域
    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("柱状图示例", ("SimSun", 15)) // 使用宋体
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(
            ["A", "B", "C", "D"].into_segmented(),
            0..90, // Y 轴范围
        )?;

    chart.configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .x_desc("类别")
        .y_desc("数量")
        .label_style(("Times New Roman", 12))
        .draw()?;

    // 数据
    let x_labels = vec!["A", "B", "C", "D"];
    let y_values = vec![65, 70, 75, 80];
    let colors = vec![RED, GREEN, BLUE, MAGENTA];

    // 绘制柱状图
    chart.draw_series(
        x_labels.iter().zip(y_values.iter()).zip(colors.iter()).map(|((x, y), color)| {
            let x_label = SegmentValue::CenterOf(x);
            Rectangle::new([(x_label.clone(), 0), (x_label, *y)], *color.filled())
        }),
    )?
    .label("数量")
    .legend(|(x, y)| Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], BLUE.filled()));

    // 添加标签
    chart.draw_series(
        x_labels.iter().zip(y_values.iter()).map(|(x, y)| {
            let x_label = SegmentValue::CenterOf(x);
            Text::new(format!("{}", y), (x_label, *y + 2), ("Times New Roman", 10).into_font())
        }),
    )?;

    // 保存文件
    root.present()?;
    println!("图表已保存到 D:/一列柱状图.png");
    Ok(())
}
