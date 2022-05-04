table! {
    commenting (id) {
        id -> Integer,
        commentor_id -> Integer,
        content_id -> Integer,
    }
}

table! {
    content (id) {
        id -> Integer,
        title -> Text,
        description -> Text,
        body -> Text,
        creater_id -> Nullable<Integer>,
        upvotes -> Nullable<Integer>,
        down -> Nullable<Integer>,
    }
}

table! {
    student (id) {
        id -> Integer,
        name -> Text,
        password -> Text,
    }
}

joinable!(commenting -> content (content_id));
joinable!(commenting -> student (commentor_id));
joinable!(content -> student (creater_id));

allow_tables_to_appear_in_same_query!(
    commenting,
    content,
    student,
);
