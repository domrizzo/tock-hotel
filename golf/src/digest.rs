use core::cell::Cell;
use hotel::hil::digest::{Client, DigestEngine, DigestMode, SyscallError};
use kernel::{AppId, AppSlice, Callback, Container, Driver, Shared};

/// Per-application driver data.
pub struct AppData {
    /// Buffer where data to be hashed will be read from.
    input_buffer: Option<AppSlice<Shared, u8>>,
    /// Buffer where the digest will be written to when hashing is finished.
    output_buffer: Option<AppSlice<Shared, u8>>,
    callback: Option<Callback>,
}

impl Default for AppData {
    fn default() -> AppData {
        AppData {
            input_buffer: None,
            output_buffer: None,
            callback: None,
        }
    }
}

pub struct DigestDriver<'a, E: DigestEngine + 'a> {
    engine: &'a E,
    apps: Container<AppData>,
    current_user: Cell<Option<AppId>>,
}

impl<'a, E: DigestEngine + 'a> DigestDriver<'a, E> {
    pub fn new(engine: &'a E, container: Container<AppData>) -> DigestDriver<'a, E> {
        DigestDriver {
            engine: engine,
            apps: container,
            current_user: Cell::new(None),
        }
    }
}

impl<'a, E: DigestEngine> Client for DigestDriver<'a, E> {
    fn done(&self, digest: &[u32]) {
        self.current_user.get().map(|current_user| {
            let _ = self.apps.enter(current_user, |app_data, _| {
                app_data.output_buffer.as_mut().map(|mut output_buffer| {
                    for (out, digest) in output_buffer.as_mut().chunks_mut(4).zip(digest) {
                        let mut offset = 0;
                        for o in out.iter_mut() {
                            *o = (digest >> offset) as u8;
                            offset += 8;
                        }
                    }
                });

                app_data.callback.map(|mut cb| {
                    cb.schedule(0, 0, 0);
                });
            });
        });
    }
}

impl<'a, E: DigestEngine> Driver for DigestDriver<'a, E> {
    fn subscribe(&self, subscribe_num: usize, callback: Callback) -> isize {
        match subscribe_num {
            0 => {
                let _ = self.apps.enter(callback.app_id(), |app_data, _| {
                    app_data.callback = Some(callback);
                });
                0
            }
            _ => -1,
        }
    }

    fn command(&self, minor_num: usize, r2: usize, caller_id: AppId) -> isize {
        match minor_num {
                // Initialize hash engine (arg: digest mode)
                0 => {
                    self.apps
                        .enter(caller_id, |_app_data, _| {
                            if self.current_user.get().is_some() {
                                return Err(SyscallError::ResourceBusy);
                            }
                            self.current_user.set(Some(caller_id));

                            let digest_mode = match r2 {
                                0 => DigestMode::Sha1,
                                1 => DigestMode::Sha256,
                                _ => return Err(SyscallError::InvalidArgument),
                            };

                            try!(self.engine
                                .initialize(digest_mode)
                                .map_err(|_| SyscallError::InternalError));

                            Ok(0)
                        })
                        .unwrap_or(Err(SyscallError::InternalError))
                }
                // Feed data from input buffer (arg: number of bytes)
                1 => {
                    self.apps
                        .enter(caller_id, |app_data, _| {
                            match self.current_user.get() {
                                Some(cur) if cur.idx() == caller_id.idx() => {}
                                _ => {
                                    return Err(SyscallError::InvalidState);
                                }
                            }

                            let app_data: &mut AppData = app_data;

                            let input_buffer = match app_data.input_buffer {
                                Some(ref slice) => slice,
                                None => return Err(SyscallError::InvalidState),
                            };

                            let input_len = r2;
                            if input_len > input_buffer.len() {
                                return Err(SyscallError::OutOfRange);
                            }

                            try!(self.engine
                                .update(&input_buffer.as_ref()[..input_len])
                                .map_err(|_| SyscallError::InternalError));

                            Ok(0)
                        })
                        .unwrap_or(Err(SyscallError::InternalError))
                }
                // Finalize hash and output to output buffer (arg: unused)
                2 => {
                    match self.current_user.get() {
                        Some(cur) if cur.idx() == caller_id.idx() => {
                            self.engine
                                .finalize()
                                .map(|_| 0)
                                .map_err(|_| SyscallError::InternalError)
                        }
                        _ => Err(SyscallError::InvalidState),
                    }
                }
                _ => Err(SyscallError::NotImplemented),
            }
            .unwrap_or_else(|err| err.into())
    }

    fn allow(&self, app_id: AppId, minor_num: usize, slice: AppSlice<Shared, u8>) -> isize {
        match minor_num {
                0 => {
                    // Input buffer
                    self.apps
                        .enter(app_id, |app_data, _| {
                            app_data.input_buffer = Some(slice);
                            Ok(0)
                        })
                        .unwrap_or(Err(SyscallError::InternalError))
                }
                1 => {
                    // Hash output buffer
                    self.apps
                        .enter(app_id, |app_data, _| {
                            app_data.output_buffer = Some(slice);
                            Ok(0)
                        })
                        .unwrap_or(Err(SyscallError::InternalError))
                }
                _ => Err(SyscallError::NotImplemented),
            }
            .unwrap_or_else(|err| err.into())
    }
}
