use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;   
use near_sdk::serde::{Serialize, Deserialize};
extern crate shakmaty; 
extern crate base64; 
use shakmaty::{fen::Fen, fen::Epd, EnPassantMode, CastlingMode, Chess, Position, Square, Move, Role};

near_sdk::setup_alloc!();

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Web4Request {
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    pub path: String,
    #[serde(default)]
    pub params: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub query: std::collections::HashMap<String, Vec<String>>,
    pub preloads: Option<std::collections::HashMap<String, Web4Response>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde", untagged)]
pub enum Web4Response {
    Body {
        #[serde(rename = "contentType")]
        content_type: String,
        body: near_sdk::json_types::Base64VecU8,
    },
    BodyUrl {
        #[serde(rename = "bodyUrl")]
        body_url: String,
    },
    PreloadUrls {
        #[serde(rename = "preloadUrls")]
        preload_urls: Vec<String>,
    },
}

#[derive(Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TysItem {
    white: String,
    black: String,
    enter_block: u64,
    turn: u8,
    fen: String,
}
 

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct NChess { 
    items: Vec<TysItem>,
}

#[near_bindgen]
impl NChess {
    /// Learn more about web4 here: https://web4.near.page
    pub fn web4_get(&self, request: Web4Request) -> Web4Response {

        if request.path == "/" {
            Web4Response::Body {
                content_type: "text/html; charset=UTF-8".to_owned(),
                body: base64::decode("PCFET0NUWVBFIGh0bWw+CjxodG1sIGxhbmc9ImVuIj4KCjxoZWFkPgogIDxtZXRhIGNoYXJzZXQ9IlVURi04Ij4KICA8bWV0YSBodHRwLWVxdWl2PSJYLVVBLUNvbXBhdGlibGUiIGNvbnRlbnQ9IklFPWVkZ2UiPgogIDxtZXRhIG5hbWU9InZpZXdwb3J0IiBjb250ZW50PSJ3aWR0aD1kZXZpY2Utd2lkdGgsIGluaXRpYWwtc2NhbGU9MS4wIj4KICA8dGl0bGU+RG9jdW1lbnQ8L3RpdGxlPgo8L2hlYWQ+Cgo8Ym9keT4KICA8c3ZnIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgd2lkdGg9IjQ3NSIgaGVpZ2h0PSI1MDAiIHZpZXdCb3g9IjAwMCAwMDAgNDc1IDUwMCI+CiAgPHJlY3Qgd2lkdGg9IjEwMCUiIGhlaWdodD0iMTAwJSIgZmlsbD0iIzAwMCIgLz4KICA8L3N2Zz4KICA8c2NyaXB0IHNyYz0iaHR0cHM6Ly9jZG4uanNkZWxpdnIubmV0L25wbS9uZWFyLWFwaS1qc0AwLjQxLjAvZGlzdC9uZWFyLWFwaS1qcy5taW4uanMiPjwvc2NyaXB0PgoKCiAgPHNjcmlwdD4gIApjb25zdCBzdmcgPSBkb2N1bWVudC5xdWVyeVNlbGVjdG9yKCJzdmciKTsKIAovLyB2YXJpYWJsZSBmb3IgdGhlIG5hbWVzcGFjZSAKY29uc3Qgc3ZnbnMgPSAiaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciOyAKCgpjb25zdCBjb250cmFjdF9pZCA9ICcxY2hlc3MudGVzdG5ldCc7CiAKLy8xIGNvbm5lY3QgdG8gTkVBUgpjb25zdCBuZWFyID0gbmV3IG5lYXJBcGkuTmVhcih7CiAga2V5U3RvcmU6IG5ldyBuZWFyQXBpLmtleVN0b3Jlcy5Ccm93c2VyTG9jYWxTdG9yYWdlS2V5U3RvcmUoKSwKICBuZXR3b3JrSWQ6ICd0ZXN0bmV0JywKICBub2RlVXJsOiAnaHR0cHM6Ly9ycGMudGVzdG5ldC5uZWFyLm9yZycsCiAgd2FsbGV0VXJsOiAnaHR0cHM6Ly93YWxsZXQudGVzdG5ldC5uZWFyLm9yZycKfSk7CgoKLy8yIGNvbm5lY3QgdG8gdGhlIE5FQVIgV2FsbGV0CmNvbnN0IHdhbGxldCA9IG5ldyBuZWFyQXBpLldhbGxldENvbm5lY3Rpb24obmVhciwgJ215LWFwcCcpOwoKLy8zIGNvbm5lY3QgdG8gYSBORUFSIHNtYXJ0IGNvbnRyYWN0CmNvbnN0IGNvbnRyYWN0ID0gbmV3IG5lYXJBcGkuQ29udHJhY3Qod2FsbGV0LmFjY291bnQoKSwgY29udHJhY3RfaWQsIHsKICB2aWV3TWV0aG9kczogWydnZXRfY29vcmRzJ10sCiAgY2hhbmdlTWV0aG9kczogWydzZXRfbW92ZScsICdzZXRfZW50ZXInLCAnc2V0X2VuZCddIAp9KTsKCgp2YXIgd2lkID0gd2FsbGV0LmdldEFjY291bnRJZCgpOwp2YXIgdmlldyA9ICIiOwp2YXIgb3BwbyA9ICIiOwoKIAppZiAoIXdhbGxldC5pc1NpZ25lZEluKCkpIHsKICBjb25zdCByZXNwb25zZSA9IGNvbnRyYWN0LmdldF9jb29yZHMoewogICAgICAgIGlkOiB3aWQKICAgICAgfSkudGhlbiggCiAgICAgICAgZnVuY3Rpb24odmFsdWUpIHsKICAgICAgICAgIC8vIGFsZXJ0KHZhbHVlWzBdKTsKICAgICAgICAgIGlmICggdmFsdWVbMF0gPT0gIkJvYXJkIG5vdCBmb3VuZCEiICkgewogICAgICAgICAgICBvcHBvPXByb21wdCgiRW50ZXIgb3Bwb25lbnQgTkVBUiB3YWxsZXQsIGxlYXZlIGJsYW5rIGZvciBhIHJhbmRvbSBvbmUsIHN0YXJ0IHdpdGggVklFVzp3YWxsZXQgZm9yIHZpZXdpbmciLCIiKTsKICAgICAgICAgICAgaWYgKCBvcHBvLmluY2x1ZGVzKCJWSUVXOiIpID09IHRydWUgKSB7CiAgICAgICAgICAgICAgLy8gYWxlcnQob3Bwbyk7CiAgICAgICAgICAgICAgY3ZpZXcgPSBvcHBvLnJlcGxhY2UoIlZJRVc6IiwgIiIpOwogICAgICAgICAgICAgIHZpZXcgPSBjdmlldzsKICAgICAgICAgICAgICB3aWQgPSB2aWV3OwogICAgICAgICAgICB9IGVsc2UgeyAKICAgICAgICAgICAgICB3YWxsZXQucmVxdWVzdFNpZ25Jbih7CiAgICAgICAgICAgICAgICBjb250cmFjdElkOiBjb250cmFjdF9pZCwKICAgICAgICAgICAgICAgIG1ldGhvZE5hbWVzOiBbJ2dldF9jb29yZHMnLCAnc2V0X21vdmUnLCAnc2V0X2VudGVyJywgJ3NldF9lbmQnXQogICAgICAgICAgICAgIH0pOyAKICAgICAgICAgICAgfQogICAgICAgICAgfSAgCiAgICAgICAgfSwKICAgICAgICBmdW5jdGlvbihlcnJvcikgewogICAgICAgICAgYWxlcnQoZXJyb3IpOyAKICAgICAgICB9CiAgKTsKfSAKCiAKLy8gaWYgKCB3YWxsZXQuaXNTaWduZWRJbigpIHx8IHZpZXcgIT0gIiIgKSB7Ci8vICAgLy9ncmVhdAovLyB9IGVsc2UgewovLyAgIHdhbGxldC5yZXF1ZXN0U2lnbkluKHsKLy8gICAgIGNvbnRyYWN0SWQ6IGNvbnRyYWN0X2lkLAovLyAgICAgbWV0aG9kTmFtZXM6IFsnZ2V0X2Nvb3JkcycsICdzZXRfbW92ZScsICdzZXRfZW50ZXInLCAnc2V0X2VuZCddCi8vICAgfSk7Ci8vIH0gIAogCnZhciBtdWx0ID0gNTAKdmFyIHN0ZXBzID0gNTAKLy8gaWYgKCB2aWV3ID09ICIiICkgeyAKLy8gICAgd2lkID0gd2FsbGV0LmdldEFjY291bnRJZCgpOwovLyAgICAgdmFyIGNvbnRyYWN0ID0gbmV3IG5lYXJBcGkuQ29udHJhY3Qod2FsbGV0LmFjY291bnQoKSwgY29udHJhY3RfaWQsIHsKLy8gICAgICAgdmlld01ldGhvZHM6IFsnZ2V0X2Nvb3JkcyddLAovLyAgICAgICBjaGFuZ2VNZXRob2RzOiBbJ3NldF9tb3ZlJywgJ3NldF9lbnRlcicsICdzZXRfZW5kJ10gCi8vICAgICB9KTsgCi8vIH0KdmFyIHp6eiA9IDE7CnZhciB4X3NoaWZ0ID0gc3RlcHM7CnZhciB5X3NoaWZ0ID0gc3RlcHM7CnZhciBlbnRlcmVkID0gMDsKCgp2YXIgY2xpY2tzID0gIiI7Cgpjb25zdCBhbHBoYWJldCA9IFsiQSIsIkIiLCJDIiwiRCIsIkUiLCJGIiwiRyIsIkgiLCJJIiwiSiIsIksiLCJMIiwiTSIsIk4iLCJPIiwiUCIsIlEiLCJSIiwiUyIsIlQiLCJVIiwiViIsIlciLCJYIiwiWSIsIloiXTsKCgppZiAoIHZpZXcgPT0gIiIgKSB7CiAgd2lkID0gd2FsbGV0LmdldEFjY291bnRJZCgpOyAKICBjb25zdCByZXNwb25zZSA9IGNvbnRyYWN0LmdldF9jb29yZHMoewogICAgICAgIGlkOiB3aWQKICAgICAgfSkudGhlbiggCiAgICAgICAgZnVuY3Rpb24odmFsdWUpIHsKICAgICAgICAgIC8vIGFsZXJ0KHZhbHVlWzBdKTsKICAgICAgICAgIGlmICggdmFsdWVbMF0gPT0gIkJvYXJkIG5vdCBmb3VuZCEiICkgewogICAgICAgICAgICBpZiAod2FsbGV0LmlzU2lnbmVkSW4oKSkgewogICAgICAgICAgICAgIG9wcG89cHJvbXB0KCJFbnRlciBvcHBvbmVudCBORUFSIHdhbGxldCwgbGVhdmUgYmxhbmsgZm9yIGEgcmFuZG9tIG9uZSwgc3RhcnQgd2l0aCBWSUVXOndhbGxldCBmb3Igdmlld2luZyIsIiIpOwogICAgICAgICAgICAgIGlmICggb3Bwby5pbmNsdWRlcygiVklFVzoiKSA9PSB0cnVlICkgewogICAgICAgICAgICAgICAgLy8gYWxlcnQob3Bwbyk7CiAgICAgICAgICAgICAgICBjdmlldyA9IG9wcG8ucmVwbGFjZSgiVklFVzoiLCAiIik7CiAgICAgICAgICAgICAgICB2aWV3ID0gY3ZpZXc7CiAgICAgICAgICAgICAgICB3aWQgPSB2aWV3OwogICAgICAgICAgICAgIH0gZWxzZSB7IAogICAgICAgICAgICAgICAgY29udHJhY3Quc2V0X2VudGVyKHsKICAgICAgICAgICAgICAgICAgYXJnczogeyAKICAgICAgICAgICAgICAgICAgICBvcHBvbmVudDogb3BwbyAKICAgICAgICAgICAgICAgICAgfQogICAgICAgICAgICAgICAgfSk7CiAgICAgICAgICAgICAgfQogICAgICAgICAgICAgIHNldFRpbWVvdXQoZnVuY3Rpb24oKSB7CiAgICAgICAgICAgICAgfSwgMTAwMCkKICAgICAgICAgICAgfQogICAgICAgICAgICAvLyB9CiAgICAgICAgICB9ICAKICAgICAgICB9LAogICAgICAgIGZ1bmN0aW9uKGVycm9yKSB7CiAgICAgICAgICBhbGVydChlcnJvcik7IAogICAgICAgIH0KICApOwp9IGVsc2UgewogIHdpZCA9IHZpZXc7Cn0gCgoKdmFyIG9iamVjdHMgPSB7fTsKdmFyIGZpZ3MgPSB7fTsKdmFyIHRpbWVzID0gODsKeCA9IDcwOwpsdl93aGl0ZSA9IHRydWU7CmZvcih2YXIgaSA9IDA7IGkgPCB0aW1lczsgaSsrKXsgCiAgeCA9IG11bHQgKiBpOwogIHkgPSBtdWx0OwogIGx2X2FiYyA9IGRvY3VtZW50LmNyZWF0ZUVsZW1lbnROUyhzdmducywgInRleHQiKTsKICBsdl9hYmMuc2V0QXR0cmlidXRlKCJ4IiwgeF9zaGlmdCArIHggKTsKICBsdl9hYmMuc2V0QXR0cmlidXRlKCJ5IiwgeSApOwogIGx2X2FiYy5zZXRBdHRyaWJ1dGUoImZpbGwiLCAiZ3JlZW4iKTsgIAogIGx2X2FiYy50ZXh0Q29udGVudCA9IGFscGhhYmV0W2ldOyAgCiAgc3ZnLmFwcGVuZENoaWxkKGx2X2FiYyk7CgogIHkzID0gbXVsdCAqIGk7CiAgeDMgPSAxNzsKICBsdl9hYmMgPSBkb2N1bWVudC5jcmVhdGVFbGVtZW50TlMoc3ZnbnMsICJ0ZXh0Iik7CiAgbHZfYWJjLnNldEF0dHJpYnV0ZSgieCIsIHgzICk7CiAgbHZfYWJjLnNldEF0dHJpYnV0ZSgieSIsIDg1ICsgeTMgKTsKICBsdl9hYmMuc2V0QXR0cmlidXRlKCJmaWxsIiwgImdyZWVuIik7ICAKICBsdl9hYmMudGV4dENvbnRlbnQgPSA4IC0gaTsgIAogIHN2Zy5hcHBlbmRDaGlsZChsdl9hYmMpOwogIAogIGZvcih2YXIgaTIgPSAwOyBpMiA8IHRpbWVzOyBpMisrKXsgCiAgICB4aWQgPSA4IC0gaTI7CiAgICByZWN0X2lkID0gYWxwaGFiZXRbaV0gKyB4aWQ7IAoKICAgIG9iamVjdHNbcmVjdF9pZF0gPSBkb2N1bWVudC5jcmVhdGVFbGVtZW50TlMoc3ZnbnMsICJyZWN0Iik7IAogICAgZnggPSAzMCArIHg7CiAgICBmeSA9IDU1ICsgeSppMjsKICAgIG9iamVjdHNbcmVjdF9pZF0uc2V0QXR0cmlidXRlKCJ4IiwgZnggKTsKICAgIG9iamVjdHNbcmVjdF9pZF0uc2V0QXR0cmlidXRlKCJ5IiwgZnkgKTsKICAgIGlmICggbHZfd2hpdGUgPT0gdHJ1ZSApIHsKICAgICAgb2JqZWN0c1tyZWN0X2lkXS5zZXRBdHRyaWJ1dGUoImZpbGwiLCAib3JhbmdlIik7CiAgICAgIGx2X3doaXRlID0gZmFsc2U7CiAgICB9IGVsc2UgeyAKICAgICAgb2JqZWN0c1tyZWN0X2lkXS5zZXRBdHRyaWJ1dGUoImZpbGwiLCAiZ3JleSIpOwogICAgICBsdl93aGl0ZSA9IHRydWU7CiAgICB9CiAgICBvYmplY3RzW3JlY3RfaWRdLnNldEF0dHJpYnV0ZSgid2lkdGgiLCBtdWx0KTsKICAgIG9iamVjdHNbcmVjdF9pZF0uc2V0QXR0cmlidXRlKCJoZWlnaHQiLCBtdWx0KTsKICAgIG9iamVjdHNbcmVjdF9pZF0uc2V0QXR0cmlidXRlKCJuYW1lIiwgcmVjdF9pZCk7CgogICAgb2JqZWN0c1tyZWN0X2lkXS5hZGRFdmVudExpc3RlbmVyKCJjbGljayIsIHJlY3RfY2xpY2spOyAgCiAgICBzdmcuYXBwZW5kQ2hpbGQob2JqZWN0c1tyZWN0X2lkXSk7CgoKICAgIGZpZ3NbcmVjdF9pZF0gPSBkb2N1bWVudC5jcmVhdGVFbGVtZW50TlMoc3ZnbnMsICJ0ZXh0Iik7CiAgICBmeCA9IHBhcnNlSW50KGZ4KSArIHhfc2hpZnQgLyAyIC0gOTsKICAgIGZ5ID0gcGFyc2VJbnQoZnkpICsgeV9zaGlmdCAvIDIgKyA5OyAgCiAgICBmaWdzW3JlY3RfaWRdLnNldEF0dHJpYnV0ZSgieCIsIGZ4ICk7CiAgICBmaWdzW3JlY3RfaWRdLnNldEF0dHJpYnV0ZSgieSIsIGZ5ICk7ICAKICAgIGZpZ3NbcmVjdF9pZF0uc2V0QXR0cmlidXRlKCJmb250LXNpemUiLCAieHgtbGFyZ2UiICk7ICAKICAgIGZpZ3NbcmVjdF9pZF0udGV4dENvbnRlbnQgPSAiIjsKICAgIGZpZ3NbcmVjdF9pZF0uc2V0QXR0cmlidXRlKCJuYW1lIiwgcmVjdF9pZCk7IAogICAgZmlnc1tyZWN0X2lkXS5hZGRFdmVudExpc3RlbmVyKCJjbGljayIsIHJlY3RfY2xpY2spOyAKICAgIHN2Zy5hcHBlbmRDaGlsZChmaWdzW3JlY3RfaWRdKTsKCiAgfQogIGlmICggbHZfd2hpdGUgPT0gdHJ1ZSApIHsgCiAgICBsdl93aGl0ZSA9IGZhbHNlOwogIH0gZWxzZSB7ICAKICAgIGx2X3doaXRlID0gdHJ1ZTsKICB9Cn0KCgpsdl93aGl0ZV93aWQgPSBkb2N1bWVudC5jcmVhdGVFbGVtZW50TlMoc3ZnbnMsICJ0ZXh0Iik7IApsdl93aGl0ZV93aWQuc2V0QXR0cmlidXRlKCJ4IiwgMjAgKTsKbHZfd2hpdGVfd2lkLnNldEF0dHJpYnV0ZSgieSIsIDIwICk7ICAKbHZfd2hpdGVfd2lkLnNldEF0dHJpYnV0ZSgiZmlsbCIsICJ5ZWxsb3ciICk7ICAKbHZfd2hpdGVfd2lkLnRleHRDb250ZW50ID0gIiI7IApzdmcuYXBwZW5kQ2hpbGQobHZfd2hpdGVfd2lkKTsKCgpsdl9iX3dpZCA9IGRvY3VtZW50LmNyZWF0ZUVsZW1lbnROUyhzdmducywgInRleHQiKTsgCmx2X2Jfd2lkLnNldEF0dHJpYnV0ZSgieCIsIDIwICk7Cmx2X2Jfd2lkLnNldEF0dHJpYnV0ZSgieSIsIDQ4MCApOyAgCmx2X2Jfd2lkLnNldEF0dHJpYnV0ZSgiZmlsbCIsICJ5ZWxsb3ciICk7ICAKbHZfYl93aWQudGV4dENvbnRlbnQgPSAiIjsgCnN2Zy5hcHBlbmRDaGlsZChsdl9iX3dpZCk7Cgpsdl9iX2luZCA9IGRvY3VtZW50LmNyZWF0ZUVsZW1lbnROUyhzdmducywgInRleHQiKTsgCmx2X2JfaW5kLnNldEF0dHJpYnV0ZSgieCIsIDQwMCApOwpsdl9iX2luZC5zZXRBdHRyaWJ1dGUoInkiLCA0ODAgKTsgIApsdl9iX2luZC5zZXRBdHRyaWJ1dGUoImZpbGwiLCAibGltZSIgKTsgIApsdl9iX2luZC50ZXh0Q29udGVudCA9ICIiOyAKc3ZnLmFwcGVuZENoaWxkKGx2X2JfaW5kKTsKCgpsdl93X2luZCA9IGRvY3VtZW50LmNyZWF0ZUVsZW1lbnROUyhzdmducywgInRleHQiKTsgCmx2X3dfaW5kLnNldEF0dHJpYnV0ZSgieCIsIDQwMCApOwpsdl93X2luZC5zZXRBdHRyaWJ1dGUoInkiLCAzMCApOyAgCmx2X3dfaW5kLnNldEF0dHJpYnV0ZSgiZmlsbCIsICJsaW1lIiApOyAgCmx2X3dfaW5kLnRleHRDb250ZW50ID0gIiI7IApzdmcuYXBwZW5kQ2hpbGQobHZfd19pbmQpOwoKCmx2X2VuZCA9IGRvY3VtZW50LmNyZWF0ZUVsZW1lbnROUyhzdmducywgInJlY3QiKTsgCmx2X2VuZC5zZXRBdHRyaWJ1dGUoIngiLCA0NDAgKTsKbHZfZW5kLnNldEF0dHJpYnV0ZSgieSIsIDIyNSApOyAKbHZfZW5kLnNldEF0dHJpYnV0ZSgid2lkdGgiLCAyMik7Cmx2X2VuZC5zZXRBdHRyaWJ1dGUoImhlaWdodCIsIDU1KTsgIApsdl9lbmQuc2V0QXR0cmlidXRlKCJmaWxsIiwgInJlZCIgKTsKbHZfZW5kLnNldEF0dHJpYnV0ZSgicngiLCAxMyk7ICAKbHZfZW5kLmFkZEV2ZW50TGlzdGVuZXIoImNsaWNrIiwgZW5kX2NsaWNrKTsgIApzdmcuYXBwZW5kQ2hpbGQobHZfZW5kKTsKCmZ1bmN0aW9uIGVuZF9jbGljayhldnQpIHsKICB2YXIgbHZfZW5kX2NvbmQ9cHJvbXB0KCJQbGVhc2UgdHlwZSB5ZXMgdG8gZmluaXNoIHRoZSBnYW1lIiwieWVzIik7IC8vcGFwZXIgaEVORHMgXikKICBpZiAoIGx2X2VuZF9jb25kID09ICJ5ZXMiKSB7IAogICAgY29udHJhY3Quc2V0X2VuZCh7CiAgICAgICAgYXJnczogewogICAgICAgIH0gCiAgICB9KTsKICB9Cn0KCnZhciBsdl9mcm9tOwpmdW5jdGlvbiByZWN0X2NsaWNrKGV2dCkgewogIGlmICggY2xpY2tzID09ICIiKSB7CiAgICBjbGlja3MgPSBldnQudGFyZ2V0LmdldEF0dHJpYnV0ZSgibmFtZSIpOyAKICAgIGx2X2Zyb20gPSBvYmplY3RzW2NsaWNrc107IAoKICAgIGx2X2Zyb20uc2V0QXR0cmlidXRlKCJyeCIsIDE1KTsKCiAgfSBlbHNlIHsgCiAgICB0byA9IGV2dC50YXJnZXQuZ2V0QXR0cmlidXRlKCJuYW1lIik7CiAgICBsdl90byA9IGV2dC50YXJnZXQ7CgogICAgbHZfZnJvbV9yb2xlID0gZmlnc1tjbGlja3NdLnRleHRDb250ZW50OwogICAgbHZfZnJvbV91cHBlciA9IGx2X2Zyb21fcm9sZS50b1VwcGVyQ2FzZSgpOwogICAgaWYgKCBsdl9mcm9tX3JvbGUgIT0gIiIgKSB7CiAgICAgIGx2X3RvX3JvbGUgPSBmaWdzW3RvXS50ZXh0Q29udGVudDsKICAgICAgaWYgKCBsdl90b19yb2xlID09ICIiKSB7CiAgICAgICAgaWYgKCBsdl9mcm9tX3VwcGVyID09ICJQIiApIHsKICAgICAgICAgIGlmICggbHZfdG8uZ2V0QXR0cmlidXRlKCJ4IikgIT0gbHZfZnJvbS5nZXRBdHRyaWJ1dGUoIngiKSApIHsKICAgICAgICAgICAgbHZfdG9fcm9sZSA9ICJFblBhc3NhbnQiOyAvL9CR0LjRgtC+0LUg0L/QvtC70LUKICAgICAgICAgIH0KICAgICAgICB9CiAgICAgIH0KCiAgICAgIHZhciBsdl9wcm9tb3Rpb24gPSAiIjsKICAgICAgaWYgKCBsdl9mcm9tX3VwcGVyID09ICJQIiApIHsKICAgICAgICBpZiAoIHRvWzFdID09ICI4IiB8fCB0b1sxXSA9PSAiMSIgKSB7IAogICAgICAgICAgdmFyIG5hbWU9cHJvbXB0KCJQbGVhc2UgZW50ZXIgcHJvbW90aW9uX3JvbGUgKEJpc2hvcCwga05pZ2h0LCBSb29rLCBPZmZpY2VyKSIsIlF1ZWVuIik7CiAgICAgICAgICBsdl9wcm9tb3Rpb24gPSBuYW1lWzBdOwogICAgICAgICAgbHZfcHJvbW90aW9uLnRvVXBwZXJDYXNlKCk7CiAgICAgICAgICBpZiAoIGx2X2Zyb21fdXBwZXIgPT0gIksiICkgewogICAgICAgICAgICBsdl9mcm9tX3VwcGVyID0gIk4iOwogICAgICAgICAgfSAKICAgICAgICB9CiAgICAgIH0KCiAgICAgIGlmICggbHZfZnJvbV91cHBlciA9PSAnSycgKSB7CiAgICAgICAgaWYgKCBjbGlja3NbMF0gPT0gJ0UnICkgewogICAgICAgICAgaWYgKCB0b1swXSA9PSAiRyIgKSB7IAogICAgICAgICAgICAvLyBjbGlja3MgPSAiRiIgKyBjbGlja3NbMV07CiAgICAgICAgICAgIHRvID0gIkUiICsgY2xpY2tzWzFdOwogICAgICAgICAgICBjbGlja3MgPSAiSCIgKyBjbGlja3NbMV07CiAgICAgICAgICAgIGx2X3RvX3JvbGUgPSAiQ2FzdGxlIjsgLy/QoNC+0LrQuNGA0L7QstC60LAKICAgICAgICAgIH0gZWxzZSBpZiAoIHRvWzBdID09ICJDIiApIHsKICAgICAgICAgICAgLy8gY2xpY2tzID0gIkQiICsgY2xpY2tzWzFdOwogICAgICAgICAgICB0byA9ICJFIiArIGNsaWNrc1sxXTsKICAgICAgICAgICAgY2xpY2tzID0gIkEiICsgY2xpY2tzWzFdOwogICAgICAgICAgICBsdl90b19yb2xlID0gIkNhc3RsZSI7IC8v0KDQvtC60LjRgNC+0LLQutCwCiAgICAgICAgICB9CiAgICAgICAgfQogICAgICB9CgogICAgICBjb250cmFjdC5zZXRfbW92ZSh7CiAgICAgICAgYXJnczogeyAKICAgICAgICAgIGZyb206IGNsaWNrcywKICAgICAgICAgIHRvOiB0bywKICAgICAgICAgIHJvbGU6IGx2X2Zyb21fcm9sZSwKICAgICAgICAgIGNhcHR1cmVfcm9sZTogbHZfdG9fcm9sZSwKICAgICAgICAgIHByb21vdGlvbl9yb2xlOiBsdl9wcm9tb3Rpb24sCiAgICAgICAgfQogICAgICB9KTsKICAgIH0KICAgIGNsaWNrcyA9ICIiOwogICAgbHZfZnJvbS5zZXRBdHRyaWJ1dGUoInJ4IiwgMCk7CiAgfSAgICAKIAp9CiAKCihmdW5jdGlvbiBteUxvb3AoaSkgewogIHNldFRpbWVvdXQoZnVuY3Rpb24oKSB7CiAgICAvLyBhbGVydCh3aWQpOwogICAgLy9jYWxsIHRoZSBnZXRfY29vcmRzIHZpZXcgbWV0aG9kCiAgICBjb25zdCByZXNwb25zZSA9IGNvbnRyYWN0LmdldF9jb29yZHMoewogICAgICAgICAgaWQ6IHdpZAogICAgICAgIH0pLnRoZW4oIAogICAgICAgICAgZnVuY3Rpb24odmFsdWUpIHsgCiAgICAgICAgICAgIC8vIGFsZXJ0KHZhbHVlKSAKICAgICAgICAgICAgbHZfd2hpdGVfd2lkLnRleHRDb250ZW50ID0gdmFsdWVbMl07CiAgICAgICAgICAgIGx2X2Jfd2lkLnRleHRDb250ZW50ID0gdmFsdWVbMV07CiAgICAgICAgICAgIGlmICggdmFsdWVbM10gPT0gIjEiICkgewogICAgICAgICAgICAgIGx2X2JfaW5kLnRleHRDb250ZW50ID0gIiI7CiAgICAgICAgICAgICAgbHZfd19pbmQudGV4dENvbnRlbnQgPSAidHVybiI7IAogICAgICAgICAgICB9IGVsc2UgaWYgKCB2YWx1ZVszXSA9PSAiMCIgKSB7CiAgICAgICAgICAgICAgbHZfd19pbmQudGV4dENvbnRlbnQgPSAiIjsKICAgICAgICAgICAgICBsdl9iX2luZC50ZXh0Q29udGVudCA9ICJ0dXJuIjsgCiAgICAgICAgICAgIH0gZWxzZSBpZiAoIHZhbHVlWzNdID09ICIzIiApIHsKICAgICAgICAgICAgICBsdl93X2luZC50ZXh0Q29udGVudCA9ICIiOwogICAgICAgICAgICAgIGx2X2JfaW5kLnRleHRDb250ZW50ID0gIldJTk5FUiI7IAogICAgICAgICAgICB9IGVsc2UgaWYgKCB2YWx1ZVszXSA9PSAiMiIgKSB7CiAgICAgICAgICAgICAgbHZfd19pbmQudGV4dENvbnRlbnQgPSAiV0lOTkVSIjsKICAgICAgICAgICAgICBsdl9iX2luZC50ZXh0Q29udGVudCA9ICIiOyAKICAgICAgICAgICAgfSBlbHNlIGlmICggdmFsdWVbM10gPT0gIjciICkgewogICAgICAgICAgICAgIGx2X3dfaW5kLnRleHRDb250ZW50ID0gIkRFVUNFIjsKICAgICAgICAgICAgICBsdl9iX2luZC50ZXh0Q29udGVudCA9ICJERVVDRSI7IAogICAgICAgICAgICB9CgogICAgICAgICAgICB4eCA9IDA7CiAgICAgICAgICAgIHl5ID0gODsKICAgICAgICAgICAgdHQgPSA3MDsgCgogICAgICAgICAgICBsdl9mZW4gPSB2YWx1ZVswXTsgCiAgICAgICAgICAgIGZvcih2YXIgZmVuX2kgPSAwOyBmZW5faSA8IHR0OyBmZW5faSsrKXsgCiAgICAgICAgICAgICAgaWYgKCBsdl9mZW5bZmVuX2ldID09ICcgJyApIHsgCiAgICAgICAgICAgICAgICBicmVhazsKICAgICAgICAgICAgICB9IGVsc2UgaWYgKCBsdl9mZW5bZmVuX2ldID09ICcvJyApIHsgCiAgICAgICAgICAgICAgICB4eCA9IDA7CiAgICAgICAgICAgICAgICB5eS0tOwogICAgICAgICAgICAgICAgY29udGludWU7CiAgICAgICAgICAgICAgfSBlbHNlIGlmICggKC9bYS16QS1aXS8pLnRlc3QobHZfZmVuW2Zlbl9pXSkgPT0gdHJ1ZSApIHsgCiAgICAgICAgICAgICAgICByaWQgPSBhbHBoYWJldFt4eF0gKyB5eTsgCiAgICAgICAgICAgICAgICB4eCsrOyAKCiAgICAgICAgICAgICAgICBpZiAobHZfZmVuW2Zlbl9pXSA9PSBsdl9mZW5bZmVuX2ldLnRvVXBwZXJDYXNlKCkpIHsKICAgICAgICAgICAgICAgICAgZmlnc1tyaWRdLnNldEF0dHJpYnV0ZSgiZmlsbCIsICJ3aGl0ZSIpOyAgCiAgICAgICAgICAgICAgICB9IGVsc2UgewogICAgICAgICAgICAgICAgICBmaWdzW3JpZF0uc2V0QXR0cmlidXRlKCJmaWxsIiwgImJsYWNrIik7IAogICAgICAgICAgICAgICAgfQogICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICBmaWdzW3JpZF0udGV4dENvbnRlbnQgPSBsdl9mZW5bZmVuX2ldOyAgCiAgICAgICAgICAgICAgfSBlbHNlIHsgCiAgICAgICAgICAgICAgICBmb3IodmFyIGNsZWFyX2kgPSAwOyBjbGVhcl9pIDwgcGFyc2VJbnQobHZfZmVuW2Zlbl9pXSk7IGNsZWFyX2krKyl7CiAgICAgICAgICAgICAgICAgIHJpZCA9IGFscGhhYmV0W3h4XSArIHl5OwogICAgICAgICAgICAgICAgICB4eCsrOwogICAgICAgICAgICAgICAgICBmaWdzW3JpZF0udGV4dENvbnRlbnQgPSAiIjsKICAgICAgICAgICAgICAgIH0KICAgICAgICAgICAgICB9CiAgICAgICAgICAgIH0KICAgICAgICAgIH0sCiAgICAgICAgICBmdW5jdGlvbihlcnJvcikge2FsZXJ0KGVycm9yKTsgfQogICAgKTsKICAgIGlmICgtLWkpIG15TG9vcChpKTsgLy8gIGRlY3JlbWVudCBpIGFuZCBjYWxsIG15TG9vcCBhZ2FpbiBpZiBpID4gMAogIH0sIDMwMDApIC8vICB0aW1lb3V0IGluIG1pbGxpc2Vjb25kcwp9KSg1MDAwMCk7IC8vICBwYXNzIHRoZSBudW1iZXIgb2YgaXRlcmF0aW9ucyBhcyBhbiBhcmd1bWVudAogCjwvc2NyaXB0Pgo8L2JvZHk+Cgo8L2h0bWw+").unwrap().to_owned().into(),
            }
        } else {
            Web4Response::Body {
                content_type: "text/html; charset=UTF-8".to_owned(),
                body: format!("<h1>Some page</h1><pre>{:#?}</pre>", request).into_bytes().into(),
            } 
        }

    }

    pub fn set_enter(&mut self, opponent: String) -> (i8, i8, u64) {
        let acc = near_sdk::env::predecessor_account_id();
        let cblock = near_sdk::env::block_index().clone();
        let mut empty = String::from("@empty@");

        for _item in &self.items {
            if _item.white == acc {
                return (-99, -99, 0);
            } else if _item.black == acc {
                return (-99, -99, 1);
            }
        }

        let mut delete = false;
        for _item in &mut self.items {
            if _item.enter_block < cblock - 100000 {
                delete = true;
            }

            if opponent == "" {
                if _item.black == empty {
                    if _item.white != empty { 
                        _item.black = acc.clone();
                        return (1, 1, 1); 
                    }
                }
            } 

        }

        if delete == true {
            self.items.remove(0);
        } 


        if opponent != "" {
            empty = opponent;
        }

        let new_user = TysItem {
            white: acc,
            black: empty, 
            enter_block: cblock,
            turn: 0,
            fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -".to_string(),
        }; 

        self.items.push(new_user);  

        return (1, 0, 0);

    }


    pub fn set_move(&mut self, from: String, to: String, role:String, capture_role:String, promotion_role:String) -> String {
 

        let acc = near_sdk::env::predecessor_account_id();
        let role_c = role.clone();
        let lv_role = NChess::convert_role(role);
        if lv_role == Role::Knight {
            if role_c != "N" {
                if role_c != "n" {
                    return "No such role".to_string();
                }
            }
        }

        let mut lv_future_turn = 0;
        for _item in &mut self.items {
            if _item.white == acc { 
                if _item.turn == 1 {
                    return "Black turn".to_string();
                }
                lv_future_turn = 1;
            } else if _item.black == acc {
                if _item.turn == 0 { 
                    return "White turn".to_string();
                }
            } else  { 
                continue;
            }
 
            let fen: Fen = _item.fen.parse().unwrap(); 
     
            let pos: Chess = fen.into_position(CastlingMode::Standard).unwrap();

            if capture_role == "".to_string() {
                if promotion_role == "".to_string() { 
                    let pos = pos.play(&Move::Normal {
                        role: lv_role, 
                        from: NChess::convert_square(from), 
                        to: NChess::convert_square(to),
                        capture: None,
                        promotion: None,
                    }).unwrap();

                    if pos.is_checkmate() == true {
                        _item.turn = lv_future_turn + 2;
                    } else if pos.is_stalemate() == true {
                        _item.turn = 7;
                    } else {
                        _item.turn = lv_future_turn;
                    }

                    _item.fen = Epd::from_position(pos, EnPassantMode::Legal).to_string();
                    return _item.fen.clone();
                } else {
                    let pos = pos.play(&Move::Normal {
                        role: lv_role, 
                        from: NChess::convert_square(from), 
                        to: NChess::convert_square(to), 
                        capture: None,
                        promotion: Some(NChess::convert_role(promotion_role)),
                    }).unwrap();

                    if pos.is_checkmate() == true {
                        _item.turn = lv_future_turn + 2;
                    } else if pos.is_stalemate() == true {
                        _item.turn = 7;
                    } else {
                        _item.turn = lv_future_turn;
                    }
 
                    _item.fen = Epd::from_position(pos, EnPassantMode::Legal).to_string();
                    return _item.fen.clone();
                }

            } else {
                if promotion_role == "".to_string() {
 
                    if capture_role == "EnPassant" {
                        let pos = pos.play(&Move::EnPassant { 
                            from: NChess::convert_square(from), 
                            to: NChess::convert_square(to),
                        }).unwrap();

                        if pos.is_checkmate() == true {
                            _item.turn = lv_future_turn + 2;
                        } else if pos.is_stalemate() == true {
                            _item.turn = 7;
                        } else {
                            _item.turn = lv_future_turn;
                        }
 
                        _item.fen = Epd::from_position(pos, EnPassantMode::PseudoLegal).to_string();
                        return _item.fen.clone();

                    } else if capture_role == "Castle" {
                        let pos = pos.play(&Move::Castle { 
                            king: NChess::convert_square(to), 
                            rook: NChess::convert_square(from),
                        }).unwrap();

                        if pos.is_checkmate() == true {
                            _item.turn = lv_future_turn + 2;
                        } else if pos.is_stalemate() == true {
                            _item.turn = 7;
                        } else {
                            _item.turn = lv_future_turn;
                        }
 
                        _item.fen = Epd::from_position(pos, EnPassantMode::Legal).to_string();
                        return _item.fen.clone(); 

                    } else {  
                        let pos = pos.play(&Move::Normal {
                            role: lv_role, 
                            from: NChess::convert_square(from), 
                            to: NChess::convert_square(to),
                            capture: Some(NChess::convert_role(capture_role)), 
                            promotion: None,
                        }).unwrap();

                        if pos.is_checkmate() == true {
                            _item.turn = lv_future_turn + 2;
                        } else if pos.is_stalemate() == true {
                            _item.turn = 7;
                        } else {
                            _item.turn = lv_future_turn;
                        }
 
                        _item.fen = Epd::from_position(pos, EnPassantMode::Legal).to_string();
                        return _item.fen.clone();
                    }


                } else {
                    let pos = pos.play(&Move::Normal {
                        role: lv_role, 
                        from: NChess::convert_square(from), 
                        to: NChess::convert_square(to), 
                        capture: Some(NChess::convert_role(capture_role)),
                        promotion: Some(NChess::convert_role(promotion_role)),
                    }).unwrap();

                    if pos.is_checkmate() == true {
                        _item.turn = lv_future_turn + 2;
                    } else if pos.is_stalemate() == true {
                        _item.turn = 7;
                    } else {
                        _item.turn = lv_future_turn;
                    }
                     
                    _item.fen = Epd::from_position(pos, EnPassantMode::Legal).to_string();
                    return _item.fen.clone();
                }

            }
        }

        return "Not entered".to_string();


    }
         
    pub fn get_coords(&self, id: String) -> (String,String,String,String) {
  
        for _item in &self.items {
           if _item.white == id {
             return ( _item.fen.clone(), _item.white.clone(), _item.black.clone(), _item.turn.clone().to_string() );
           } else if _item.black == id {
             return ( _item.fen.clone(), _item.white.clone(), _item.black.clone(), _item.turn.clone().to_string() );
           }
        } 
        return ( "Board not found!".to_string(), "".to_string(), "".to_string(), "".to_string() );

    }
 

    fn convert_square(id:String) -> Square {
        match id.as_str() {
            "A1" => Square::A1,
            "B1" => Square::B1, 
            "C1" => Square::C1, 
            "D1" => Square::D1, 
            "E1" => Square::E1, 
            "F1" => Square::F1, 
            "G1" => Square::G1, 
            "H1" => Square::H1, 
            "A2" => Square::A2, 
            "B2" => Square::B2, 
            "C2" => Square::C2, 
            "D2" => Square::D2, 
            "E2" => Square::E2, 
            "F2" => Square::F2, 
            "G2" => Square::G2, 
            "H2" => Square::H2,  
            "A3" => Square::A3, 
            "B3" => Square::B3, 
            "C3" => Square::C3, 
            "D3" => Square::D3, 
            "E3" => Square::E3, 
            "F3" => Square::F3, 
            "G3" => Square::G3, 
            "H3" => Square::H3, 
            "A4" => Square::A4, 
            "B4" => Square::B4, 
            "C4" => Square::C4, 
            "D4" => Square::D4, 
            "E4" => Square::E4, 
            "F4" => Square::F4, 
            "G4" => Square::G4, 
            "H4" => Square::H4, 
            "A5" => Square::A5, 
            "B5" => Square::B5, 
            "C5" => Square::C5, 
            "D5" => Square::D5, 
            "E5" => Square::E5, 
            "F5" => Square::F5,
            "G5" => Square::G5, 
            "H5" => Square::H5, 
            "A6" => Square::A6, 
            "B6" => Square::B6, 
            "C6" => Square::C6, 
            "D6" => Square::D6, 
            "E6" => Square::E6, 
            "F6" => Square::F6, 
            "G6" => Square::G6, 
            "H6" => Square::H6, 
            "A7" => Square::A7, 
            "B7" => Square::B7, 
            "C7" => Square::C7, 
            "D7" => Square::D7, 
            "E7" => Square::E7, 
            "F7" => Square::F7, 
            "G7" => Square::G7, 
            "H7" => Square::H7, 
            "A8" => Square::A8, 
            "B8" => Square::B8, 
            "C8" => Square::C8, 
            "D8" => Square::D8, 
            "E8" => Square::E8, 
            "F8" => Square::F8, 
            "G8" => Square::G8, 
            "H8" => Square::H8,
            _ => Square::H8,
        }
    }


    fn convert_role(id:String) -> Role {
        match id.as_str() {
            "P"|"p" => Role::Pawn,
            "N"|"n" => Role::Knight,
            "B"|"b" => Role::Bishop,
            "R"|"r" => Role::Rook,
            "Q"|"q" => Role::Queen,
            "K"|"k" => Role::King,
            _ => Role::Knight,
        }
    }

    pub fn set_end(&mut self) -> String {
        let mut acc = near_sdk::env::predecessor_account_id().clone(); 

        let mut index = 0usize;
        for _item in &self.items {
            if _item.white == acc {
                acc = "".to_string();
                break;
            } else if _item.black == acc {
                acc = "".to_string();
                break;
            }
            index += 1;
        } 

        if acc == "" {
            self.items.remove(index);
            return "GameOver".to_string();
        } 

        return "!Not found!".to_string();

    }

}
