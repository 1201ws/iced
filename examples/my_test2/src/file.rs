use std::fs::OpenOptions;
use simple_excel_writer::*;
use std::error::Error;


pub fn storage(raw: Vec<String>) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
    .write(true)
    .create(true)
    .append(true)
    .open("foo.csv")
    .unwrap();
    let mut wtr = csv::Writer::from_writer(file);
    wtr.write_record(raw)?;
    wtr.flush()?; 
    convert_csv_to_excel("./foo.csv", "./foo.xlsx");
    Ok(()) 
    // C:/Users/student/Desktop/
}

pub fn convert_csv_to_excel(
    csv_path: &str,
    excel_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_path(csv_path)?;

    let mut wb = simple_excel_writer::Workbook::create(excel_path);
    let mut sheet = wb.create_sheet("Events");

    for n in 1..39{
        sheet.add_column(Column { width: 18.0 });
    } 

    wb.write_sheet(&mut sheet, |sw| {
        sw.append_row(simple_excel_writer::row![
            "实验人","审批人","指导老师","归属项目/人员","实验材料","实验类型","实验方案","气炮信息","探测设备",
            "靶结构信息","飞片材料","飞片材料直径","飞片材料厚度","飞片材料密度","飞片材料纵波声速","飞片材料横波声速",
            "样品材料","样品材料直径","样品材料厚度","样品材料密度","样品材料纵波声速","样品材料横波声速",
            "基板材料","基板材料直径","基板材料厚度","基板材料密度","基板材料纵波声速","基板材料横波声速",
            "窗口材料","窗口材料直径","窗口材料厚度","窗口材料密度","窗口材料纵波声速","窗口材料横波声速",
            "弹托类型","子弹重量","气压","气体"
        ])?;
        for result in rdr.records() {
            if let Ok(record) = result {
                let mut row = simple_excel_writer::Row::new();
                for field in record.iter() {
                    row.add_cell(field);
                }

                sw.append_row(row)?;
            }
        }
        Ok(())
    })?;

    wb.close()?;
    Ok(())
}