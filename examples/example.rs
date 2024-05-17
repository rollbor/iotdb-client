//
// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//  http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.
//

use std::{collections::BTreeMap, vec};

use chrono::Local;
use iotdb_client::client::remote::{Config, RpcSession};
use iotdb_client::client::{MeasurementSchema, Result, RowRecord, Session, Tablet, Value};
use iotdb_client::protocal::{TSCompressionType, TSDataType, TSEncoding};
use prettytable::{cell, Row, Table};
use structopt::StructOpt;

fn main() {
    demo().expect("failed to run example.");
}

fn demo() -> Result<()> {

    let config = Config::builder()
        .host("127.0.0.1")
        .port(6667)
        .username("root")
        .password("root")
        .build();

    let mut session = RpcSession::new(config)?;
    session.open()?;

    //time_zone
    let tz = session.get_time_zone()?;
    if tz != "Asia/Shanghai" {
        session.set_time_zone("Asia/Shanghai")?;
    }

    //execute_query_statement
    {
        let dataset = session.execute_query_statement(" select * from root.sg_rs.*;", None)?;
        // Get columns, column types and values from the dataset
        // For example:
        let width = 18;
        let column_count = dataset.get_column_names().len();
        let print_line_sep =
            || println!("{:=<width$}", '=', width = (width + 1) * column_count + 1);

        print_line_sep();
        dataset
            .get_column_names()
            .iter()
            .for_each(|c| print!("|{:>width$}", c.split('.').last().unwrap(), width = width));
        println!("|");
        print_line_sep();
        dataset.get_data_types().iter().for_each(|t| {
            let type_name = format!("{:?}", t);
            print!("|{:>width$}", type_name, width = width)
        });
        println!("|");
        print_line_sep();
        dataset.for_each(|r| {
            r.values.iter().for_each(|v| match v {
                Value::Bool(v) => print!("|{:>width$}", v, width = width),
                Value::Int32(v) => print!("|{:>width$}", v, width = width),
                Value::Int64(v) => print!("|{:>width$}", v, width = width),
                Value::Float(v) => print!("|{:>width$}", v, width = width),
                Value::Double(v) => print!("|{:>width$}", v, width = width),
                Value::Text(v) => print!("|{:>width$}", v, width = width),
                Value::Null => print!("|{:>width$}", "null", width = width),
            });
            println!("|");
        });
        print_line_sep();
    }
    session.close()?;
    Ok(())
}



