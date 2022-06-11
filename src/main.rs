// Copyright (C) 2022 baris-inandi
//
// This file is part of osrepo.
//
// osrepo is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// osrepo is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with osrepo.  If not, see <http://www.gnu.org/licenses/>.

mod db;
pub mod utils;
use db::Db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut db = Db::new("osrepo.yml").unwrap();
    let entry = db.entry("windows")?;
    let version = entry.version("11")?;
    println!("\n{}", entry);
    println!("\n{}", version);
    version.download(&db.client).await?;
    // db.update().await?;
    Ok(())
}
