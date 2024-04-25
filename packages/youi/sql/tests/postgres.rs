use youi_sql::read_sql;

#[test]
pub fn test_query(){
    let conn = "postgresql://postgres:3541154@192.168.173.1:5432/stats";
    let df = read_sql(conn,"select * from stats.stats_working_dimension","binary");
    let data = df.unwrap();
    println!("test postgres query {:?}",data.get_columns());
}