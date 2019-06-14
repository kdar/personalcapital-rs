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
  pub user_guid: Option<String>,
  #[serde(rename = "authLevel")]
  pub auth_level: AuthLevel,
  pub username: Option<String>,
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
  pub original_value: Option<String>,
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
  pub interval_type: Option<String>,
  #[serde(rename = "endDate")]
  pub end_date: String,
  #[serde(rename = "moneyIn")]
  pub money_in: Option<f64>,
  pub transactions: Option<Vec<Transaction>>,
  #[serde(rename = "netCashflow")]
  pub net_cashflow: Option<f64>,
  #[serde(rename = "averageOut")]
  pub average_out: Option<f64>,
  #[serde(rename = "moneyOut")]
  pub money_out: Option<f64>,
  #[serde(rename = "startDate")]
  pub start_date: String,
  #[serde(rename = "averageIn")]
  pub average_in: Option<f64>,
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
  #[serde(rename = "LOCKED")]
  Locked,
  #[serde(rename = "NONE")]
  None,
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

#[derive(Serialize, Deserialize)]
pub struct UserSpending {
  #[serde(rename = "intervals")]
  intervals: Vec<SpendingInterval>,
}

#[derive(Serialize, Deserialize)]
pub struct SpendingInterval {
  #[serde(rename = "average")]
  average: Option<f64>,
  #[serde(rename = "current")]
  current: f64,
  #[serde(rename = "details")]
  details: Vec<SpendingDetail>,
  #[serde(rename = "target")]
  target: f64,
  #[serde(rename = "type")]
  interval_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct SpendingDetail {
  #[serde(rename = "amount")]
  amount: f64,
  #[serde(rename = "date")]
  date: String,
}

#[derive(Serialize, Deserialize)]
pub struct Accounts {
  #[serde(rename = "creditCardAccountsTotal")]
  credit_card_accounts_total: f64,
  assets: f64,
  #[serde(rename = "otherLiabilitiesAccountsTotal")]
  other_liabilities_accounts_total: f64,
  #[serde(rename = "cashAccountsTotal")]
  cash_accounts_total: f64,
  liabilities: f64,
  networth: f64,
  #[serde(rename = "investmentAccountsTotal")]
  investment_accounts_total: f64,
  #[serde(rename = "mortgageAccountsTotal")]
  mortgage_accounts_total: f64,
  #[serde(rename = "loanAccountsTotal")]
  loan_accounts_total: f64,
  accounts: Vec<Account>,
  #[serde(rename = "otherAssetAccountsTotal")]
  other_asset_accounts_total: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Account {
  #[serde(rename = "isOnUs")]
  is_on_us: bool,
  #[serde(rename = "userProductId")]
  user_product_id: i64,
  #[serde(rename = "contactInfo")]
  contact_info: ContactInfo,
  #[serde(rename = "dueDate")]
  due_date: Option<i64>,
  memo: Option<String>,
  #[serde(rename = "isHome")]
  is_home: bool,
  #[serde(rename = "nextAction")]
  next_action: NextAction,
  #[serde(rename = "isIAVEligible")]
  is_iav_eligible: bool,
  #[serde(rename = "loginFields")]
  login_fields: Vec<LoginField>,
  #[serde(rename = "enrollmentConciergeRequested")]
  enrollment_concierge_requested: i64,
  #[serde(rename = "isCrypto")]
  is_crypto: bool,
  #[serde(rename = "isPartner")]
  is_partner: bool,
  #[serde(rename = "isCustomManual")]
  is_custom_manual: bool,
  #[serde(rename = "originalName")]
  original_name: String,
  #[serde(rename = "isIAVAccountNumberValid")]
  is_iav_account_number_valid: bool,
  #[serde(rename = "accountHolder")]
  account_holder: Option<String>,
  #[serde(rename = "isExcludeFromHousehold")]
  is_exclude_from_household: bool,
  #[serde(rename = "isAsset")]
  is_asset: bool,
  aggregating: bool,
  balance: f64,
  #[serde(rename = "isStatementDownloadEligible")]
  is_statement_download_eligible: bool,
  #[serde(rename = "is401KEligible")]
  is401_k_eligible: bool,
  #[serde(rename = "creditLimit")]
  credit_limit: Option<f64>,
  #[serde(rename = "isAccountUsedInFunding")]
  is_account_used_in_funding: bool,
  #[serde(rename = "isOnUs401K")]
  is_on_us401_k: bool,
  #[serde(rename = "advisoryFeePercentage")]
  advisory_fee_percentage: f64,
  #[serde(rename = "lastRefreshed")]
  last_refreshed: i64,
  apr: Option<f64>,
  #[serde(rename = "availableCredit")]
  available_credit: Option<f64>,
  #[serde(rename = "productId")]
  product_id: i64,
  #[serde(rename = "userSiteId")]
  user_site_id: i64,
  #[serde(rename = "is365DayTransactionEligible")]
  is365_day_transaction_eligible: bool,
  #[serde(rename = "isManual")]
  is_manual: bool,
  #[serde(rename = "logoPath")]
  logo_path: String,
  #[serde(rename = "currentBalance")]
  current_balance: f64,
  #[serde(rename = "accountType")]
  account_type: String,
  #[serde(rename = "paymentFromStatus")]
  payment_from_status: bool,
  #[serde(rename = "isRefetchTransactionEligible")]
  is_refetch_transaction_eligible: bool,
  #[serde(rename = "accountId")]
  account_id: String,
  #[serde(rename = "homeUrl")]
  home_url: String,
  #[serde(rename = "isManualPortfolio")]
  is_manual_portfolio: bool,
  #[serde(rename = "excludeFromProposal")]
  exclude_from_proposal: Option<bool>,
  #[serde(rename = "userAccountId")]
  user_account_id: i64,
  name: String,
  #[serde(rename = "firmName")]
  firm_name: String,
  #[serde(rename = "accountTypeGroup")]
  account_type_group: String,
  #[serde(rename = "paymentToStatus")]
  payment_to_status: bool,
  #[serde(rename = "isSelectedForTransfer")]
  is_selected_for_transfer: bool,
  #[serde(rename = "oldestTransactionDate")]
  oldest_transaction_date: Option<String>,
  #[serde(rename = "isOnUsBank")]
  is_on_us_bank: bool,
  #[serde(rename = "accountName")]
  account_name: Option<String>,
  #[serde(rename = "isPaymentToCapable")]
  is_payment_to_capable: bool,
  link: Option<String>,
  #[serde(rename = "closedComment")]
  closed_comment: String,
  #[serde(rename = "loginUrl")]
  login_url: Option<String>,
  #[serde(rename = "mfaType")]
  mfa_type: Option<MfaType>,
  #[serde(rename = "isTaxDeferredOrNonTaxable")]
  is_tax_deferred_or_non_taxable: bool,
  #[serde(rename = "lastPaymentDate")]
  last_payment_date: Option<i64>,
  #[serde(rename = "lastPaymentAmount")]
  last_payment_amount: Option<AmountDue>,
  currency: Currency,
  #[serde(rename = "pcbEnrollmentState")]
  pcb_enrollment_state: String,
  #[serde(rename = "productType")]
  product_type: ProductType,
  #[serde(rename = "isAccountNumberValidated")]
  is_account_number_validated: Option<bool>,
  #[serde(rename = "minPaymentDue")]
  min_payment_due: Option<AmountDue>,
  #[serde(rename = "accountTypeNew")]
  account_type_new: String,
  #[serde(rename = "isLiability")]
  is_liability: bool,
  #[serde(rename = "isTransferEligible")]
  is_transfer_eligible: Option<bool>,
  #[serde(rename = "creditUtilization")]
  credit_utilization: Option<f64>,
  #[serde(rename = "amountDue")]
  amount_due: Option<AmountDue>,
  #[serde(rename = "isEsog")]
  is_esog: bool,
  #[serde(rename = "createdDate")]
  created_date: i64,
  #[serde(rename = "closedDate")]
  closed_date: String,
  #[serde(rename = "isPaymentFromCapable")]
  is_payment_from_capable: bool,
  #[serde(rename = "siteId")]
  site_id: i64,
  #[serde(rename = "originalFirmName")]
  original_firm_name: String,
  #[serde(rename = "runningBalance")]
  running_balance: Option<f64>,
  #[serde(rename = "payoffDate")]
  payoff_date: Option<i64>,
  #[serde(rename = "principalBalance")]
  principal_balance: Option<f64>,
  #[serde(rename = "accruedInterest")]
  accrued_interest: Option<f64>,
  #[serde(rename = "originationDate")]
  origination_date: Option<i64>,
  #[serde(rename = "billingCycle")]
  billing_cycle: Option<String>,
  description: Option<String>,
  lender: Option<String>,
  #[serde(rename = "interestRate")]
  interest_rate: Option<f64>,
  #[serde(rename = "originalLoanAmount")]
  original_loan_amount: Option<f64>,
  #[serde(rename = "interestPaidYtd")]
  interest_paid_ytd: Option<f64>,
  #[serde(rename = "accountProperties")]
  account_properties: Option<Vec<i64>>,
  #[serde(rename = "isTransferPending")]
  is_transfer_pending: Option<bool>,
  #[serde(rename = "treatedAsInvestment")]
  treated_as_investment: Option<bool>,
  #[serde(rename = "availableBalance")]
  available_balance: Option<f64>,
  #[serde(rename = "interestEarnedYtd")]
  interest_earned_ytd: Option<f64>,
  #[serde(rename = "routingNumberSource")]
  routing_number_source: Option<RoutingNumberSource>,
  #[serde(rename = "isRoutingNumberValidated")]
  is_routing_number_validated: Option<bool>,
  #[serde(rename = "routingNumber")]
  routing_number: Option<String>,
  #[serde(rename = "useHomeValuation")]
  use_home_valuation: Option<bool>,
  state: Option<String>,
  addressline: Option<String>,
  longitude: Option<f64>,
  zip: Option<String>,
  #[serde(rename = "userAccountName")]
  user_account_name: Option<String>,
  #[serde(rename = "customProductName")]
  custom_product_name: Option<String>,
  #[serde(rename = "zillowStatus")]
  zillow_status: Option<String>,
  city: Option<String>,
  latitude: Option<f64>,
  #[serde(rename = "propertyType")]
  property_type: Option<String>,
  #[serde(rename = "accountNumber")]
  account_number: Option<String>,
  #[serde(rename = "accountTypeSubtype")]
  account_type_subtype: Option<AccountTypeSubtype>,
  #[serde(rename = "availableCash")]
  available_cash: Option<String>,
  #[serde(rename = "priorBalance")]
  prior_balance: Option<f64>,
  #[serde(rename = "fundFees")]
  fund_fees: Option<f64>,
  #[serde(rename = "defaultAdvisoryFee")]
  default_advisory_fee: Option<f64>,
  #[serde(rename = "isAdvised")]
  is_advised: Option<bool>,
  #[serde(rename = "feesPerYear")]
  fees_per_year: Option<String>,
  #[serde(rename = "totalFee")]
  total_fee: Option<f64>,
  #[serde(rename = "disbursementType")]
  disbursement_type: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ContactInfo {
  url: Option<String>,
  phone: Option<String>,
  email: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginField {
  #[serde(rename = "isUsername")]
  is_username: Option<bool>,
  #[serde(rename = "isRequired")]
  is_required: Option<bool>,
  hint: Option<String>,
  parts: Vec<LoginPart>,
  label: String,
  #[serde(rename = "isPassword")]
  is_password: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginPart {
  size: i64,
  name: String,
  id: Id,
  #[serde(rename = "type")]
  part_type: PartType,
  #[serde(rename = "maxLength")]
  max_length: i64,
  mask: Option<Mask>,
}

#[derive(Serialize, Deserialize)]
pub struct NextAction {
  #[serde(rename = "nextActionMessage")]
  next_action_message: String,
  #[serde(rename = "iconType")]
  icon_type: Action,
  action: Action,
  #[serde(rename = "reportAction")]
  report_action: Action,
  #[serde(rename = "statusMessage")]
  status_message: String,
  prompts: Vec<Option<serde_json::Value>>,
  #[serde(rename = "aggregationErrorType")]
  aggregation_error_type: AggregationErrorType,
}

#[derive(Serialize, Deserialize)]
pub enum AccountTypeSubtype {
  #[serde(rename = "")]
  Empty,
  #[serde(rename = "ROTH")]
  Roth,
  #[serde(rename = "TRADITIONAL")]
  Traditional,
}

#[derive(Serialize, Deserialize)]
pub enum AmountDue {
  NaN,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Currency {
  #[serde(rename = "")]
  Empty,
  #[serde(rename = "USD")]
  Usd,
}

#[derive(Serialize, Deserialize)]
pub enum Id {
  #[serde(rename = "LOGIN")]
  Login,
  #[serde(rename = "OP_LOGIN4")]
  OpLogin4,
  #[serde(rename = "PASSWORD")]
  Password,
}

#[derive(Serialize, Deserialize)]
pub enum Mask {
  #[serde(rename = "LOGIN_FIELD")]
  LoginField,
}

#[derive(Serialize, Deserialize)]
pub enum PartType {
  #[serde(rename = "PASSWORD")]
  Password,
  #[serde(rename = "TEXT")]
  Text,
}

#[derive(Serialize, Deserialize)]
pub enum MfaType {
  #[serde(rename = "SECURITY_QUESTION")]
  SecurityQuestion,
}

#[derive(Serialize, Deserialize)]
pub enum Action {
  #[serde(rename = "NONE")]
  None,
}

#[derive(Serialize, Deserialize)]
pub enum AggregationErrorType {
  #[serde(rename = "NO_ERROR")]
  NoError,
  #[serde(rename = "ZILLOW_ADDRESS_SUCCESS")]
  ZillowAddressSuccess,
}

#[derive(Serialize, Deserialize)]
pub enum ProductType {
  #[serde(rename = "BANK")]
  Bank,
  #[serde(rename = "CREDIT_CARD")]
  CreditCard,
  #[serde(rename = "INVESTMENT")]
  Investment,
  #[serde(rename = "MORTGAGE")]
  Mortgage,
  #[serde(rename = "OTHER_ASSETS")]
  OtherAssets,
}

#[derive(Serialize, Deserialize)]
pub enum RoutingNumberSource {
  #[serde(rename = "YODLEE_AGGREGATION")]
  YodleeAggregation,
}

pub type Categories = Vec<Category>;

#[derive(Serialize, Deserialize)]
pub struct Category {
  #[serde(rename = "isEditable")]
  is_editable: bool,
  name: String,
  #[serde(rename = "isCustom")]
  is_custom: bool,
  #[serde(rename = "isOverride")]
  is_override: bool,
  #[serde(rename = "transactionCategoryId")]
  transaction_category_id: i64,
  #[serde(rename = "shortDescription")]
  short_description: Option<String>,
  #[serde(rename = "type")]
  category_type: CategoryType,
  #[serde(rename = "transactionCategoryKey")]
  transaction_category_key: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum CategoryType {
  #[serde(rename = "EXPENSE")]
  Expense,
  #[serde(rename = "INCOME")]
  Income,
  #[serde(rename = "TRANSFER")]
  Transfer,
  #[serde(rename = "UNCATEGORIZED")]
  Uncategorized,
}
