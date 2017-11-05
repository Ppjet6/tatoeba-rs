//
// Tatoeba - src/schema.rs
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

table! {
    sentences (id) {
        id -> Integer,
        lang -> Nullable<Varchar>,
        text -> Varbinary,
        correctness -> Tinyint,
        user_id -> Nullable<Integer>,
        created -> Nullable<Datetime>,
        modified -> Nullable<Datetime>,
        script -> Nullable<Varchar>,
        hash -> Binary,
    }
}

table! {
    sentences_translations (id) {
        id -> Integer,
        sentence_id -> Integer,
        translation_id -> Integer,
        sentence_lang -> Nullable<Varchar>,
        translation_lang -> Nullable<Varchar>,
        distance -> Smallint,
    }
}
