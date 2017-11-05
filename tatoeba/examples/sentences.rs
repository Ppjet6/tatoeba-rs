//
// Tatoeba - examples/sentences.rs
// Copyright (C) 2017 Maxime “pep” Buquet <pep@bouah.net>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

extern crate diesel;
extern crate tatoeba;

use diesel::prelude::*;

use tatoeba::establish_connection;
use tatoeba::models::Sentence;
use tatoeba::schema::sentences::dsl::*;

fn main() {
    let connection = establish_connection();

    let results = sentences.filter(lang.eq("fra"))
        .order(modified.desc())
        .limit(20)
        .load::<Sentence>(&connection)
        .unwrap();

    for sentence in results {
        println!("------");
        println!("{:?}", sentence);
        println!("------");
        println!("id: {}", sentence.id);
        println!("lang: {:?}", sentence.lang);
        println!("text: {}", String::from_utf8(sentence.text).unwrap());
        println!("------\n");
    }
}
