#![cfg_attr(target_os = "macos", allow(unexpected_cfgs))]
// The `objc` macros reference a `cargo-clippy` cfg internally, which trips the
// new `unexpected_cfgs` lint on stable. Allow it on macOS builds to keep the
// output clean without touching the macro crate.

#[cfg(target_os = "macos")]
use objc::{class, msg_send, sel, sel_impl};
#[cfg(target_os = "macos")]
use cocoa::base::{id, nil};
#[cfg(target_os = "macos")]
use std::os::raw::c_char;
#[cfg(target_os = "macos")]
use std::sync::mpsc::channel;
#[cfg(target_os = "macos")]
use std::time::Duration;

#[cfg(target_os = "macos")]
pub fn get_current_calendar_event() -> Result<Option<String>, String> {
    unsafe {
        // Get the shared event store
        let event_store_class = class!(EKEventStore);
        let event_store: id = msg_send![event_store_class, new];

        if event_store == nil {
            return Err("Failed to create EKEventStore".to_string());
        }

        // Check authorization status
        let auth_status: i64 = msg_send![event_store_class, authorizationStatusForEntityType: 0];

        println!("Calendar authorization status: {}", auth_status);

        // Status values: 0 = not determined, 1 = restricted, 2 = denied, 3 = authorized
        if auth_status == 0 {
            // Not determined - need to request access
            println!("Requesting calendar access...");
            let (tx, rx) = channel();

            let handler = block::ConcreteBlock::new(move |granted: bool| {
                let _ = tx.send(granted);
            });
            let handler = handler.copy();

            let _: () = msg_send![event_store, requestAccessToEntityType:0 completion:handler];

            // Wait for response (with timeout)
            match rx.recv_timeout(Duration::from_secs(30)) {
                Ok(granted) => {
                    if !granted {
                        return Err("Calendar access denied. Please enable in System Settings > Privacy & Security > Calendars".to_string());
                    }
                    println!("Calendar access granted!");
                }
                Err(_) => {
                    return Err("Calendar permission request timed out".to_string());
                }
            }
        } else if auth_status == 1 || auth_status == 2 {
            // Restricted or denied
            return Err("Calendar access denied. Please enable in System Settings > Privacy & Security > Calendars".to_string());
        }
        // auth_status == 3 means already authorized, continue

        // Get current date and time
        let now: id = msg_send![class!(NSDate), date];

        // Create a time interval for searching (1 hour before and after current time)
        let time_interval: f64 = 3600.0; // 1 hour in seconds
        let start_date: id = msg_send![now, dateByAddingTimeInterval: -time_interval];
        let end_date: id = msg_send![now, dateByAddingTimeInterval: time_interval];

        // Get all calendars
        let calendars: id = msg_send![event_store, calendarsForEntityType: 0];
        let calendar_count: usize = msg_send![calendars, count];
        println!("Found {} calendars", calendar_count);

        // Create predicate for events in the time window
        let predicate: id = msg_send![event_store,
            predicateForEventsWithStartDate: start_date
            endDate: end_date
            calendars: calendars
        ];

        // Fetch events matching predicate
        let events: id = msg_send![event_store, eventsMatchingPredicate: predicate];

        // Get event count
        let count: usize = msg_send![events, count];
        println!("Found {} events in time window", count);

        if count == 0 {
            return Ok(None);
        }

        // Find an event that's happening RIGHT NOW
        for i in 0..count {
            let event: id = msg_send![events, objectAtIndex: i];
            let event_start: id = msg_send![event, startDate];
            let event_end: id = msg_send![event, endDate];
            let title: id = msg_send![event, title];

            if title != nil {
                let utf8_ptr: *const c_char = msg_send![title, UTF8String];
                if !utf8_ptr.is_null() {
                    let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
                    let event_title = c_str.to_string_lossy().into_owned();
                    println!("Event {}: {}", i, event_title);
                }
            }

            // Check if current time is between start and end
            let start_comparison: i64 = msg_send![event_start, compare: now];
            let starts_before_now: bool = start_comparison != 1; // NSOrderedDescending = 1
            let end_comparison: i64 = msg_send![event_end, compare: now];
            let ends_after_now: bool = end_comparison != -1; // NSOrderedAscending = -1

            println!("  Start comparison: {}, Ends after: {}", start_comparison, ends_after_now);

            if starts_before_now && ends_after_now {
                // This event is happening now!
                println!("  -> This event is happening NOW!");
                let title: id = msg_send![event, title];

                if title == nil {
                    continue;
                }

                // Convert NSString to Rust String
                let utf8_ptr: *const c_char = msg_send![title, UTF8String];
                if utf8_ptr.is_null() {
                    continue;
                }

                let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
                let rust_string = c_str.to_string_lossy().into_owned();
                return Ok(Some(rust_string));
            }
        }

        // No event currently happening
        Ok(None)
    }
}

#[cfg(target_os = "macos")]
pub fn request_calendar_access() -> Result<String, String> {
    // This function is no longer needed - just call get_current_calendar_event directly
    // which will trigger the permission dialog automatically
    Ok("Call get_current_event to trigger permission dialog".to_string())
}

#[cfg(not(target_os = "macos"))]
pub fn request_calendar_access() -> Result<String, String> {
    Err("Calendar integration is only supported on macOS".to_string())
}

#[cfg(not(target_os = "macos"))]
pub fn get_current_calendar_event() -> Result<Option<String>, String> {
    Err("Calendar integration is only supported on macOS".to_string())
}
