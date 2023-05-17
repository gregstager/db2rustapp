/**********************************************************************
** (c) Copyright IBM Corp. 2007 All rights reserved.
** 
** The following sample of source code ("Sample") is owned by International 
** Business Machines Corporation or one of its subsidiaries ("IBM") and is 
** copyrighted and licensed, not sold. You may use, copy, modify, and 
** distribute the Sample in any form without payment to IBM, for the purpose of 
** assisting you in the development of your applications.
** 
** The Sample code is provided to you on an "AS IS" basis, without warranty of 
** any kind. IBM HEREBY EXPRESSLY DISCLAIMS ALL WARRANTIES, EITHER EXPRESS OR 
** IMPLIED, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF 
** MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. Some jurisdictions do 
** not allow for the exclusion or limitation of implied warranties, so the above 
** limitations or exclusions may not apply to you. IBM shall not be liable for 
** any damages you suffer as a result of using, copying, modifying or 
** distributing the Sample, even if IBM has been advised of the possibility of 
** such damages.
***********************************************************************/

// Much of this code was originally taken from the sample
// https://github.com/ibmdb/rust-ibm_db/blob/main/examples/connect.rs
// Which has the following license:

// MIT License

// Copyright (c) 2021 ibmdb

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.


use std::io;
use ibm_db::{safe::AutocommitOn,Statement,Environment,Version3, create_environment_v3, Connection,ResultSetState::{NoData, Data}};
use std::error::Error;
use std::io::Write;

fn main() {
    let mut dbname = String::new();
    print!("Enter database name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dbname).unwrap();

    let mut userid = String::new();
    print!("Enter username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut userid).unwrap();

    let password = rpassword::prompt_password("Enter password: ").unwrap();

    let env = create_environment_v3().unwrap();

    let conn = match connect(&env, &dbname.trim(), &userid.trim(), &password ) {
        Ok(s) => {
            println!("Connection Successful.");
            s
        }
        Err(diag) => {
            println!("Error Connecting: {}", diag);
            return;
        }
    };

    match execute_statement(&conn){
        Ok(_a) => {},
        Err(diag) => {
            println!("Error of some sort: {}", diag);
            return;
        }
    }

    match execute_statement(&conn){
        Ok(_a) => {},
        Err(diag) => {
            println!("Error of another sort: {}", diag);
            return;
        }
    }
}

fn connect<'env>(env:  &'env Environment<Version3>  , dbname : &str, userid : &str, password : &str )
      -> Result<Connection<'env, AutocommitOn>, Box<(dyn std::error::Error +'static)>> {

    Ok(env.connect( dbname, userid, password)?)
}

fn execute_statement<'env>(conn: &Connection<'env, AutocommitOn>) -> Result<(),Box<dyn Error>> {
    let stmt = Statement::with_parent(conn)?;

    let mut sql_text = String::new();
    println!("Please enter SQL statement string: ");
    io::stdin().read_line(&mut sql_text).unwrap();

    match stmt.exec_direct(&sql_text)? {
        Data(mut stmt) => {
            let cols = stmt.num_result_cols()?;
            while let Some(mut cursor) = stmt.fetch()? {
                for i in 1..(cols + 1) {
                    match cursor.get_data::<&str>(i as u16)? {
                        Some(val) => print!(" {}", val),
                        None => print!(" NULL"),
                    }
                }
                println!();
            }
        }
        NoData(_) => println!("Query executed, no data returned"),
    }

    Ok(())
}
