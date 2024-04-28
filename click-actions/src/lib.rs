use x11::xlib::{
    ClientMessage, ClientMessageData, Display, False, SubstructureNotifyMask, SubstructureRedirectMask, True, XClientMessageEvent, XCloseDisplay, XDefaultRootWindow, XDefaultScreen, XEvent, XIconifyWindow, XInternAtom, XMapRaised, XOpenDisplay, XSendEvent
};

pub struct XorgConnection {
    connection: *mut Display,
}
impl XorgConnection {
    pub fn new() -> Self {
        let connection = unsafe { XOpenDisplay(std::ptr::null()) };
        //TODO better be something than NULL
        Self { connection }
    }
    pub fn get_reference(&self) -> &mut Display {
        unsafe { &mut *self.connection }
    }
    pub fn intern_atom(&self, msg: &str) -> u64 {
        unsafe { XInternAtom(self.connection, msg.as_ptr() as *const i8, False) }
    }
    pub fn send_event(&self, mask: i64, event: &mut XEvent) {
        let sent = unsafe {
            XSendEvent(
                self.connection,
                XDefaultRootWindow(self.connection),
                0,
                mask,
                event,
            )
        };
    }
    fn create_client_message_event(&self, client_id: u64, msg: &str) -> XClientMessageEvent {
        XClientMessageEvent {
            type_: ClientMessage,
            serial: 0,
            send_event: True,
            display: self.connection,
            window: client_id,
            message_type: self.intern_atom(msg),
            format: 32,
            data: ClientMessageData::new(),
        }
    }
    pub fn minimize(&self, client_id: u64) {
        unsafe {
            XIconifyWindow(self.connection, client_id, XDefaultScreen(self.connection));
        }
    }
    pub fn close(&self, client_id: u64) {
        let client_msg = self.create_client_message_event(client_id, "_NET_CLOSE_WINDOW");
        let mask = SubstructureRedirectMask | SubstructureNotifyMask;

        let mut event = XEvent {
            client_message: client_msg,
        };

        self.send_event(mask, &mut event);
    }
    pub fn raise(&self, client_id: u64) {
        unsafe {
            XMapRaised(self.connection, client_id);
        }
    }
    pub fn set_active(&self, client_id: u64) {
        let client_msg = self.create_client_message_event(client_id, "_NET_ACTIVE_WINDOW");
        let mask = SubstructureRedirectMask | SubstructureNotifyMask;

        let mut event = XEvent {
            client_message: client_msg,
        };

        self.send_event(mask, &mut event);
    }
}

impl Drop for XorgConnection {
    fn drop(&mut self) {
        unsafe {
            XCloseDisplay(self.connection);
        }
    }
}
