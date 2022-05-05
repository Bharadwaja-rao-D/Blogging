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
        creator_id -> Integer,
        upvotes -> Integer,
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
joinable!(content -> student (creator_id));

allow_tables_to_appear_in_same_query!(
    commenting,
    content,
    student,
);
