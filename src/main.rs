//
// Tatoeba - main.rs
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

#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(@app (app_from_crate!())
        (@arg db_host: -h --host +takes_value +required "Database hostname")
        (@arg db_user: -u --user +takes_value +required "Database username")
        (@arg db_passwd: -p --password +takes_value +required "Database password")
    ).get_matches();

    println!("{:?}", matches);
}
