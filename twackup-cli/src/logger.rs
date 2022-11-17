/*
 * Copyright 2020 DanP
 *
 * This file is part of Twackup
 *
 * Twackup is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Twackup is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Twackup. If not, see <http://www.gnu.org/licenses/>.
 */

use super::progress_bar::PROGRESS_BAR;
use log::{LevelFilter, Log, Metadata, Record};
use stderrlog::LogLevelNum;

pub(crate) struct Logger(stderrlog::StdErrLog);

impl Logger {
    pub(crate) fn init() {
        let mut logger = stderrlog::new();
        logger.show_level(true);
        logger.verbosity(LogLevelNum::Info);

        let logger = Self(logger);

        log::set_max_level(LevelFilter::Info);
        log::set_boxed_logger(Box::new(logger)).expect("Logger failed");
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        self.0.enabled(metadata)
    }

    fn log(&self, record: &Record<'_>) {
        match unsafe { PROGRESS_BAR } {
            Some(progress_bar) => progress_bar.0.suspend(|| self.0.log(record)),
            None => self.0.log(record),
        }
    }

    fn flush(&self) {
        self.0.flush();
    }
}
