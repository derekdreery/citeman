//! Structure for getting metadata from crossref.org and decoding it.
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Work {
    publisher: String,
    title: Vec<String>,
    #[serde(rename = "original-title")]
    original_title: Option<Vec<String>>,
    #[serde(rename = "short-title")]
    short_title: Option<Vec<String>>,
    #[serde(rename = "abstract")]
    abstract_: Option<String>,
    #[serde(rename = "reference-count")]
    reference_count: u32,
    #[serde(rename = "references-count")]
    references_count: u32,
    #[serde(rename = "is-referenced-by-count")]
    is_referenced_by_count: u32,
    source: String,
    prefix: String,
    #[serde(rename = "DOI")]
    doi: String,
    #[serde(rename = "URL")]
    url: String,
    member: String,
    #[serde(rename = "type")]
    type_: String,
    created: Date,
    deposited: Date,
    indexed: Date,
    issued: PartialDate,
    posted: Option<PartialDate>,
    accepted: Option<PartialDate>,
    subtitle: Option<Vec<String>>,
    #[serde(rename = "container-title")]
    container_title: Option<Vec<String>>,
    #[serde(rename = "short-container-title")]
    short_container_title: Option<Vec<String>>,
    #[serde(rename = "group-title")]
    group_title: Option<String>,
    issue: Option<String>,
    volume: Option<String>,
    page: Option<String>,
    #[serde(rename = "article-number")]
    article_number: Option<String>,
    #[serde(rename = "published-print")]
    published_print: Option<PartialDate>,
    #[serde(rename = "published-online")]
    published_online: Option<PartialDate>,
    subject: Option<Vec<String>>,
    #[serde(rename = "ISSN")]
    issn: Option<Vec<String>>,
    #[serde(rename = "issn-type")]
    issn_type: Option<Vec<IssnWithType>>,
    #[serde(rename = "ISBN")]
    isbn: Option<Vec<String>>,
    archive: Option<Vec<String>>,
    license: Option<Vec<License>>,
    funder: Option<Vec<Funder>>,
    assertion: Option<Vec<Assertion>>,
    author: Option<Vec<Contributor>>,
    editor: Option<Vec<Contributor>>,
    chair: Option<Vec<Contributor>>,
    translator: Option<Vec<Contributor>>,
    #[serde(rename = "update-to")]
    update_to: Option<Vec<Update>>,
    #[serde(rename = "update-policy")]
    update_policy: Option<String>,
    link: Option<Vec<ResourceLink>>,
    #[serde(rename = "clinical-trial-number")]
    clinical_trial_number: Option<Vec<ClinicalTrialNumber>>,
    #[serde(rename = "alternative-id")]
    alternative_id: Option<Vec<ClinicalTrialNumber>>,
    reference: Option<Vec<Reference>>,
    #[serde(rename = "content-domain")]
    content_domain: Option<ContentDomain>,
    relation: Option<Relations>,
    review: Option<Review>,
}

#[derive(Debug, Deserialize)]
pub struct CrossRefMessage<'a, Msg> {
    #[serde(borrow)]
    status: &'a str,
    #[serde(rename = "message-type", borrow)]
    message_type: &'a str,
    #[serde(rename = "message-version", borrow)]
    message_version: &'a str,
    message: Msg,
}

#[derive(Debug, Deserialize)]
pub struct Funder {
    pub name: String,
    pub doi: Option<String>,
    pub award: Option<Vec<String>>,
    #[serde(rename = "doi-asserted-by")]
    pub doi_asserted_by: Option<DoiAssertedBy>,
}

#[derive(Debug, Deserialize)]
pub enum DoiAssertedBy {
    #[serde(rename = "crossref")]
    CrossRef,
    #[serde(rename = "publisher")]
    Publisher,
}

#[derive(Debug, Deserialize)]
pub struct ClinicalTrialNumber {
    #[serde(rename = "clinical-trial-number")]
    number: String,
    registry: String,
    #[serde(rename = "type")]
    type_: Option<ResultsType>,
}

#[derive(Debug, Deserialize)]
pub enum ResultsType {
    #[serde(rename = "preResults")]
    PreResults,
    #[serde(rename = "results")]
    Results,
    #[serde(rename = "postResults")]
    PostResults,
}

#[derive(Debug, Deserialize)]
pub struct Contributor {
    family: String,
    given: Option<String>,
    #[serde(rename = "ORCID")]
    orcid: Option<String>,
    #[serde(rename = "authenticated-orcid")]
    authenticated_orcid: Option<bool>,
    affiliation: Option<Vec<Affiliation>>
}

#[derive(Debug, Deserialize)]
pub struct Affiliation {
    name: String
}

#[derive(Debug, Deserialize)]
pub struct Date {
    #[serde(rename = "date-parts")]
    date_parts: [[u16; 3]; 1],
    timestamp: u64,
    #[serde(rename = "date-time")]
    date_time: String,
}

#[derive(Debug, Deserialize)]
pub struct PartialDate {
    #[serde(rename = "date-parts")]
    date_parts: Vec<Vec<u16>>,
}

#[derive(Debug, Deserialize)]
pub struct Update {
    updated: PartialDate,
    #[serde(rename = "DOI")]
    doi: String,
    #[serde(rename = "type")]
    type_: String,
    label: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct Assertion {
    name: String,
    value: String,
    #[serde(rename = "URL")]
    url: Option<String>,
    explanation: Option<String>,
    label: Option<String>,
    order: Option<u32>, // XXX this may be the wrong type
    group: Option<AssertionGroup>,
}

#[derive(Debug, Deserialize)]
pub struct AssertionGroup {
    name: String,
    label: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct License {
    #[serde(rename = "content-version")]
    content_version: ContentVersion,
    #[serde(rename = "delay-in-days")]
    delay_in_days: u32,
    start: PartialDate,
    #[serde(rename = "URL")]
    url: String,
}

#[derive(Debug, Deserialize)]
pub enum ContentVersion {
    #[serde(rename = "vor")]
    VersionOfRecord,
    #[serde(rename = "am")]
    AcceptedManuscript,
    #[serde(rename = "tdm")]
    TextAndDataMining,
    #[serde(rename = "unspecified")]
    Unspecified,
}

#[derive(Debug, Deserialize)]
pub struct ResourceLink {
    #[serde(rename = "indented-application")]
    intended_application: IntendedApplication,
    #[serde(rename = "content-version")]
    content_version: ContentVersion,
    #[serde(rename = "URL")]
    url: String,
    #[serde(rename = "content-type")]
    content_type: Option<String>, // this is a MIME type.
}

#[derive(Debug, Deserialize)]
pub enum IntendedApplication {
    #[serde(rename = "text-mining")]
    TextMining,
    #[serde(rename = "similarity-checking")]
    SimilarityChecking,
    #[serde(rename = "unspecified")]
    Unspecified,
}

#[derive(Debug, Deserialize)]
pub struct Reference {
    key: String,
    #[serde(rename = "DOI")]
    doi: Option<String>,
    #[serde(rename = "doi-asserted-by")]
    doi_asserted_by: Option<DoiAssertedBy>,
    issue: Option<String>,
    #[serde(rename = "first-page")]
    first_page: Option<String>,
    volume: Option<String>,
    edition: Option<String>,
    component: Option<String>,
    #[serde(rename = "standard-designator")]
    standard_designator: Option<String>,
    #[serde(rename = "standards-body")]
    standards_body: Option<String>,
    author: Option<String>,
    year: Option<String>,
    unstructured: Option<String>,
    #[serde(rename = "journal-title")]
    journal_title: Option<String>,
    #[serde(rename = "article-title")]
    article_title: Option<String>,
    #[serde(rename = "series-title")]
    series_title: Option<String>,
    #[serde(rename = "volume-title")]
    volume_title: Option<String>,
    #[serde(rename = "ISSN")]
    issn: Option<String>,
    #[serde(rename = "issn-type")]
    issn_type: Option<IssnType>,
    #[serde(rename = "ISBN")]
    isbn: Option<String>,
    #[serde(rename = "isbn-type")]
    isbn_type: Option<IssnType>,
}

#[derive(Debug, Deserialize)]
pub enum IssnType {
    #[serde(rename = "pissn")]
    Pissn,
    #[serde(rename = "eissn")]
    Eissn,
    #[serde(rename = "lissn")]
    Lissn,
    // added by inspection
    #[serde(rename = "electronic")]
    Electronic,
}

#[derive(Debug, Deserialize)]
pub struct IssnWithType {
    value: String,
    #[serde(rename = "type")]
    type_: IssnType,
}

#[derive(Debug, Deserialize)]
pub struct ContentDomain {
    domain: Vec<String>,
    #[serde(rename = "crossmark-restriction")]
    crossmark_restriction: bool,
}

pub type Relations = HashMap<String, Vec<Relation>>;

#[derive(Debug, Deserialize)]
pub struct Relation {
    #[serde(rename = "id-type")]
    id_type: String,
    id: String,
    #[serde(rename = "asserted-by")]
    asserted_by: AssertedBy,
}

#[derive(Debug, Deserialize)]
pub enum AssertedBy {
    #[serde(rename = "subject")]
    Subject,
    #[serde(rename = "object")]
    Object,
}

#[derive(Debug, Deserialize)]
pub struct Review {
    #[serde(rename = "running-number")]
    running_number: Option<String>,
    #[serde(rename = "revision-round")]
    revision_round: Option<String>,
    stage: Option<Stage>,
    recommendation: Option<String>,
    #[serde(rename = "type")]
    type_: Option<ReviewType>,
    #[serde(rename = "competing-interest-statement")]
    competing_interest_statement: Option<String>,
    language: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum Stage {
    #[serde(rename = "pre-publication")]
    PrePublication,
    #[serde(rename = "post-publication")]
    PostPublication,
}

#[derive(Debug, Deserialize)]
pub enum Recommendation {
    #[serde(rename = "major-revision")]
    MajorRevision,
    #[serde(rename = "minor-revision")]
    MinorRevision,
    #[serde(rename = "reject")]
    Reject,
    #[serde(rename = "reject-with-resubmit")]
    RejectWithResubmit,
    #[serde(rename = "accept")]
    Accept,
}

#[derive(Debug, Deserialize)]
pub enum ReviewType {
    #[serde(rename = "referee-report")]
    RefereeReport,
    #[serde(rename = "editor-report")]
    EditorReport,
    #[serde(rename = "author-comment")]
    AuthorComment,
    #[serde(rename = "community-comment")]
    CommunityComment,
    #[serde(rename = "aggregate")]
    Aggregate,
}

/*
pub fn find_by_doi(doi: &str, client: reqwest::Client) -> () {
    ()
}
*/