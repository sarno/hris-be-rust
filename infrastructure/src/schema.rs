// @generated automatically by Diesel CLI.

diesel::table! {
    attendance_histories (id) {
        id -> Int8,
        user_id -> Int8,
        check_in_latitude -> Nullable<Float8>,
        check_in_longitude -> Nullable<Float8>,
        #[max_length = 255]
        check_in_photo -> Nullable<Varchar>,
        check_in_date -> Timestamp,
        check_out_latitude -> Nullable<Float8>,
        check_out_longitude -> Nullable<Float8>,
        #[max_length = 255]
        check_out_photo -> Nullable<Varchar>,
        check_out_date -> Nullable<Timestamp>,
        #[max_length = 255]
        working_hour -> Nullable<Varchar>,
        #[max_length = 255]
        status -> Varchar,
        company_id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    cache (key) {
        #[max_length = 255]
        key -> Varchar,
        value -> Text,
        expiration -> Int4,
    }
}

diesel::table! {
    cache_locks (key) {
        #[max_length = 255]
        key -> Varchar,
        #[max_length = 255]
        owner -> Varchar,
        expiration -> Int4,
    }
}

diesel::table! {
    companies (id) {
        id -> Int8,
        #[max_length = 255]
        company_code -> Varchar,
        #[max_length = 255]
        company_name -> Varchar,
        #[max_length = 255]
        photo -> Nullable<Varchar>,
        #[max_length = 255]
        address -> Varchar,
        latitude -> Nullable<Float8>,
        longitude -> Nullable<Float8>,
        #[max_length = 255]
        status -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    departments (id) {
        id -> Int8,
        #[max_length = 255]
        department_code -> Varchar,
        #[max_length = 255]
        department_name -> Varchar,
        company_id -> Int8,
        #[max_length = 255]
        status -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    failed_jobs (id) {
        id -> Int8,
        #[max_length = 255]
        uuid -> Varchar,
        connection -> Text,
        queue -> Text,
        payload -> Text,
        exception -> Text,
        failed_at -> Timestamp,
    }
}

diesel::table! {
    job_batches (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        total_jobs -> Int4,
        pending_jobs -> Int4,
        failed_jobs -> Int4,
        failed_job_ids -> Text,
        options -> Nullable<Text>,
        cancelled_at -> Nullable<Int4>,
        created_at -> Int4,
        finished_at -> Nullable<Int4>,
    }
}

diesel::table! {
    jobs (id) {
        id -> Int8,
        #[max_length = 255]
        queue -> Varchar,
        payload -> Text,
        attempts -> Int2,
        reserved_at -> Nullable<Int4>,
        available_at -> Int4,
        created_at -> Int4,
    }
}

diesel::table! {
    migrations (id) {
        id -> Int4,
        #[max_length = 255]
        migration -> Varchar,
        batch -> Int4,
    }
}

diesel::table! {
    password_reset_tokens (email) {
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        token -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    personal_access_tokens (id) {
        id -> Int8,
        #[max_length = 255]
        tokenable_type -> Varchar,
        tokenable_id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 64]
        token -> Varchar,
        abilities -> Nullable<Text>,
        last_used_at -> Nullable<Timestamp>,
        expires_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    projects (id) {
        id -> Int8,
        #[max_length = 255]
        project_code -> Varchar,
        #[max_length = 255]
        project_name -> Varchar,
        company_id -> Int8,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    sessions (id) {
        #[max_length = 255]
        id -> Varchar,
        user_id -> Nullable<Int8>,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
        user_agent -> Nullable<Text>,
        payload -> Text,
        last_activity -> Int4,
    }
}

diesel::table! {
    user_request_histories (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 255]
        request_type -> Varchar,
        request_date -> Timestamp,
        #[max_length = 255]
        description -> Varchar,
        #[max_length = 255]
        attachment -> Nullable<Varchar>,
        #[max_length = 255]
        status -> Varchar,
        approvedby -> Nullable<Int4>,
        rejectedby -> Nullable<Int4>,
        #[max_length = 255]
        rejected_note -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        email_verified_at -> Nullable<Timestamp>,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        role -> Nullable<Varchar>,
        #[max_length = 255]
        nik -> Nullable<Varchar>,
        company_id -> Nullable<Int8>,
        department_id -> Nullable<Int8>,
        #[max_length = 255]
        position -> Nullable<Varchar>,
        join_date -> Nullable<Date>,
        exit_date -> Nullable<Date>,
        #[max_length = 255]
        photo -> Nullable<Varchar>,
        #[max_length = 255]
        phone -> Nullable<Varchar>,
        #[max_length = 255]
        address -> Nullable<Varchar>,
        #[max_length = 255]
        gender -> Nullable<Varchar>,
        #[max_length = 255]
        religion -> Nullable<Varchar>,
        #[max_length = 255]
        blood_type -> Nullable<Varchar>,
        birth_date -> Nullable<Date>,
        #[max_length = 255]
        birth_place -> Nullable<Varchar>,
        #[max_length = 255]
        marital_status -> Nullable<Varchar>,
        #[max_length = 255]
        emergency_contact_name -> Nullable<Varchar>,
        #[max_length = 255]
        emergency_contact_relationship -> Nullable<Varchar>,
        #[max_length = 255]
        emergency_contact_phone -> Nullable<Varchar>,
        #[max_length = 255]
        emergency_contact_address -> Nullable<Varchar>,
        #[max_length = 255]
        status -> Nullable<Varchar>,
        #[max_length = 100]
        remember_token -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    work_locations (id) {
        id -> Int8,
        #[max_length = 255]
        wl_code -> Varchar,
        #[max_length = 255]
        wl_name -> Varchar,
        #[max_length = 255]
        wl_address -> Varchar,
        #[max_length = 255]
        latitude -> Nullable<Varchar>,
        #[max_length = 255]
        longitude -> Nullable<Varchar>,
        #[max_length = 255]
        radius -> Nullable<Varchar>,
        project_id -> Int8,
        company_id -> Int8,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    work_schedules (id) {
        id -> Int8,
        working_date -> Date,
        company_id -> Int8,
        user_id -> Int8,
        working_hour_id -> Int8,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    working_hours (id) {
        id -> Int8,
        #[max_length = 255]
        wh_code -> Varchar,
        #[max_length = 255]
        wh_name -> Varchar,
        company_id -> Int8,
        #[max_length = 255]
        status -> Varchar,
        wh_before -> Time,
        wh_start -> Time,
        wh_late -> Time,
        wh_end -> Time,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

// diesel::table! {
//     hris.companies (id) {
//         id -> Int4,
//         company_code -> Varchar,
//         company_name -> Varchar,
//         photo -> Nullable<Varchar>,
//         address -> Varchar,
//         latitude -> Nullable<Float8>,
//         longitude -> Nullable<Float8>,
//         status -> Varchar,
//         created_at -> Nullable<Timestamp>,
//         updated_at -> Nullable<Timestamp>,
//     }
// }

// diesel::table! {
//     hris.projects (id) {
//         id -> Int4,
//         project_code -> Varchar,
//         project_name -> Varchar,
//         company_id -> Int8,
//         created_at -> Nullable<Timestamp>,
//         updated_at -> Nullable<Timestamp>,
//     }
// }

// diesel::joinable!(hris.projects -> hris.companies (company_id));
diesel::joinable!(attendance_histories -> users (user_id));
diesel::joinable!(departments -> companies (company_id));
diesel::joinable!(user_request_histories -> users (user_id));
diesel::joinable!(projects -> companies (company_id));
diesel::joinable!(work_locations -> companies (company_id));
diesel::joinable!(work_locations -> projects (project_id));
diesel::joinable!(work_schedules -> companies (company_id));
diesel::joinable!(work_schedules -> users (user_id));
diesel::joinable!(work_schedules -> working_hours (working_hour_id));
diesel::joinable!(working_hours -> companies (company_id));

diesel::allow_tables_to_appear_in_same_query!(
    attendance_histories,
    cache,
    cache_locks,
    companies,
    departments,
    failed_jobs,
    job_batches,
    jobs,
    migrations,
    password_reset_tokens,
    personal_access_tokens,
    projects,
    sessions,
    user_request_histories,
    users,
    work_locations,
    work_schedules,
    working_hours,
);
