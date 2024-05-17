<!--

    Licensed to the Apache Software Foundation (ASF) under one
    or more contributor license agreements.  See the NOTICE file
    distributed with this work for additional information
    regarding copyright ownership.  The ASF licenses this file
    to you under the Apache License, Version 2.0 (the
    "License"); you may not use this file except in compliance
    with the License.  You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing,
    software distributed under the License is distributed on an
    "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
    KIND, either express or implied.  See the License for the
    specific language governing permissions and limitations
    under the License.

-->

# Apache IoTDB

[![Main Mac and Linux](https://github.com/apache/iotdb/actions/workflows/main-unix.yml/badge.svg)](https://github.com/apache/iotdb/actions/workflows/main-unix.yml)
[![Main Win](https://github.com/apache/iotdb/actions/workflows/main-win.yml/badge.svg)](https://github.com/apache/iotdb/actions/workflows/main-win.yml)
[![coveralls](https://coveralls.io/repos/github/apache/iotdb/badge.svg?branch=master)](https://coveralls.io/repos/github/apache/iotdb/badge.svg?branch=master)
[![GitHub release](https://img.shields.io/github/release/apache/iotdb.svg)](https://github.com/apache/iotdb/releases)
[![License](https://img.shields.io/badge/license-Apache%202-4EB1BA.svg)](https://www.apache.org/licenses/LICENSE-2.0.html)
![](https://github-size-badge.herokuapp.com/apache/iotdb.svg)
![](https://img.shields.io/github/downloads/apache/iotdb/total.svg)
![](https://img.shields.io/badge/platform-win10%20%7C%20macox%20%7C%20linux-yellow.svg)
![](https://img.shields.io/badge/java--language-1.8-blue.svg)
[![IoTDB Website](https://img.shields.io/website-up-down-green-red/https/shields.io.svg?label=iotdb-website)](https://iotdb.apache.org/)


Apache IoTDB (Database for Internet of Things) is an IoT native database with high performance for
data management and analysis, deployable on the edge and the cloud. Due to its light-weight
architecture, high performance and rich feature set together with its deep integration with
Apache Hadoop, Spark and Flink, Apache IoTDB can meet the requirements of massive data storage,
high-speed data ingestion and complex data analysis in the IoT industrial fields.

# Apache IoTDB Client for Rust

## About version support
Supports the latest version 1.3.1 of IoTDB and is also compatible with operations below version 1.0

## Overview

This is the Rust client of Apache IoTDB.

Apache IoTDB website: https://iotdb.apache.org
Apache IoTDB Github: https://github.com/apache/iotdb

## Prerequisites

apache-iotdb 0.12.0 and newer.</br>
rust 1.56.0 and newer.

## How to Use the Client (Quick Start)

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
iotdb-client="0.1.0"
```

## Example

Put this in your example's `Cargo.toml`:

```toml
[dependencies]
iotdb-client="0.1.0"
chrono="0.4.19"
prettytable-rs="0.8.0"
structopt = "0.3.25"
```

```rust
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

```
