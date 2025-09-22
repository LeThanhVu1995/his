use app_web::security::PermissionDef;

pub mod perm {
    // Providers
    pub const PROV_LIST:   &str = "his.appt.provider.list";
    pub const PROV_CREATE: &str = "his.appt.provider.create";
    pub const PROV_UPDATE: &str = "his.appt.provider.update";

    // Rooms
    pub const ROOM_LIST:   &str = "his.appt.room.list";
    pub const ROOM_CREATE: &str = "his.appt.room.create";
    pub const ROOM_UPDATE: &str = "his.appt.room.update";

    // Schedules
    pub const SCHED_LIST:   &str = "his.appt.schedule.list";
    pub const SCHED_CREATE: &str = "his.appt.schedule.create";
    pub const SCHED_UPDATE: &str = "his.appt.schedule.update";

    // Slots
    pub const SLOT_LIST:    &str = "his.appt.slot.list";
    pub const SLOT_GENERATE:&str = "his.appt.slot.generate";

    // Appointments
    pub const APPT_LIST:   &str = "his.appt.appt.list";
    pub const APPT_BOOK:   &str = "his.appt.appt.book";
    pub const APPT_CANCEL: &str = "his.appt.appt.cancel";
    pub const APPT_RESCH:  &str = "his.appt.appt.reschedule";
}

pub fn permission_catalog(svc: &str) -> Vec<PermissionDef> {
    use perm::*;
    vec![
        PermissionDef::new(PROV_LIST, "List providers", "providers", "list"),
        PermissionDef::new(PROV_CREATE, "Create provider", "providers", "create"),
        PermissionDef::new(PROV_UPDATE, "Update provider", "providers", "update"),
        PermissionDef::new(ROOM_LIST, "List rooms", "rooms", "list"),
        PermissionDef::new(ROOM_CREATE, "Create room", "rooms", "create"),
        PermissionDef::new(ROOM_UPDATE, "Update room", "rooms", "update"),
        PermissionDef::new(SCHED_LIST, "List schedules", "schedules", "list"),
        PermissionDef::new(SCHED_CREATE, "Create schedule", "schedules", "create"),
        PermissionDef::new(SCHED_UPDATE, "Update schedule", "schedules", "update"),
        PermissionDef::new(SLOT_LIST, "List slots", "slots", "list"),
        PermissionDef::new(SLOT_GENERATE, "Generate slots", "slots", "generate"),
        PermissionDef::new(APPT_LIST, "List appointments", "appointments", "list"),
        PermissionDef::new(APPT_BOOK, "Book appointment", "appointments", "book"),
        PermissionDef::new(APPT_CANCEL, "Cancel appointment", "appointments", "cancel"),
        PermissionDef::new(APPT_RESCH, "Reschedule appointment", "appointments", "reschedule"),
    ]
}
