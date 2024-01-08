use actix_web::web::{Json, Query};
use oasgen::{oasgen, OaSchema, Server};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, OaSchema)]
pub struct SendCode {
    pub mobile: String,
}

#[derive(Serialize, OaSchema)]
pub struct SendCodeResponse {
    pub found_account: bool,
}

#[oasgen]
async fn send_code(_body: Json<SendCode>) -> Json<SendCodeResponse> {
    Json(SendCodeResponse {
        found_account: false,
    })
}

#[derive(Deserialize, OaSchema)]
pub struct GetCode {
    pub code: String,
}

#[derive(Serialize, OaSchema)]
pub struct CodeResponse {
    pub found_code: bool,
}

#[oasgen]
async fn get_code(Query(GetCode { code }): Query<GetCode>) -> Json<CodeResponse> {
    Json(CodeResponse {
        found_code: matches!(&*code, "1234" | "5678"),
    })
}

static expected: &str = r"openapi: 3.0.3
info:
  title: ''
  version: ''
paths:
  /hello:
    post:
      operationId: send_code
      requestBody:
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/SendCode'
        required: true
      responses:
        '200':
          description: ''
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/SendCodeResponse'
  /get-code:
    get:
      operationId: get_code
      responses:
        '200':
          description: ''
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/CodeResponse'
components:
  schemas:
    CodeResponse:
      type: object
      properties:
        found_code:
          type: boolean
      required:
      - found_code
    GetCode:
      type: object
      properties:
        code:
          type: string
      required:
      - code
    SendCode:
      type: object
      properties:
        mobile:
          type: string
      required:
      - mobile
    SendCodeResponse:
      type: object
      properties:
        found_account:
          type: boolean
      required:
      - found_account";

fn main() {
    use pretty_assertions::assert_eq;
    let server = Server::actix()
        .post("/hello", send_code)
        .get("/get-code", get_code);
    let spec = serde_yaml::to_string(&server.openapi).unwrap();
    assert_eq!(spec.trim(), expected);
}
