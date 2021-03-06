use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

mod data_base;
mod env_var_intialization;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let a = env_var_intialization::start_up(&args);
    
    a.hard_reset();

    if !file_check() {
        file_set_up_creator();
    }

    let mut file = File::open("setup.txt").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents);

    let lines = contents.split("\n");
    
    for line in lines{
        if line != "" {
            a.execute(&line)
        }

    }
}

fn file_check() -> bool {
    let path = Path::new("setup.txt");

    path.exists()
}

fn file_set_up_creator() -> std::io::Result<()> {
    let mut file = File::create("setup.txt")?;
    file.write_all(
"CREATE TABLE Branch (Branch_ID INT, House_number MEDIUMINT, Street VARCHAR(32), City VARCHAR(32), Province CHAR(2), Name VARCHAR(64), PRIMARY KEY(Branch_ID))

CREATE TABLE Employee (Employee_ID INT, Branch_ID INT, First_name VARCHAR(32), Middle_name VARCHAR(32), Last_name VARCHAR(32), House_number MEDIUMINT, Street VARCHAR(32), City VARCHAR(32), Province CHAR(2), Gender Binary(1), Email_address VARCHAR(32), Date_of_birth DATE, Phone_number VARCHAR(16), SSN INT, Employee_type INT, Salary MEDIUMINT, PRIMARY KEY(Employee_ID), Foreign KEY(Branch_ID) REFERENCES Branch(Branch_ID))

CREATE TABLE User (Username VARCHAR(32), Employee_ID INT, Password BINARY(32), Insurance INT, PRIMARY KEY(Username), Foreign KEY(Employee_ID) REFERENCES Employee(Employee_ID))

CREATE TABLE Patient (Patient_ID INT, Username VARCHAR(32), First_name VARCHAR(32), Middle_name VARCHAR(32), Last_name VARCHAR(32), House_number MEDIUMINT, Street VARCHAR(32), City VARCHAR(32), Province CHAR(2), Gender Binary(1), Email_address VARCHAR(32), Date_of_birth DATE, Phone_number VARCHAR(16), SSN INT, PRIMARY KEY(Patient_ID), Foreign KEY(Username) REFERENCES User(Username))

CREATE TABLE Invoice (Payee INT, Appointment INT, Date_of_issue DATE, Insurance_charge INT, Patient_charge INT, Discount INT, Penalty INT)

CREATE TABLE Treatment (Treatment_ID INT, Type INT, Tooth INT, Medication VARCHAR(32), Comments VARCHAR(256), Penalty INT, PRIMARY KEY(Treatment_ID))

CREATE TABLE Procedures (Procedure_ID INT, Procedure_type INT, Fee INT, PRIMARY KEY(Procedure_ID))

CREATE TABLE Treatment_Instructions (Treatment_ID INT, Procedure_ID INT, Foreign KEY(Treatment_ID) REFERENCES Treatment(Treatment_ID), Foreign KEY(Procedure_ID) REFERENCES Procedures(Procedure_ID))

CREATE TABLE Appointment (Appointment_ID INT, Patient_ID INT, Employee_ID INT, Treatment_ID INT, Start_time DATE, End_time DATE, Status INT, Appointment_type INT, PRIMARY KEY(Appointment_ID), Foreign KEY(Patient_ID) REFERENCES Patient(Patient_ID), Foreign KEY(Employee_ID) REFERENCES Employee(Employee_ID),Foreign KEY(Treatment_ID) REFERENCES Treatment(Treatment_ID)) 

CREATE TABLE Review (Review_ID INT, Appointment_ID INT, Professionalism INT, Communication INT, Cleanliness INT, Score INT, PRIMARY KEY(Review_ID), Foreign KEY(Appointment_ID) REFERENCES Appointment(Appointment_ID))

CREATE TABLE Branch_location (Appointment_ID INT, Branch_ID INT, Assigned_room INT, Foreign KEY(Appointment_ID) REFERENCES Appointment(Appointment_ID), Foreign KEY(Branch_ID) REFERENCES Branch(Branch_ID))
".as_bytes()
    )?;
    Ok(())
}