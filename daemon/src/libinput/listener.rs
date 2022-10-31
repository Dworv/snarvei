use std::fs::{File, OpenOptions};
use std::os::unix::{fs::OpenOptionsExt, io::{RawFd, FromRawFd, IntoRawFd}};
use std::path::Path;
use std::sync::mpsc::Sender;

use input::{Libinput, LibinputInterface};
use libc::{O_RDONLY, O_RDWR, O_WRONLY};

use super::Event;

struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<RawFd, i32> {
        println!("{flags}");
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into_raw_fd())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: RawFd) {
        unsafe {
            File::from_raw_fd(fd);
        }
    }
}

pub fn listen(tx: Sender<Event>) {
    let mut input = Libinput::new_with_udev(Interface);
    input.udev_assign_seat("seat0").unwrap();
    loop {
        input.dispatch().unwrap();
        for e in &mut input {
            // Convert input.rs event to our event
            println!("hi");
            if let Ok(ok_e) = Event::try_from(e) {
                // If tx sending the event fails return
                if let Err(_) = tx.send(ok_e) {
                    return
                }
            }
        }
    }
}