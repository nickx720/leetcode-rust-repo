impl Solution {
    pub fn reformat_date(date: String) -> String {
        let date = date.trim().split(" ").collect::<Vec<&str>>();
        let (day,month,year) = (date[0].replace("th", "").replace("nd", "").replace("st", "").replace("rd",""),date[1],date[2]);
        let month = match month {
            "Jan" => "01",
            "Feb" => "02",
            "Mar" => "03",
            "Apr" => "04",
            "May" => "05",
            "Jun" => "06",
            "Jul" => "07",
            "Aug" => "08",
            "Sep" => "09",
            "Oct" => "10",
            "Nov" => "11",
            "Dec" => "12",
            _=> unreachable!(),
        };
        let day = if day.len() < 2 {
            "0".to_string() + &day
        } else {
            day
        };

        format!("{}-{}-{}",year,month,day)

    }
}
