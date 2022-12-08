// @generated automatically by Diesel CLI.

diesel::table! {
    commenting (id) {
        id -> Integer,
        commentor_id -> Integer,
        content_id -> Integer,
        comment_text -> Text,
    }
}

diesel::table! {
    content (id) {
        id -> Integer,
        title -> Text,
        description -> Text,
        body -> Text,
        creator_id -> Integer,
        upvotes -> Integer,
    }
}

diesel::table! {
    student (id) {
        id -> Integer,
        name -> Text,
        password -> Text,
    }
}

diesel::joinable!(commenting -> content (content_id));
diesel::joinable!(commenting -> student (commentor_id));
diesel::joinable!(content -> student (creator_id));

diesel::allow_tables_to_appear_in_same_query!(
    commenting,
    content,
    student,
);
