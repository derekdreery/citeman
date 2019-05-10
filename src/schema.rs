table! {
    authors (id) {
        id -> Integer,
        given_names -> Text,
        family_name -> Text,
    }
}

table! {
    documents (id) {
        id -> Integer,
        title -> Text,
        time_created -> Text,
        time_updated -> Text,
        doc_type -> Text,
        paper_location -> Text,
    }
}

table! {
    documents_authors (document_id, author_id) {
        document_id -> Integer,
        author_id -> Integer,
    }
}

table! {
    papers (document_id) {
        document_id -> Integer,
        year -> Nullable<Integer>,
        volume -> Nullable<Integer>,
        number -> Nullable<Integer>,
        pages -> Binary,
    }
}

joinable!(documents_authors -> authors (author_id));
joinable!(documents_authors -> documents (document_id));
joinable!(papers -> documents (document_id));

allow_tables_to_appear_in_same_query!(
    authors,
    documents,
    documents_authors,
    papers,
);
