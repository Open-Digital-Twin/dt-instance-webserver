#[macro_use]
#[allow(unused_imports)]
extern crate cdrs;

#[macro_use]
#[allow(unused_imports)]
extern crate cdrs_helpers_derive;

use reqwest::StatusCode;

#[allow(unused_imports)]
use crate::cdrs::query::QueryExecutor;

#[cfg(test)]
mod common;
use common::models::response::{Response, LoginResponse, DataResponse, DataResponseWithTopics, VecDataResponse};
use common::models::twin::{Element, ElementSerialized, ElementRegister, Source, SourceRegister};
use common::models::user::{Register, UserLogin};
use common::db::get_db_session;
use common::requests::{get, post, put};

#[test]
fn get_twin_elements() {
  let session = get_db_session();

  // Create user
  let user_1 = Register {
    email: "example_gettwinelements@example.com".to_string(),
    name: "Example user name".to_string(),
    password: "password123".to_string()
  };
  session.query(format!("DELETE FROM user WHERE email='{}'", user_1.email).to_string()).unwrap();

  let resp_1 = post("user/register").json(&user_1).send().unwrap();
  assert_eq!(resp_1.status(), StatusCode::OK);
  let resp_1_body: Response = resp_1.json().unwrap();
  assert_eq!(resp_1_body.status, true);

  let login = UserLogin {
    email: user_1.email.to_string(),
    password: user_1.password.to_string(),
    remember_me: false
  };

  let resp_login = post("user/login").json(&login).send().unwrap();
  assert_eq!(resp_login.status(), StatusCode::OK);
  let resp_login_json: LoginResponse = resp_login.json().unwrap();
  assert!(resp_login_json.status);

  let token = resp_login_json.token;

  // Create element
  let element_register_1 = ElementRegister {
    name: "Element with Sources".to_string(),
    parent: None
  };

  let resp_2 = put("element").bearer_auth(&token).json(&element_register_1).send().unwrap();
  assert_eq!(resp_2.status(), StatusCode::CREATED);
  let resp_2_body: DataResponse<Element> = resp_2.json().unwrap();
  let element_1 = resp_2_body.data;

  let resp_3 = put("element").bearer_auth(&token).json(&element_register_1).send().unwrap();
  assert_eq!(resp_3.status(), StatusCode::CREATED);
  let resp_3_body: DataResponse<Element> = resp_3.json().unwrap();
  let element_2 = resp_3_body.data;

  let resp_4 = put("element").bearer_auth(&token).json(&element_register_1).send().unwrap();
  assert_eq!(resp_4.status(), StatusCode::CREATED);
  let resp_4_body: DataResponse<Element> = resp_4.json().unwrap();
  let element_3 = resp_4_body.data;

  let resp_5 = put("element").bearer_auth(&token).json(&element_register_1).send().unwrap();
  assert_eq!(resp_5.status(), StatusCode::CREATED);
  let resp_5_body: DataResponse<Element> = resp_5.json().unwrap();
  let element_4 = resp_5_body.data;

  let element_register_2 = ElementRegister {
    name: "Element with Sources".to_string(),
    parent: Some(element_4.id)
  };

  let resp_6 = put("element").bearer_auth(&token).json(&element_register_2).send().unwrap();
  assert_eq!(resp_6.status(), StatusCode::CREATED);
  let resp_6_body: DataResponse<Element> = resp_6.json().unwrap();
  let element_4_child = resp_6_body.data;

  let source_register = SourceRegister {
    name: "Source".to_string(),
    element: element_1.id
  };

  let resp_source_1 = put("source").bearer_auth(&token).json(&source_register).send().unwrap();
  assert_eq!(resp_source_1.status(), StatusCode::CREATED);
  let resp_source_1_body: DataResponseWithTopics<Source> = resp_source_1.json().unwrap();

  let resp_source_2 = put("source").bearer_auth(&token).json(&source_register).send().unwrap();
  assert_eq!(resp_source_2.status(), StatusCode::CREATED);
  let resp_source_2_body: DataResponseWithTopics<Source> = resp_source_2.json().unwrap();

  let resp_source_3 = put("source").bearer_auth(&token).json(&source_register).send().unwrap();
  assert_eq!(resp_source_3.status(), StatusCode::CREATED);
  let resp_source_3_body: DataResponseWithTopics<Source> = resp_source_3.json().unwrap();

  let source_1 = resp_source_1_body.data;
  let source_2 = resp_source_2_body.data;
  let source_3 = resp_source_3_body.data;

  let source_register_2 = SourceRegister {
    name: "Source".to_string(),
    element: element_2.id
  };

  let resp_source_4 = put("source").bearer_auth(&token).json(&source_register_2).send().unwrap();
  assert_eq!(resp_source_4.status(), StatusCode::CREATED);
  let resp_source_4_body: DataResponseWithTopics<Source> = resp_source_4.json().unwrap();

  let resp_source_5 = put("source").bearer_auth(&token).json(&source_register_2).send().unwrap();
  assert_eq!(resp_source_5.status(), StatusCode::CREATED);
  let resp_source_5_body: DataResponseWithTopics<Source> = resp_source_5.json().unwrap();

  let source_4 = resp_source_4_body.data;
  let source_5 = resp_source_5_body.data;

  let source_register_3 = SourceRegister {
    name: "Source".to_string(),
    element: element_4.id
  };

  let source_register_4 = SourceRegister {
    name: "Source".to_string(),
    element: element_4_child.id
  };

  let resp_source_6 = put("source").bearer_auth(&token).json(&source_register_3).send().unwrap();
  assert_eq!(resp_source_6.status(), StatusCode::CREATED);
  let resp_source_6_body: DataResponseWithTopics<Source> = resp_source_6.json().unwrap();

  let resp_source_7 = put("source").bearer_auth(&token).json(&source_register_4).send().unwrap();
  assert_eq!(resp_source_7.status(), StatusCode::CREATED);
  let resp_source_7_body: DataResponseWithTopics<Source> = resp_source_7.json().unwrap();

  let source_6 = resp_source_6_body.data;
  let source_7 = resp_source_7_body.data;

  let element_register_3 = ElementRegister {
    name: "Element with no sources".to_string(),
    parent: None
  };
  
  let resp_8 = put("element").bearer_auth(&token).json(&element_register_3).send().unwrap();
  assert_eq!(resp_8.status(), StatusCode::CREATED);
  let resp_8_body: DataResponse<Element> = resp_8.json().unwrap();
  let element_5 = resp_8_body.data;

  let element_register_3_child = ElementRegister {
    name: "Child Element with no sources".to_string(),
    parent: Some(element_5.id)
  };

  let resp_9 = put("element").bearer_auth(&token).json(&element_register_3_child).send().unwrap();
  assert_eq!(resp_9.status(), StatusCode::CREATED);
  let resp_9_body: DataResponse<Element> = resp_9.json().unwrap();
  let element_5_child = resp_9_body.data;

  let element_register_3_child_2 = ElementRegister {
    name: "Child Element with sources".to_string(),
    parent: Some(element_5_child.id)
  };

  let resp_10 = put("element").bearer_auth(&token).json(&element_register_3_child_2).send().unwrap();
  assert_eq!(resp_10.status(), StatusCode::CREATED);
  let resp_10_body: DataResponse<Element> = resp_10.json().unwrap();
  let resp_11 = put("element").bearer_auth(&token).json(&element_register_3_child_2).send().unwrap();
  assert_eq!(resp_11.status(), StatusCode::CREATED);
  let resp_11_body: DataResponse<Element> = resp_11.json().unwrap();
  let resp_12 = put("element").bearer_auth(&token).json(&element_register_3_child_2).send().unwrap();
  assert_eq!(resp_12.status(), StatusCode::CREATED);
  let resp_12_body: DataResponse<Element> = resp_12.json().unwrap();

  let element_5_cc_1 = resp_10_body.data;
  let element_5_cc_2 = resp_11_body.data;
  let element_5_cc_3 = resp_12_body.data;

  let resp_source_8 = put("source").bearer_auth(&token).json(&SourceRegister {
    name: "Source child".to_string(),
    element: element_5_cc_1.id
  }).send().unwrap();
  assert_eq!(resp_source_8.status(), StatusCode::CREATED);
  let resp_source_8_body: DataResponseWithTopics<Source> = resp_source_8.json().unwrap();

  let resp_source_9 = put("source").bearer_auth(&token).json(&SourceRegister {
    name: "Source child".to_string(),
    element: element_5_cc_2.id
  }).send().unwrap();
  assert_eq!(resp_source_9.status(), StatusCode::CREATED);
  let resp_source_9_body: DataResponseWithTopics<Source> = resp_source_9.json().unwrap();

  let resp_source_10 = put("source").bearer_auth(&token).json(&SourceRegister {
    name: "Source child".to_string(),
    element: element_5_cc_3.id
  }).send().unwrap();
  assert_eq!(resp_source_10.status(), StatusCode::CREATED);
  let resp_source_10_body: DataResponseWithTopics<Source> = resp_source_10.json().unwrap();

  let source_cc_1 = resp_source_8_body.data;
  let source_cc_2 = resp_source_9_body.data;
  let source_cc_3 = resp_source_10_body.data;

  {
    // Get elements of twin
    let resp_elements = get("twin/elements").bearer_auth(&token).send().unwrap();
    assert_eq!(resp_elements.status(), StatusCode::OK);
  
    let resp_elements_body: VecDataResponse<Element> = resp_elements.json().unwrap();
    let obtained_elements = resp_elements_body.data;
  
    assert!(obtained_elements.len() >= 3);
  
    let obtained_element_1 = obtained_elements.iter().find(|&element| element.id == element_1.id).unwrap();
    let obtained_element_2 = obtained_elements.iter().find(|&element| element.id == element_2.id).unwrap();
    let obtained_element_3 = obtained_elements.iter().find(|&element| element.id == element_3.id).unwrap();
  
    assert_eq!(element_1, *obtained_element_1);
    assert_eq!(element_2, *obtained_element_2);
    assert_eq!(element_3, *obtained_element_3);
  }

  {
    // Get serialized elements of twin
    let resp_elements_s = get("twin/elements").query(&[("serialized", true)]).bearer_auth(&token).send().unwrap();
    assert_eq!(resp_elements_s.status(), StatusCode::OK);
  
    let resp_elements_body: VecDataResponse<ElementSerialized> = resp_elements_s.json().unwrap();
    let obtained_elements = resp_elements_body.data;
  
    let s_element_1 = obtained_elements.iter().find(|&item| item.element.id == element_1.id).unwrap();
    let s_element_2 = obtained_elements.iter().find(|&item| item.element.id == element_2.id).unwrap();
    let s_element_3 = obtained_elements.iter().find(|&item| item.element.id == element_3.id).unwrap();
    let s_element_4 = obtained_elements.iter().find(|&item| item.element.id == element_4.id).unwrap();

    assert_eq!(s_element_1.element, element_1);
    assert_eq!(s_element_2.element, element_2);
    assert_eq!(s_element_3.element, element_3);
    assert_eq!(s_element_4.element, element_4);

    // Element 1 has sources 1, 2 and 3.
    let f_source_1 = s_element_1.sources.iter().find(|&source| source.id == source_1.id).unwrap();
    let f_source_2 = s_element_1.sources.iter().find(|&source| source.id == source_2.id).unwrap();
    let f_source_3 = s_element_1.sources.iter().find(|&source| source.id == source_3.id).unwrap();
    assert_eq!(s_element_1.sources.len(), 3);
    assert_eq!(*f_source_1, source_1);
    assert_eq!(*f_source_2, source_2);
    assert_eq!(*f_source_3, source_3);
    assert_eq!(s_element_1.element.id, f_source_1.element);
    assert_eq!(s_element_1.element.id, f_source_2.element);
    assert_eq!(s_element_1.element.id, f_source_3.element);
    
    // Element 2 has sources 4, 5.
    let f_source_4 = s_element_2.sources.iter().find(|&source| source.id == source_4.id).unwrap();
    let f_source_5 = s_element_2.sources.iter().find(|&source| source.id == source_5.id).unwrap();
    assert_eq!(s_element_2.sources.len(), 2);
    assert_eq!(*f_source_4, source_4);
    assert_eq!(*f_source_5, source_5);
    assert_eq!(s_element_2.element.id, f_source_4.element);
    assert_eq!(s_element_2.element.id, f_source_5.element);
    
    // Element 3 has no sources.
    assert_eq!(s_element_3.sources.len(), 0);

    // Element 4 has 1 source and has child element with 1 source
    let f_source_6 = s_element_4.sources.iter().find(|&source| source.id == source_6.id).unwrap();

    assert_eq!(s_element_4.sources.len(), 1);
    assert_eq!(s_element_4.element.id, f_source_6.element);
    assert_eq!(*f_source_6, source_6);
    assert_eq!(s_element_4.children.len(), 1);
    
    let child_element = s_element_4.children.iter().find(|&item| item.element.id == element_4_child.id).unwrap();
    assert_eq!(s_element_4.element.id, child_element.element.parent.unwrap());
    assert_eq!(child_element.sources.len(), 1);
    let f_source_7 = child_element.sources.iter().find(|&source| source.id == source_7.id).unwrap();
    assert_eq!(*f_source_7, source_7);
    assert_eq!(child_element.element.id, f_source_7.element);
    assert_eq!(child_element.children.len(), 0);

    // Element 5 has a child element. Child element has 3 child elements, each with 1 source.
    let s_element_5 = obtained_elements.iter().find(|&item| item.element.id == element_5.id).unwrap();

    assert_eq!(s_element_5.element, element_5);
    assert_eq!(s_element_5.children.len(), 1);
    assert_eq!(s_element_5.sources.len(), 0);

    let s_element_5_child = s_element_5.children.iter().find(|&item| item.element.id == element_5_child.id).unwrap();
    assert_eq!(s_element_5_child.element, element_5_child);
    assert_eq!(s_element_5_child.children.len(), 3);
    assert_eq!(s_element_5_child.sources.len(), 0);

    let s_element_5_cc_1 = s_element_5_child.children.iter().find(|&item| item.element.id == element_5_cc_1.id).unwrap();
    let s_element_5_cc_2 = s_element_5_child.children.iter().find(|&item| item.element.id == element_5_cc_2.id).unwrap();
    let s_element_5_cc_3 = s_element_5_child.children.iter().find(|&item| item.element.id == element_5_cc_3.id).unwrap();

    assert_eq!(s_element_5_cc_1.element, element_5_cc_1);
    assert_eq!(s_element_5_cc_1.children.len(), 0);
    assert_eq!(s_element_5_cc_1.sources.len(), 1);
    assert_eq!(s_element_5_cc_1.sources[0], source_cc_1);

    assert_eq!(s_element_5_cc_2.element, element_5_cc_2);
    assert_eq!(s_element_5_cc_2.children.len(), 0);
    assert_eq!(s_element_5_cc_2.sources.len(), 1);
    assert_eq!(s_element_5_cc_2.sources[0], source_cc_2);

    assert_eq!(s_element_5_cc_3.element, element_5_cc_3);
    assert_eq!(s_element_5_cc_3.children.len(), 0);
    assert_eq!(s_element_5_cc_3.sources.len(), 1);
    assert_eq!(s_element_5_cc_3.sources[0], source_cc_3);
  }
}
