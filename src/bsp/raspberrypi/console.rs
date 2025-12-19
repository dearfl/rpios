use crate::{
    console,
    synchronization::{NullLock, interface::Mutex},
};
use core::fmt::{self, Write};

struct QEMUOutputInner {
    chars_written: usize,
}

impl QEMUOutputInner {
    const fn new() -> Self {
        Self { chars_written: 0 }
    }

    fn write_char(&mut self, c: char) {
        // not working on raspi4b right now, IDK why
        // SAFETY: ???
        unsafe {
            core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
        }

        self.chars_written += 1;
    }
}

impl fmt::Write for QEMUOutputInner {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            if c == '\n' {
                self.write_char('\r');
            }
            self.write_char(c);
        }

        Ok(())
    }
}

struct QEMUOutput {
    inner: NullLock<QEMUOutputInner>,
}

impl QEMUOutput {
    pub const fn new() -> Self {
        Self {
            inner: NullLock::new(QEMUOutputInner::new()),
        }
    }
}

impl console::interface::Write for QEMUOutput {
    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result {
        self.inner.lock(|inner| inner.write_fmt(args))
    }
}

impl console::interface::Statistics for QEMUOutput {
    fn chars_written(&self) -> usize {
        self.inner.lock(|inner| inner.chars_written)
    }
}

impl console::interface::All for QEMUOutput {}

static QEMU_OUTPUT: QEMUOutput = QEMUOutput::new();

pub fn console() -> &'static dyn console::interface::All {
    &QEMU_OUTPUT
}
