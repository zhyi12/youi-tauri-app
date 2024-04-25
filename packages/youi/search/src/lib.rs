///
///
///
extern crate cang_jie;
extern crate tantivy_jieba;


use std::sync::Arc;
pub use tantivy::schema::*;
pub use tantivy::Index;
pub use tantivy::collector::TopDocs;
use tantivy::{IndexWriter};
use jieba_rs::Jieba;

use cang_jie::{CANG_JIE, CangJieTokenizer, TokenizerOption};

use tantivy::query::QueryParser;
#[derive(Debug)]
pub struct MatchItem{
    pub score:f32,
    pub doc:Document
}
///
///
///
pub enum FieldValue{
    Text(String),
    F64(f64),
    I64(i64)
}
///
/// 创建内存索引
///
pub fn create_ram_index(fields:&Vec<String>)->Index{
    let mut schema_builder = SchemaBuilder::default();

    let text_indexing = text_field_indexing();
    let text_options = text_options(text_indexing);

    fields.iter().for_each(|field|{
        schema_builder.add_text_field(field, text_options.clone());
    });

    let schema = schema_builder.build();

    let index = Index::create_in_ram(schema);
    index.tokenizers().register(CANG_JIE, tokenizer()); // Build cang-jie Tokenizer

    index
}

///
///
///
pub fn create_writer(index:&Index)->IndexWriter{
    index.writer(50 * 1024 * 1024).unwrap()
}
///
///
///
pub fn write_row_data(index:&Index,index_writer:&mut IndexWriter,fields:&Vec<String>,values: &Vec<FieldValue>){
    if fields.len() == values.len(){
        let mut doc = Document::default();
        for i in 0..fields.len(){
            let value = &values[i];
            match value {
                FieldValue::Text(t) => {
                    doc.add_text(index.schema().get_field(&fields[i]).unwrap(),&t);
                }
                FieldValue::F64(f) => {
                    doc.add_f64(index.schema().get_field(&fields[i]).unwrap(),*f);
                }
                FieldValue::I64(v) => {
                    doc.add_i64(index.schema().get_field(&fields[i]).unwrap(),*v);
                }
            }
        }
        index_writer.add_document(doc).unwrap();
    }
}

///
pub fn search(index:&Index,term:&str,limit:usize,match_fields:&Vec<String>)->Vec<MatchItem>{
    let fields:Vec<Field> = match_fields.iter()
        .map(|name|index.schema().get_field(name.as_str()).unwrap()).collect();

    let reader = index.reader().unwrap();
    let searcher = reader.searcher();

    let query = QueryParser::for_index(index, fields).parse_query(term).unwrap();

    let top_docs = searcher.search(query.as_ref(), &TopDocs::with_limit(limit)).unwrap();

    let mut matched_items:Vec<MatchItem> = Vec::with_capacity(top_docs.len());

    for (score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address).unwrap();
        matched_items.push(MatchItem{
            score,
            doc:retrieved_doc
        });
    }
    matched_items
}

///
///
///
fn text_field_indexing()->TextFieldIndexing{
    TextFieldIndexing::default()
        .set_tokenizer(CANG_JIE) // Set custom tokenizer
        .set_index_option(IndexRecordOption::WithFreqsAndPositions)
}

fn text_options(text_indexing:TextFieldIndexing)->TextOptions{
    TextOptions::default()
        .set_indexing_options(text_indexing)
        .set_stored()
}

fn tokenizer() -> CangJieTokenizer {
    CangJieTokenizer {
        worker: Arc::new(Jieba::empty()), // empty dictionary
        option: TokenizerOption::Unicode,
    }
}
