/*
    Copyright (C) 2019  John Goerzen <jgoerzen@complete.org

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use std::io;
use std::io::{BufRead};
use crate::lorastik::{LoraStik};
pub use crate::pipe::loratostdout;

// Spec: http://www.ax25.net/kiss.aspx

const FEND: u8 = 0xC0;
// const FESC: u8 = 0xDB;
// const TFEND: u8 = 0xDC;
// const TFESC: u8 = 0xDD;

/// A thread for stdin processing
pub fn stdintolorakiss(ls: &mut LoraStik, maxframesize: usize) -> io::Result<()> {
    let stdin = io::stdin();
    let mut br = io::BufReader::new(stdin);

    let mut buf = vec![0u8; 8192];

    loop {
        let res = br.read_until(FEND, &mut buf)?;
        if res == 0 {
            // EOF
            return Ok(());
        } else if res < 2 {
            // Every frame from stdin will start with FEND and a control character;
            // we got just FEND, we are in the space between to frames, so we should just
            // proceed.  Similar if we have some non-data frame.
            continue;
        } else if buf[0] != 0 {
            // A TNC control frame; do not send.
            continue;
        }
        // OK, we've got it, now make sure it doesn't exceed the limit and transmit.
        for chunk in buf[0..res].chunks(maxframesize) {
            ls.transmit(&chunk);
        }
    }
}

// loratostdout just comes from pipe
