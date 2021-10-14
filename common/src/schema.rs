table! {
    delegations (id) {
        id -> Integer,
        subject_id -> Text,
        object_id -> Text,
        issuer_id -> Text,
    }
}

table! {
    entities (id) {
        id -> Integer,
        actor_id -> Text,
        name -> Text,
        secret_key -> Nullable<Binary>,
        public_key -> Nullable<Binary>,
    }
}

table! {
    entity_central_relation (id) {
        id -> Integer,
        entity_id -> Text,
        central_key -> Binary,
    }
}

table! {
    roles (id) {
        id -> Integer,
        actor_id -> Text,
        entity_id -> Text,
        name -> Text,
        is_assignment -> Bool,
        secret_key -> Nullable<Binary>,
        public_key -> Nullable<Binary>,
    }
}

table! {
    users (id) {
        id -> Integer,
        actor_id -> Text,
        entity_id -> Text,
        name -> Text,
        secret_key -> Nullable<Binary>,
        public_key -> Nullable<Binary>,
    }
}

allow_tables_to_appear_in_same_query!(
    delegations,
    entities,
    entity_central_relation,
    roles,
    users,
);
