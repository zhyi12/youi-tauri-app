use rhai::plugin::*;
use youi_calamine::DataType;
use crate::{Area, Cell};

#[export_module]
pub mod grid_module {
    use grid::Grid;
    use itertools::Itertools;
    use rhai::{Dynamic, NativeCallContext};
    use youi_calamine::DataType;
    use crate::{Area, Cell};
    use crate::formula::func::{find_cell_text,area_value_list,to_f64};

    pub type G = Grid<Cell>;

    pub type D = DataType;

    pub type A = Area;

    ///
    ///
    ///
    #[rhai_fn(name = "SET")]
    pub fn set(mut g:G,row_index:i64,col_index:i64,value:D)->D{
        let mut opt_cell = g.get_mut(row_index,col_index);
        match opt_cell {
            None => {
                println!("{:?}",g);
            }
            Some(ref mut cell) => {
                cell.text = Some(value);
            }
        };
        D::Empty
    }

    #[rhai_fn(name = "SET")]
    pub fn set_f64(g:G,row_index:i64,col_index:i64,value:f64)->D{
        set(g,row_index,col_index,D::Float(value))
    }

    ///
    /// row_index,col_index坐标从1开始
    ///
    #[rhai_fn(name = "GET")]
    pub fn get(g:G,row_index:i64,col_index:i64)->D{
        find_cell_text(&g,(row_index-1) as u32,(col_index-1) as u32)
    }

    ///
    /// VLOOKUP (要查找的项、要查找位置、区域中包含要返回的值的列号、返回近似匹配（用1/TRUE指示）或精确匹配（用0/FALSE指示）
    ///
    #[rhai_fn(name = "VLOOKUP")]
    pub fn vlookup(d:DataType,area_g:G,area:A,offset:i64,_a:i64)->D{
        let opt_row_index = (area.start_row..area.end_row+1).find(|row_idx|{
            let opt_cell = area_g.get(*row_idx,area.start_col);
            match opt_cell {
                None => false,
                Some(cell) => match &cell.text {
                    None => false,
                    Some(text) => {
                        d.eq(text)
                    }
                }
            }
        });

        match opt_row_index {
            None => D::Float(0.),
            Some(row_index) => {
                find_cell_text(&area_g, row_index as u32, (area.start_col + offset as u32 -1) as u32)
            }
        }
    }

    ///
    /// 区域求和
    ///
    #[rhai_fn(name = "SUM")]
    pub fn sum(g:G,area:A)->f64{
        (area.start_row..area.end_row+1).map(|row_index|
            (area.start_col..area.end_col+1).map(|col_index|{
                to_f64(&find_cell_text(&g,row_index,col_index))
            }).collect::<Vec<f64>>()
        ).flatten().sum::<f64>()
    }

    ///
    /// 参数的坐标从1开始
    ///
    #[rhai_fn(name = "AREA")]
    pub fn area(start_row:i64,end_row:i64,start_col:i64,end_col:i64)->A{
        Area{
            start_row:(start_row-1) as u32,
            end_row: (end_row-1) as u32,
            start_col: (start_col-1) as u32,
            end_col: (end_col-1) as u32
        }
    }

    ///
    ///
    ///
    #[rhai_fn(name = "COUNTA")]
    pub fn count_a(g:G,area:A)->i64{
        (area.start_row..area.end_row+1).map(|row_index|
            (area.start_col..area.end_col+1).map(|col_index|{
                let cell  = g.get(row_index,col_index);
                match cell {
                    None=>0,
                    Some(_)=>1,
                }
            }).collect::<Vec<i64>>()
        ).flatten().sum::<i64>()
    }

    ///
    /// 平均值
    ///
    #[rhai_fn(name = "AVERAGE")]
    pub fn average(params:Vec<Dynamic>)->f64{

        let count = params.len();
        let values = (0..count).map(|idx|{
            let param = &params[idx];
            //单元格数据
            let opt_data = param.clone().try_cast::<D>();

            match opt_data {
                None=>{
                    //区域数据
                    let opt_area = param.clone().try_cast::<A>();
                    match &opt_area {
                        None=>vec![],
                        Some(area)=>{
                            if idx>0{
                                let opt_grid = params[idx-1].clone().try_cast::<G>();
                                match &opt_grid {
                                    None=>vec![],
                                    Some(grid)=>area_value_list(grid,area)
                                }
                            }else{
                                vec![]
                            }
                        }
                    }
                },
                Some(data)=>{
                    match &data {
                        DataType::Int(x) => vec![(*x as f64)],
                        DataType::Float(x) => vec![(*x)],
                        _=>vec![]
                    }
                }
            }
        }).flatten().collect::<Vec<f64>>();

        if values.is_empty(){
            0.
        }else {
            values.iter().sum::<f64>()/(values.len() as f64)
        }
    }

    ///
    /// 判断是否满足某个条件，如果满足返回一个值，如果不满足则返回另一个值。
    // 语法
    // IF(logical_test,value_if_true,value_if_false)
    // ▪	Logical_test:  是任何可能被计算为 TRUE 或 FALSE 的数值或表达式。
    // ▪	Value_if_true:  是 Logical_test 为 TRUE 时的返回值。如果忽略，则返回 TRUE。IF 函数最多可嵌套七层。
    // ▪	Value_if_false:  是当 Logical_test 为 FALSE 时的返回值。如果忽略，则返回 FALSE
    ///
    #[rhai_fn(name = "IF")]
    pub fn logic_if(logical_test:bool,value_if_true:D,value_if_false:D)->D{
        if logical_test{
            value_if_true
        }else{
            value_if_false
        }
    }

    ///
    ///  IFS
    ///  检查是否满足一个或多个条件并返回与第一个 TRUE 条件对应的值
    ///  语法
    ///  IFS(logical_test,value_if_true,...)
    /// ▪	Logical_test1:  是任何可求值为 TRUE 或 FALSE 的值或表达式
    ///
    #[rhai_fn(name = "IFS")]
    pub fn logic_ifs(_logical_test:bool)->D{
        D::Empty
    }
    ///
    /// 连接列表或文本字符串区域
    /// 语法
    /// CONCAT(text1,...)
    /// ▪	Text1: text1,text2,... 是要与单个文本字符串联接的 1 到 254 个文本字符串或区域
    /// ▪	Text2: text1,text2,... 是要与单个文本字符串联接的 1 到 254 个文本字符串或区域
    /// ▪	Text3: text1,text2,... 是要与单个文本字符串联接的 1 到 254 个文本字符串或区域
    /// ▪	Text4: text1,text2,... 是要与单个文本字符串联接的 1 到 254 个文本字符串或区域
    ///
    #[rhai_fn(name = "CONCAT")]
    pub fn concat(params:Vec<Dynamic>)->D{
        // 字符
        D::String(params.iter().map(|p|{
            if p.is_float(){
                format!("{}",p.as_float().unwrap())
            }else if p.is_int(){
                format!("{}",p.as_int().unwrap())
            }else if p.is_string(){
                p.clone_cast::<String>()
            }else{
                let opt_data = p.clone().try_cast::<D>();
                match opt_data {
                    None=>"".to_string(),
                    Some(d)=>str(d)
                }
            }
        }).join(""))
    }

    ///
    ///
    /// 使用分隔符连接列表或文本字符串区域
    /// 语法
    /// TEXTJOIN(delimiter,ignore_empty,text1,...)
    /// ▪	Delimiter: 要在每个文本项之间插入的字符或字符串
    /// ▪	Ignore_empty: 如果为 TRUE (默认)，则忽略空单元格
    ///
    #[rhai_fn(name = "TEXTJOIN")]
    pub fn text_join(_delimiter:String,_ignore_empty:bool)->D{
        D::Empty
    }

    ///
    /// 语法
    /// ROUND(number,num_digits)
    /// ▪	Number: 要四舍五入的数值
    /// ▪	Num_digits: 执行四舍五入时采用的位数。如果此参数为负数，则圆整到小数点的左边；如果此参数为零，则圆整到最接近的整数
    ///
    #[rhai_fn(name = "ROUND")]
    pub fn round(d:D)->D{
        D::Int(f64(d).round() as i64)
    }

    #[rhai_fn(name = "ROUND")]
    pub fn round_f64(value:f64)->D{
        D::Int(value.round() as i64)
    }

    ///从一个文本字符串的第一个字符开始返回指定个数的字符
    // 语法
    // LEFT(text,num_chars)
    // ▪	Text: 要提取字符的字符串
    // ▪	Num_chars: 要 LEFT 提取的字符数；如果忽略，为 1
    #[rhai_fn(name = "LEFT")]
    pub fn left_str(value:String,num_chars:i64)->String{
        (value.as_str()[0..num_chars as usize]).to_string()
    }

    #[rhai_fn(name = "LEFT")]
    pub fn left(value:D,num_chars:i64)->String{
        (&str(value)[0..num_chars as usize]).to_string()
    }

    #[rhai_fn(name = "RIGHT")]
    pub fn right(value:D,num_chars:i64)->String{
        let str_value = str(value);
        right_str(str_value,num_chars)
    }

    #[rhai_fn(name = "RIGHT")]
    pub fn right_str(value:String,num_chars:i64)->String{
        let len = value.len();
        (&value[(len - num_chars as usize)..len]).to_string()
    }

    ///
    /// 根据给定的 X 轴及 Y 轴坐标值，返回反正切值。返回值在 -Pi 到 Pi 之间(不包括 -Pi)
    /// 语法
    /// ATAN2(x_num,y_num)
    /// ▪	X_num: 某点的 X 轴坐标值
    /// ▪	Y_num: 某点的 Y 轴坐标值
    ///
    #[rhai_fn(name = "ATAN2")]
    pub fn atan2(x:D,y:D)->f64{
        f64(x).atan2(f64(y))
    }

    #[rhai_fn(name = "ATAN2")]
    pub fn atan2_f64_d(x:f64,y:D)->f64{
        x.atan2(f64(y))
    }

    #[rhai_fn(name = "ATAN2")]
    pub fn atan2_d_f64(x:D,y:f64)->f64{
        f64(x).atan2(y)
    }

    #[rhai_fn(name = "ATAN2")]
    pub fn atan2_f64_f64(x:f64,y:f64)->f64{
        x.atan2(y)
    }

    ///
    /// 获取数值
    ///
    pub fn f64(d:D)->f64{
        to_f64(&d)
    }

    ///
    ///
    ///
    pub fn str(d:D)->String{
        d.as_string().unwrap_or("".to_string())
    }

}
///
///
///
pub(crate) fn find_cell_text(g:&grid::Grid<Cell>, row_index:u32, col_index:u32)->DataType{
    let opt_cell = g.get(row_index,col_index);
    match opt_cell {
        None => DataType::Empty,
        Some(cell) => {
            match &cell.text {
                None => DataType::Empty,
                Some(text) => text.clone()
            }
        }
    }
}
///
///
///
pub(crate) fn to_f64(d:&DataType)->f64{
    match d {
        DataType::Int(x) => *x as f64,
        DataType::Float(x) => *x,
        _ => 0.
    }
}

///
///
///
pub(crate) fn area_value_list(g:&grid::Grid<Cell>,area:&Area)->Vec<f64>{
    (area.start_row..area.end_row+1).map(|row_index|{
        (area.start_col..area.end_col+1).map(|col_index|{
            let opt_cell = g.get(row_index,col_index);
            match opt_cell {
                None=>None,
                Some(cell)=>match &cell.text {
                    None=>None,
                    Some(data)=>match data {
                        DataType::Int(x) => Some(*x as f64),
                        DataType::Float(x) => Some(*x),
                        _=>None,
                    }
                }
            }
        }).filter(|s|s.is_some()).collect::<Vec<Option<f64>>>()
    }).flatten().map(|s|s.unwrap()).collect::<Vec<f64>>()
}