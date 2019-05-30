use serde_json::value::RawValue;

fn empty_rawvalue() -> Box<RawValue> {
  serde_json::value::RawValue::from_string("null".into()).unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
  #[serde(rename = "spData", default = "empty_rawvalue")]
  pub sp_data: Box<RawValue>,
  #[serde(rename = "spHeader")]
  pub sp_header: SpHeader,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpHeader {
  #[serde(rename = "SP_HEADER_VERSION")]
  pub sp_header_version: i64,
  #[serde(rename = "userStage")]
  pub user_stage: Option<String>,
  #[serde(rename = "betaTester")]
  pub beta_tester: Option<bool>,
  #[serde(rename = "accountsMetaData")]
  pub accounts_meta_data: Option<Vec<String>>,
  pub success: bool,
  #[serde(rename = "accountsSummary")]
  pub accounts_summary: Option<AccountsSummary>,
  #[serde(rename = "qualifiedLead")]
  pub qualified_lead: Option<bool>,
  pub developer: Option<bool>,
  #[serde(rename = "userGuid")]
  pub user_guid: String,
  #[serde(rename = "authLevel")]
  pub auth_level: AuthLevel,
  pub username: String,
  pub status: Status,
  #[serde(rename = "deviceName")]
  pub device_name: Option<String>,
  pub csrf: Option<String>,
  pub errors: Option<Vec<Error>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
  pub code: i64,
  pub details: Option<ErrorDetails>,
  pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorDetails {
  #[serde(rename = "fieldName")]
  pub field_name: Option<String>,
  #[serde(rename = "originalValue")]
  pub original_value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountsSummary {
  #[serde(rename = "hasCredit")]
  pub has_credit: bool,
  #[serde(rename = "hasAggregated")]
  pub has_aggregated: bool,
  #[serde(rename = "hasCash")]
  pub has_cash: bool,
  #[serde(rename = "hasInvestment")]
  pub has_investment: bool,
  #[serde(rename = "hasOnUs")]
  pub has_on_us: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticatePassword {
  #[serde(rename = "allCredentials")]
  pub all_credentials: Vec<Credential>,
  pub credentials: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Credential {
  pub name: String,
  pub status: Status,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserTransactions {
  #[serde(rename = "intervalType")]
  pub interval_type: String,
  #[serde(rename = "endDate")]
  pub end_date: String,
  #[serde(rename = "moneyIn")]
  pub money_in: f64,
  pub transactions: Vec<Transaction>,
  #[serde(rename = "netCashflow")]
  pub net_cashflow: f64,
  #[serde(rename = "averageOut")]
  pub average_out: f64,
  #[serde(rename = "moneyOut")]
  pub money_out: f64,
  #[serde(rename = "startDate")]
  pub start_date: String,
  #[serde(rename = "averageIn")]
  pub average_in: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
  #[serde(rename = "isInterest")]
  pub is_interest: bool,
  #[serde(rename = "netCost")]
  pub net_cost: f64,
  #[serde(rename = "accountName")]
  pub account_name: String,
  pub description: String,
  #[serde(rename = "isCredit")]
  pub is_credit: bool,
  #[serde(rename = "isEditable")]
  pub is_editable: bool,
  #[serde(rename = "isCashOut")]
  pub is_cash_out: bool,
  #[serde(rename = "merchantId")]
  pub merchant_id: String,
  pub price: f64,
  #[serde(rename = "userTransactionId")]
  pub user_transaction_id: i64,
  pub currency: Currency,
  #[serde(rename = "isDuplicate")]
  pub is_duplicate: bool,
  #[serde(rename = "resultType")]
  pub result_type: ResultType,
  #[serde(rename = "originalDescription")]
  pub original_description: String,
  #[serde(rename = "isSpending")]
  pub is_spending: bool,
  pub amount: f64,
  #[serde(rename = "transactionTypeId")]
  pub transaction_type_id: i64,
  #[serde(rename = "isIncome")]
  pub is_income: bool,
  #[serde(rename = "includeInCashManager")]
  pub include_in_cash_manager: bool,
  pub merchant: Option<String>,
  #[serde(rename = "isNew")]
  pub is_new: bool,
  #[serde(rename = "isCashIn")]
  pub is_cash_in: bool,
  #[serde(rename = "transactionDate")]
  pub transaction_date: String,
  #[serde(rename = "transactionType")]
  pub transaction_type: TransactionType,
  #[serde(rename = "accountId")]
  pub account_id: String,
  #[serde(rename = "originalAmount")]
  pub original_amount: f64,
  #[serde(rename = "isCost")]
  pub is_cost: bool,
  #[serde(rename = "userAccountId")]
  pub user_account_id: i64,
  #[serde(rename = "simpleDescription")]
  pub simple_description: Option<String>,
  #[serde(rename = "catKeyword")]
  pub cat_keyword: Option<String>,
  #[serde(rename = "runningBalance")]
  pub running_balance: f64,
  #[serde(rename = "hasViewed")]
  pub has_viewed: bool,
  #[serde(rename = "categoryId")]
  pub category_id: i64,
  pub status: TransactionStatus,
  pub quantity: Option<f64>,
  #[serde(rename = "investmentType")]
  pub investment_type: Option<InvestmentType>,
  pub symbol: Option<String>,
  #[serde(rename = "cusipNumber")]
  pub cusip_number: Option<String>,
  #[serde(rename = "originalCategoryId")]
  pub original_category_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdentifyUser {
  #[serde(rename = "userStatus")]
  pub user_status: Status,
  pub credentials: Vec<String>,
  #[serde(rename = "allCredentials")]
  pub all_credentials: Vec<Credential>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Status {
  #[serde(rename = "ACTIVE")]
  Active,
  #[serde(rename = "INACTIVE")]
  Inactive,
  #[serde(rename = "NONE")]
  None,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Currency {
  #[serde(rename = "USD")]
  Usd,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum InvestmentType {
  Dividend,
  Transfer,
  Buy,
  Sell,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResultType {
  #[serde(rename = "aggregated")]
  Aggregated,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionStatus {
  #[serde(rename = "posted")]
  Posted,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionType {
  Buy,
  #[serde(rename = "Cash In")]
  CashIn,
  Credit,
  Debit,
  #[serde(rename = "Dividend Received")]
  DividendReceived,
  #[serde(rename = "MMF Rein")]
  MmfRein,
  #[serde(rename = "MMF Sweep")]
  MmfSweep,
  #[serde(rename = "Reinvest Dividend")]
  ReinvestDividend,
  Sell,
  Conversion,
  #[serde(rename = "Shares Out")]
  SharesOut,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum AuthLevel {
  Null, // initial state
  Csrf, // fake auth level signifying we got the csrf token

  // Personal Capital auth levels:
  #[serde(rename = "USER_REMEMBERED")]
  UserRemembered,
  #[serde(rename = "USER_IDENTIFIED")]
  UserIdentified,
  #[serde(rename = "DEVICE_AUTHORIZED")]
  DeviceAuthorized,
  #[serde(rename = "SESSION_AUTHENTICATED")]
  SessionAuthenticated,
  #[serde(rename = "NONE")]
  None,
}
