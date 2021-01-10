pub enum StreamError {
    Undefined,
    UnknownState,
    Schedule,
}

pub enum StreamSender<R> {
    Ok,
    Error(StreamError),
    Value(R),
    Partial {
        index: isize,
        length: isize,
        value: R,
    },
    Waiting {
        length: isize,
    },
    Request,
    Done,
}

pub enum StreamReceiver {
    Ok,
    Error(StreamError),
    Close,
    Send { index: isize, length: isize },
    Create,
    Pause,
    Resume,
    Request,
}

pub trait StreamInstance<T> {
    fn wake_client(&self);
    fn get_handle(&self) -> i64;
}
