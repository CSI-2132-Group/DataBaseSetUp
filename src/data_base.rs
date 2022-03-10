use mysql::*;
use mysql::prelude::*;

#[derive(Debug)]
pub struct DataBase {
    pub db_host : String,
    pub db_port : String,
    pub db_name : String,
    pub db_username : String,
    pub db_password : String,
}

impl DataBase {
    fn get_conn(&self) -> mysql::PooledConn {
        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            self.db_username,
            self.db_password,
            self.db_host,
            self.db_port,
            self.db_name
        );

        println!("{}", url);

        let url : Opts = Opts::from_url(&url).unwrap();

        let pool = Pool::new(url).unwrap();

        pool.get_conn().unwrap()
    }

    pub fn hard_reset(&self) {
        let mut conn = self.get_conn();

        let stmp = conn.prep(r"SHOW TABLES;").unwrap();

        let result = conn.exec_iter(stmp, ())
        .unwrap()
        .map(
            |row_tmp| {
                let row : mysql::Row = row_tmp.unwrap();
                let name : String = row.get(0).unwrap();

                let mut conn = self.get_conn();

                
                let stmp = conn.prep(r"SET FOREIGN_KEY_CHECKS=0").unwrap();
                let result = conn.exec_iter(stmp, ())
                .unwrap();
                for r in result {

                }


                let stmt = format!(r"DROP TABLE {}", name);

                let tmp = conn.prep(stmt).unwrap();

                let result = conn.exec_iter(tmp, ())
                .unwrap();

                for r in result {

                }
            }
        );

        for r in result {

        }

        

        let stmp = conn.prep(r"SET FOREIGN_KEY_CHECKS=1").unwrap();
        let result = conn.exec_iter(stmp, ())
        .unwrap();
        for r in result {

        }
    }

    pub fn execute(&self, command : &str){
        let mut conn = self.get_conn();

        let tmp = conn.prep(command).unwrap();

        let result = conn.exec_iter(tmp, ())
        .unwrap();

        for r in result {

        }
    }
}