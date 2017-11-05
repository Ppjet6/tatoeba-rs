//
// Tatoeba - src/models.rs
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

use chrono::NaiveDateTime;

#[derive(Queryable, Debug)]
pub struct Sentence {
    pub id: i32,
    pub lang: Option<String>,
    pub text: Vec<u8>,
    pub correctness: i8,
    pub user_id: Option<i32>,
    pub created: Option<NaiveDateTime>,
    pub modified: Option<NaiveDateTime>,
    pub dico_id: Option<i32>,
    pub script: Option<String>,
    pub hash: Vec<u8>,
}

#[derive(Queryable)]
pub struct Translation {
    pub id: i32,
    pub sentence_id: i32,
    pub translation_id: i32,
    pub sentence_lang: Option<String>,
    pub translation_lang: Option<String>,
    pub distance: i16,
}
