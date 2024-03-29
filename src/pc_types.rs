use std::collections::HashMap;

use chrono::{self, serde::ts_milliseconds_option, DateTime, NaiveDate, Utc};
use serde::Deserialize;
use serde_json::value::RawValue;

use crate::serde_util::{deserialize_f64_option, deserialize_maybe_nan, empty_string_as_none};

fn empty_rawvalue() -> Box<RawValue> {
  serde_json::value::RawValue::from_string("null".into()).unwrap()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
  #[serde(rename = "spData", default = "empty_rawvalue")]
  pub sp_data: Box<RawValue>,
  #[serde(rename = "spHeader")]
  pub sp_header: SpHeader,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SpHeader {
  #[serde(rename = "SP_HEADER_VERSION")]
  pub sp_header_version: i64,
  #[serde(rename = "userStage")]
  pub user_stage: Option<String>,
  #[serde(rename = "isDelegate")]
  pub is_delegate: Option<bool>,
  #[serde(rename = "SP_DATA_CHANGES")]
  pub sp_data_changes: Option<Vec<SpDataChange>>,
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
  #[serde(rename = "personId")]
  pub person_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SpDataChange {
  #[serde(rename = "serverChangeId")]
  pub server_change_id: i64,
  #[serde(rename = "details")]
  pub details: Details,
  #[serde(rename = "eventType")]
  pub event_type: String,
}

pub type Tags = Vec<Tag>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Tag {
  #[serde(rename = "tagId")]
  pub tag_id: i64,

  #[serde(rename = "tagName")]
  pub tag_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Details {
  #[serde(rename = "id")]
  pub id: Option<i64>,
  pub cause: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct QuerySession {
  pub interval: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Error {
  pub code: i64,
  pub details: Option<ErrorDetails>,
  pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ErrorDetails {
  #[serde(rename = "fieldName")]
  pub field_name: Option<String>,
  #[serde(rename = "originalValue")]
  pub original_value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AccountsSummary {
  #[serde(rename = "hasCredit")]
  pub has_credit: Option<bool>,
  #[serde(rename = "hasAggregated")]
  pub has_aggregated: Option<bool>,
  #[serde(rename = "hasCash")]
  pub has_cash: Option<bool>,
  #[serde(rename = "hasInvestment")]
  pub has_investment: Option<bool>,
  #[serde(rename = "hasOnUs")]
  pub has_on_us: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AuthenticatePassword {
  #[serde(rename = "allCredentials")]
  pub all_credentials: Vec<Credential>,
  pub credentials: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Credential {
  pub name: String,
  pub status: Status,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UserTransactions {
  #[serde(rename = "intervalType")]
  pub interval_type: Option<Interval>,
  #[serde(rename = "endDate")]
  pub end_date: chrono::NaiveDate,
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
  pub start_date: chrono::NaiveDate,
  #[serde(rename = "averageIn")]
  pub average_in: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Transaction {
  #[serde(rename = "isInterest")]
  pub is_interest: bool,
  #[serde(rename = "netCost")]
  pub net_cost: Option<f64>,
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
  pub price: Option<f64>,
  #[serde(rename = "userTransactionId")]
  pub user_transaction_id: i64,
  pub currency: Currency,
  #[serde(rename = "isDuplicate")]
  pub is_duplicate: bool,
  #[serde(rename = "resultType")]
  pub result_type: Option<ResultType>,
  #[serde(rename = "originalDescription")]
  pub original_description: String,
  #[serde(rename = "isSpending")]
  pub is_spending: bool,
  pub amount: f64,
  #[serde(rename = "hasSplits")]
  pub has_splits: Option<bool>,
  pub splits: Option<Vec<Split>>,
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
  pub transaction_date: chrono::NaiveDate,
  #[serde(rename = "transactionType")]
  pub transaction_type: TransactionType,
  #[serde(rename = "accountId")]
  pub account_id: String,
  #[serde(rename = "originalAmount")]
  #[serde(deserialize_with = "deserialize_maybe_nan")]
  #[serde(default)]
  pub original_amount: Option<f64>,
  #[serde(rename = "isCost")]
  pub is_cost: bool,
  #[serde(rename = "userAccountId")]
  pub user_account_id: i64,
  #[serde(rename = "simpleDescription")]
  pub simple_description: Option<String>,
  #[serde(rename = "catKeyword")]
  pub cat_keyword: Option<String>,
  #[serde(rename = "runningBalance")]
  pub running_balance: Option<f64>,
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
  #[serde(rename = "customTags")]
  pub custom_tags: Option<CustomTags>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Split {
  pub amount: f64,
  #[serde(rename = "customTags")]
  pub custom_tags: Option<CustomTags>,
  #[serde(rename = "userTransactionId")]
  pub user_transaction_id: String,
  #[serde(rename = "categoryId")]
  pub category_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CustomTags {
  #[serde(rename = "systemTags")]
  pub system_tags: Vec<i64>,

  #[serde(rename = "userTags")]
  pub user_tags: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IdentifyUser {
  #[serde(rename = "userStatus")]
  pub user_status: Status,
  pub credentials: Vec<String>,
  #[serde(rename = "allCredentials")]
  pub all_credentials: Vec<Credential>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum InvestmentType {
  Dividend,
  Transfer,
  Buy,
  Sell,
  #[serde(rename = "Mgmt Fees")]
  MgmtFees,
  Interest,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum ResultType {
  #[serde(rename = "aggregated")]
  Aggregated,
  #[serde(rename = "")]
  None,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum TransactionStatus {
  #[serde(rename = "posted")]
  Posted,
  #[serde(rename = "pending")]
  Pending,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum TransactionType {
  Buy,
  #[serde(rename = "Cash In")]
  CashIn,
  #[serde(rename = "Cash Out")]
  CashOut,
  Credit,
  Debit,
  #[serde(rename = "Dividend Received")]
  DividendReceived,
  #[serde(rename = "Account Fee")]
  AccountFee,
  #[serde(rename = "MMF Rein")]
  MmfRein,
  #[serde(rename = "MMF Sweep")]
  MmfSweep,
  #[serde(rename = "Reinvest Dividend")]
  ReinvestDividend,
  Sell,
  Conversion,
  #[serde(rename = "Shares In")]
  SharesIn,
  #[serde(rename = "Shares Out")]
  SharesOut,
  #[serde(rename = "Contribution")]
  Contribution,
  #[serde(rename = "401k Contribution")]
  Contribution401k,
  #[serde(rename = "RollOver Contribution")]
  ContributionRollOver,
  #[serde(rename = "Adjustment")]
  Adjustment,
  #[serde(rename = "Misc Exp")]
  MiscExp,
  #[serde(rename = "ACH Out")]
  ACHOut,
  #[serde(rename = "Sweep")]
  Sweep,
  #[serde(rename = "Interest Income")]
  InterestIncome,
  #[serde(rename = "RolloverToQual")]
  RolloverToQual,
  #[serde(rename = "ST CG Dist")]
  STCGDist,
  #[serde(rename = "LT CG Dist")]
  LTCGDist,
  #[serde(rename = "Unknown")]
  Unknown,
  #[serde(rename = "Roth Contribution")]
  RothContribution,
  #[serde(rename = "Interest ReInvestment")]
  InterestReInvestment,
  #[serde(rename = "Return of Capital")]
  ReturnOfCapital,
  #[serde(rename = "Recharacterization")]
  Recharacterization,
  #[serde(rename = "Fund Exchange")]
  FundExchange,
  #[serde(rename = "Administrative Fee")]
  AdministrativeFee,
  #[serde(rename = "Direct Deposit")]
  DirectDeposit,
  #[serde(rename = "Refund")]
  Refund,
  #[serde(rename = "Payment")]
  Payment,
  #[serde(rename = "Transfer")]
  Transfer,
  #[serde(rename = "Deposit Credits")]
  DepositCredits,
  #[serde(rename = "Interest")]
  Interest,
  #[serde(rename = "Employee contribution")]
  EmployeeContribution,
  #[serde(rename = "Interest charge")]
  InterestCharge,
  #[serde(rename = "Deposit")]
  Deposit,
  #[serde(rename = "Fee")]
  Fee,
  #[serde(rename = "Withdrawal")]
  Withdrawal,
  #[serde(rename = "Charges Fees")]
  ChargesFees,
  #[serde(rename = "Misc Income")]
  MiscIncome,
  #[serde(rename = "!REVERSAL.REVERSAL!")]
  Reversal,
  #[serde(rename = "Purchase")]
  Purchase,
  #[serde(rename = "Credit Adjustment")]
  CreditAdjustment,
  #[serde(rename = "Debit Adjustment")]
  DebitAdjustment,
  #[serde(rename = "Other")]
  Other,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserSpending {
  #[serde(rename = "intervals")]
  pub intervals: Vec<SpendingInterval>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SpendingInterval {
  #[serde(rename = "average")]
  pub average: Option<f64>,
  #[serde(rename = "current")]
  pub current: f64,
  #[serde(rename = "details")]
  pub details: Vec<SpendingDetail>,
  #[serde(rename = "target")]
  pub target: f64,
  #[serde(rename = "type")]
  pub interval_type: Interval,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Interval {
  #[serde(rename = "YEAR")]
  Year,
  #[serde(rename = "MONTH")]
  Month,
  #[serde(rename = "WEEK")]
  Week,
  #[serde(rename = "DAY")]
  Day,
}

impl AsRef<str> for Interval {
  fn as_ref(&self) -> &'static str {
    match self {
      Self::Year => "YEAR",
      Self::Month => "MONTH",
      Self::Week => "WEEK",
      Self::Day => "DAY",
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SpendingDetail {
  #[serde(rename = "amount")]
  pub amount: f64,
  #[serde(rename = "date")]
  pub date: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Accounts {
  #[serde(rename = "creditCardAccountsTotal")]
  pub credit_card_accounts_total: f64,
  pub assets: f64,
  #[serde(rename = "otherLiabilitiesAccountsTotal")]
  pub other_liabilities_accounts_total: f64,
  #[serde(rename = "cashAccountsTotal")]
  pub cash_accounts_total: f64,
  pub liabilities: f64,
  pub networth: f64,
  #[serde(rename = "investmentAccountsTotal")]
  pub investment_accounts_total: f64,
  #[serde(rename = "mortgageAccountsTotal")]
  pub mortgage_accounts_total: f64,
  #[serde(rename = "loanAccountsTotal")]
  pub loan_accounts_total: f64,
  pub accounts: Vec<Account>,
  #[serde(rename = "otherAssetAccountsTotal")]
  pub other_asset_accounts_total: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Account {
  #[serde(rename = "isOnUs")]
  pub is_on_us: bool,
  #[serde(rename = "userProductId")]
  pub user_product_id: Option<i64>,
  #[serde(rename = "contactInfo")]
  pub contact_info: Option<ContactInfo>,
  #[serde(rename = "dueDate")]
  pub due_date: Option<i64>,
  pub memo: Option<String>,
  #[serde(rename = "isHome")]
  pub is_home: Option<bool>,
  #[serde(rename = "nextAction")]
  pub next_action: NextAction,
  #[serde(rename = "isIAVEligible")]
  pub is_iav_eligible: Option<bool>,
  #[serde(rename = "loginFields")]
  pub login_fields: Option<Vec<Option<LoginField>>>,
  #[serde(rename = "enrollmentConciergeRequested")]
  pub enrollment_concierge_requested: Option<i64>,
  #[serde(rename = "isCrypto")]
  pub is_crypto: bool,
  #[serde(rename = "isPartner")]
  pub is_partner: bool,
  #[serde(rename = "isCustomManual")]
  pub is_custom_manual: Option<bool>,
  #[serde(rename = "originalName")]
  pub original_name: Option<String>,
  #[serde(rename = "isIAVAccountNumberValid")]
  pub is_iav_account_number_valid: Option<bool>,
  #[serde(rename = "accountHolder")]
  pub account_holder: Option<String>,
  #[serde(rename = "isExcludeFromHousehold")]
  pub is_exclude_from_household: bool,
  #[serde(rename = "isAsset")]
  pub is_asset: bool,
  pub aggregating: bool,
  pub balance: Option<f64>,
  #[serde(rename = "isStatementDownloadEligible")]
  pub is_statement_download_eligible: Option<bool>,
  #[serde(rename = "is401KEligible")]
  pub is401_k_eligible: Option<bool>,
  #[serde(
    rename = "creditLimit",
    default,
    deserialize_with = "deserialize_f64_option"
  )]
  pub credit_limit: Option<f64>,
  #[serde(rename = "isAccountUsedInFunding")]
  pub is_account_used_in_funding: bool,
  #[serde(rename = "isOnUs401K")]
  pub is_on_us401_k: bool,
  #[serde(rename = "advisoryFeePercentage")]
  pub advisory_fee_percentage: Option<f64>,
  #[serde(rename = "lastRefreshed", with = "ts_milliseconds_option", default)]
  pub last_refreshed: Option<DateTime<Utc>>,
  pub apr: Option<f64>,
  #[serde(rename = "availableCredit")]
  pub available_credit: Option<f64>,
  #[serde(rename = "productId")]
  pub product_id: Option<i64>,
  #[serde(rename = "userSiteId")]
  pub user_site_id: i64,
  #[serde(rename = "is365DayTransactionEligible")]
  pub is365_day_transaction_eligible: bool,
  #[serde(rename = "isManual")]
  pub is_manual: bool,
  #[serde(rename = "logoPath")]
  pub logo_path: Option<String>,
  #[serde(rename = "currentBalance")]
  pub current_balance: Option<f64>,
  #[serde(rename = "accountType", default)]
  pub account_type: AccountType,
  #[serde(rename = "paymentFromStatus")]
  pub payment_from_status: bool,
  #[serde(rename = "isRefetchTransactionEligible")]
  pub is_refetch_transaction_eligible: bool,
  #[serde(rename = "accountId")]
  pub account_id: String,
  #[serde(rename = "homeUrl")]
  pub home_url: Option<String>,
  #[serde(rename = "isManualPortfolio")]
  pub is_manual_portfolio: bool,
  #[serde(rename = "excludeFromProposal")]
  pub exclude_from_proposal: Option<bool>,
  #[serde(rename = "userAccountId")]
  pub user_account_id: Option<i64>,
  pub name: Option<String>,
  #[serde(rename = "firmName")]
  pub firm_name: String,
  #[serde(rename = "accountTypeGroup", default)]
  pub account_type_group: AccountTypeGroup,
  #[serde(rename = "paymentToStatus")]
  pub payment_to_status: bool,
  #[serde(rename = "isSelectedForTransfer")]
  pub is_selected_for_transfer: Option<bool>,
  #[serde(rename = "oldestTransactionDate")]
  pub oldest_transaction_date: Option<chrono::NaiveDate>,
  #[serde(rename = "isOnUsBank")]
  pub is_on_us_bank: bool,
  #[serde(rename = "accountName")]
  pub account_name: Option<String>,
  #[serde(rename = "isPaymentToCapable")]
  pub is_payment_to_capable: bool,
  pub link: Option<String>,
  #[serde(rename = "closedComment")]
  pub closed_comment: Option<String>,
  #[serde(rename = "loginUrl")]
  pub login_url: Option<String>,
  #[serde(rename = "mfaType")]
  pub mfa_type: Option<MfaType>,
  #[serde(rename = "isTaxDeferredOrNonTaxable")]
  pub is_tax_deferred_or_non_taxable: Option<bool>,
  #[serde(rename = "lastPaymentDate", with = "ts_milliseconds_option", default)]
  pub last_payment_date: Option<DateTime<Utc>>,
  #[serde(rename = "lastPaymentAmount")]
  pub last_payment_amount: Option<AmountDue>,
  pub currency: Option<Currency>,
  #[serde(rename = "pcbEnrollmentState")]
  pub pcb_enrollment_state: Option<String>,
  #[serde(rename = "productType")]
  pub product_type: Option<ProductType>,
  #[serde(rename = "isAccountNumberValidated")]
  pub is_account_number_validated: Option<bool>,
  #[serde(rename = "minPaymentDue")]
  pub min_payment_due: Option<AmountDue>,
  #[serde(rename = "accountTypeNew", default)]
  pub account_type_new: AccountTypeNew,
  #[serde(rename = "isLiability")]
  pub is_liability: bool,
  #[serde(rename = "isTransferEligible")]
  pub is_transfer_eligible: Option<bool>,
  #[serde(rename = "creditUtilization")]
  pub credit_utilization: Option<f64>,
  #[serde(rename = "amountDue")]
  pub amount_due: Option<AmountDue>,
  #[serde(rename = "isEsog")]
  pub is_esog: bool,
  #[serde(rename = "createdDate", with = "ts_milliseconds_option", default)]
  pub created_date: Option<DateTime<Utc>>,
  #[serde(
    rename = "closedDate",
    deserialize_with = "empty_string_as_none",
    default
  )]
  pub closed_date: Option<NaiveDate>,
  #[serde(rename = "isPaymentFromCapable")]
  pub is_payment_from_capable: bool,
  #[serde(rename = "siteId")]
  pub site_id: i64,
  #[serde(rename = "originalFirmName")]
  pub original_firm_name: String,
  #[serde(rename = "runningBalance")]
  pub running_balance: Option<f64>,
  #[serde(rename = "payoffDate", with = "ts_milliseconds_option", default)]
  pub payoff_date: Option<DateTime<Utc>>,
  #[serde(rename = "principalBalance")]
  pub principal_balance: Option<f64>,
  #[serde(rename = "accruedInterest")]
  pub accrued_interest: Option<f64>,
  #[serde(rename = "originationDate", with = "ts_milliseconds_option", default)]
  pub origination_date: Option<DateTime<Utc>>,
  #[serde(rename = "billingCycle")]
  pub billing_cycle: Option<String>,
  pub description: Option<String>,
  pub lender: Option<String>,
  #[serde(rename = "interestRate")]
  pub interest_rate: Option<f64>,
  #[serde(rename = "originalLoanAmount")]
  pub original_loan_amount: Option<f64>,
  #[serde(rename = "interestPaidYtd")]
  pub interest_paid_ytd: Option<f64>,
  #[serde(rename = "accountProperties")]
  pub account_properties: Option<Vec<i64>>,
  #[serde(rename = "isTransferPending")]
  pub is_transfer_pending: Option<bool>,
  #[serde(rename = "treatedAsInvestment")]
  pub treated_as_investment: Option<bool>,
  #[serde(rename = "availableBalance")]
  pub available_balance: Option<f64>,
  #[serde(rename = "interestEarnedYtd")]
  pub interest_earned_ytd: Option<f64>,
  #[serde(rename = "routingNumberSource")]
  pub routing_number_source: Option<RoutingNumberSource>,
  #[serde(rename = "isRoutingNumberValidated")]
  pub is_routing_number_validated: Option<bool>,
  #[serde(rename = "routingNumber")]
  pub routing_number: Option<String>,
  #[serde(rename = "useHomeValuation")]
  pub use_home_valuation: Option<bool>,
  pub state: Option<String>,
  pub addressline: Option<String>,
  pub longitude: Option<f64>,
  pub zip: Option<String>,
  #[serde(rename = "userAccountName")]
  pub user_account_name: Option<String>,
  #[serde(rename = "customProductName")]
  pub custom_product_name: Option<String>,
  #[serde(rename = "zillowStatus")]
  pub zillow_status: Option<String>,
  pub city: Option<String>,
  pub latitude: Option<f64>,
  #[serde(rename = "propertyType")]
  pub property_type: Option<String>,
  #[serde(rename = "accountNumber")]
  pub account_number: Option<String>,
  #[serde(rename = "accountTypeSubtype", default)]
  pub account_type_subtype: AccountTypeSubtype,
  #[serde(rename = "availableCash")]
  pub available_cash: Option<String>,
  #[serde(rename = "priorBalance")]
  pub prior_balance: Option<f64>,
  #[serde(rename = "fundFees")]
  pub fund_fees: Option<f64>,
  #[serde(rename = "defaultAdvisoryFee")]
  pub default_advisory_fee: Option<f64>,
  #[serde(rename = "isAdvised")]
  pub is_advised: Option<bool>,
  #[serde(rename = "feesPerYear")]
  pub fees_per_year: Option<String>,
  #[serde(rename = "totalFee")]
  pub total_fee: Option<f64>,
  #[serde(rename = "disbursementType")]
  pub disbursement_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ContactInfo {
  pub url: Option<String>,
  pub phone: Option<String>,
  pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LoginField {
  #[serde(rename = "isUsername")]
  pub is_username: Option<bool>,
  #[serde(rename = "isRequired")]
  pub is_required: Option<bool>,
  pub hint: Option<String>,
  pub parts: Vec<LoginPart>,
  pub label: String,
  #[serde(rename = "isPassword")]
  pub is_password: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LoginPart {
  pub size: Option<i64>,
  pub name: Option<String>,
  pub id: Id,
  #[serde(rename = "type")]
  pub part_type: PartType,
  #[serde(rename = "maxLength")]
  pub max_length: Option<i64>,
  pub mask: Option<Mask>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct NextAction {
  #[serde(rename = "nextActionMessage")]
  pub next_action_message: Option<String>,
  #[serde(rename = "iconType")]
  pub icon_type: Option<Action>,
  pub action: Option<Action>,
  #[serde(rename = "reportAction")]
  pub report_action: Option<Action>,
  #[serde(rename = "statusMessage")]
  pub status_message: Option<String>,
  pub prompts: Vec<Option<serde_json::Value>>,
  #[serde(rename = "aggregationErrorType")]
  pub aggregation_error_type: Option<AggregationErrorType>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum AccountTypeNew {
  #[serde(rename = "")]
  None,
  #[serde(rename = "INVESTMENT")]
  Investment,
  #[serde(rename = "IRA")]
  IRA,
  #[serde(rename = "401K")]
  Retirement401k,
  #[serde(rename = "529")]
  Educational529,
  #[serde(rename = "PERSONAL")]
  Personal,
  #[serde(rename = "MORTGAGE")]
  Mortgage,
  #[serde(rename = "MMA")]
  MoneyMarket,
  #[serde(rename = "CHECKING")]
  Checking,
  #[serde(rename = "SAVINGS")]
  Savings,
  #[serde(rename = "ESPP")]
  ESPP,
  #[serde(rename = "ESOP")]
  ESOP,
  #[serde(rename = "CRYPTO_CURRENCY")]
  CryptoCurrency,
}

impl Default for AccountTypeNew {
  fn default() -> Self {
    Self::None
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum AccountTypeSubtype {
  #[serde(rename = "")]
  None,
  #[serde(rename = "ROTH")]
  Roth,
  #[serde(rename = "TRADITIONAL")]
  Traditional,
}

impl Default for AccountTypeSubtype {
  fn default() -> Self {
    Self::None
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum AccountType {
  #[serde(rename = "")]
  None,
  #[serde(rename = "Checking")]
  Checking,
  #[serde(rename = "Cash")]
  Cash,
  #[serde(rename = "Credit")]
  Credit,
  #[serde(rename = "IRA - Traditional")]
  TraditionalIra,
  #[serde(rename = "IRA - Roth")]
  RothIra,
  #[serde(rename = "Savings")]
  Savings,
  #[serde(rename = "401K")]
  Traditional401k,
  #[serde(rename = "Roth 401k")]
  Roth401k,
  #[serde(rename = "Investment")]
  Investment,
  #[serde(rename = "Individual Account")]
  IndividualAccount,
  #[serde(rename = "Joint Account")]
  JointAccount,
  #[serde(rename = "Brokerage")]
  Brokerage,
  #[serde(rename = "Mortgage", alias = "MORTGAGE")]
  Mortgage,
  #[serde(rename = "529")]
  Educational529,
  #[serde(rename = "Money Market")]
  MoneyMarket,
  #[serde(rename = "ESPP")]
  ESPP,
  #[serde(rename = "ESOP")]
  ESOP,
  #[serde(rename = "401K, Former Employer")]
  Traditional401kFormerEmployer,
  #[serde(rename = "Personal")]
  Personal,
  #[serde(rename = "Assets")]
  Assets,
  #[serde(rename = "Loan")]
  Loan,
  #[serde(rename = "Crypto Currency")]
  CryptoCurrency,
}

impl Default for AccountType {
  fn default() -> Self {
    Self::None
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum AccountTypeGroup {
  #[serde(rename = "")]
  None,
  #[serde(rename = "BANK")]
  Bank,
  #[serde(rename = "CREDIT_CARD")]
  CreditCard,
  #[serde(rename = "RETIREMENT")]
  Retirement,
  #[serde(rename = "INVESTMENT")]
  Investment,
  #[serde(rename = "MORTGAGE")]
  Mortgage,
  #[serde(rename = "EDUCATIONAL")]
  Educational,
  #[serde(rename = "ESOP")]
  ESOP,
  #[serde(rename = "ESPP")]
  ESPP,
  #[serde(rename = "CRYPTO_CURRENCY")]
  CryptoCurrency,
}

impl Default for AccountTypeGroup {
  fn default() -> Self {
    Self::None
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum AmountDue {
  NaN,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Currency {
  #[serde(rename = "")]
  Empty,
  #[serde(rename = "USD")]
  Usd,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Id {
  #[serde(rename = "LOGIN")]
  Login,
  #[serde(rename = "OP_LOGIN")]
  OpLogin,
  #[serde(rename = "OP_LOGIN1")]
  OpLogin1,
  #[serde(rename = "OP_LOGIN4")]
  OpLogin4,
  #[serde(rename = "PASSWORD")]
  Password,
  #[serde(rename = "OP_PASSWORD1")]
  OpPassword1,
  #[serde(rename = "OPTION")]
  Option,
  #[serde(rename = "OP_OPTION")]
  OpOption,
  #[serde(rename = "PASSWORD1")]
  Password1,
  #[serde(rename = "0001Choice")]
  Choice0001,
  #[serde(rename = "LOGIN1")]
  Login1,
  #[serde(rename = "OP_PASSWORD")]
  OpPassword,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Mask {
  #[serde(rename = "LOGIN_FIELD")]
  LoginField,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum PartType {
  #[serde(rename = "PASSWORD")]
  Password,
  #[serde(rename = "TEXT")]
  Text,
  #[serde(rename = "OPTIONS")]
  Options,
  #[serde(rename = "RADIO")]
  Radio,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum MfaType {
  #[serde(rename = "SECURITY_QUESTION")]
  SecurityQuestion,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Action {
  #[serde(rename = "NONE")]
  None,
  #[serde(rename = "AGGREGATING")]
  Aggregatng,
  #[serde(rename = "WAIT")]
  Wait,
  #[serde(rename = "WARNING")]
  Warning,
  #[serde(rename = "ERROR")]
  Error,
  #[serde(rename = "INITIATE_REFRESH")]
  InitiateRefresh,
  #[serde(rename = "VISIT_SITE")]
  VisitSite,
  #[serde(rename = "INFO")]
  Info,
  #[serde(rename = "CLOSE_ACCOUNT")]
  CloseAccount,
  #[serde(rename = "BLOCKED")]
  Blocked,
  #[serde(rename = "MORE_INFO")]
  MoreInfo,
  #[serde(rename = "PCB_FINISH_SETUP")]
  PcbFinishSetup,
  #[serde(rename = "PCB_FUND_NOW")]
  PcbFundNow,
  #[serde(rename = "BLACKOUT")]
  Blackout,
  #[serde(rename = "MIGRATE_OAUTH")]
  MigrateOauth,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum AggregationErrorType {
  #[serde(rename = "NO_ERROR")]
  NoError,
  #[serde(rename = "ZILLOW_ADDRESS_INTERNAL_ERROR")]
  ZillowAddressInternalError,
  #[serde(rename = "ZILLOW_ADDRESS_SUCCESS")]
  ZillowAddressSuccess,
  #[serde(rename = "ACCOUNT_NOT_FOUND")]
  AccountNotFound,
  #[serde(rename = "MFA_REQUIRED")]
  MfaRequired,
  #[serde(rename = "VISIT_SITE_REQUIRED")]
  VisitSiteRequired,
  #[serde(rename = "SITE_ERROR")]
  SiteError,
  #[serde(rename = "AGENT_ERROR")]
  AgentError,
  #[serde(rename = "MFA_TIMEDOUT")]
  MfaTimedout,
  #[serde(rename = "PASSWORD_OR_QUESTIONS_INCORRECT")]
  PasswordOrQuestionsIncorrect,
  #[serde(rename = "AGGREGATION_NEVER_DONE")]
  AggregationNeverDone,
  #[serde(rename = "LOCKED_OUT")]
  LockedOut,
  #[serde(rename = "CATCH_ALL")]
  CatchAll,
  #[serde(rename = "REGISTRATION_FAILED")]
  RegistrationFailed,
  #[serde(rename = "OAUTH_CONNECTION_LINK_FAILURE")]
  OauthConnectionLinkFailure,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum ProductType {
  #[serde(rename = "BANK")]
  Bank,
  #[serde(rename = "CREDIT_CARD")]
  CreditCard,
  #[serde(rename = "INVESTMENT")]
  Investment,
  #[serde(rename = "MORTGAGE")]
  Mortgage,
  #[serde(rename = "LOAN")]
  Loan,
  #[serde(rename = "OTHER_ASSETS")]
  OtherAssets,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum RoutingNumberSource {
  #[serde(rename = "YODLEE_AGGREGATION")]
  YodleeAggregation,
  #[serde(rename = "ENROLLMENT")]
  Enrollment,
}

pub type Categories = Vec<Category>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Category {
  #[serde(rename = "isEditable")]
  pub is_editable: bool,
  pub name: String,
  #[serde(rename = "isCustom")]
  pub is_custom: bool,
  #[serde(rename = "isOverride")]
  pub is_override: bool,
  #[serde(rename = "transactionCategoryId")]
  pub transaction_category_id: i64,
  #[serde(rename = "shortDescription")]
  pub short_description: Option<String>,
  #[serde(rename = "type")]
  pub category_type: CategoryType,
  #[serde(rename = "transactionCategoryKey")]
  pub transaction_category_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum CategoryType {
  #[serde(rename = "EXPENSE")]
  Expense,
  #[serde(rename = "INCOME")]
  Income,
  #[serde(rename = "TRANSFER")]
  Transfer,
  #[serde(rename = "UNCATEGORIZED")]
  Uncategorized,
  #[serde(rename = "DEFERRED_COMPENSATION")]
  DeferredCompensation,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Holdings {
  #[serde(rename = "classifications")]
  pub classifications: Vec<Option<serde_json::Value>>,
  #[serde(rename = "holdings")]
  pub holdings: Vec<Holding>,
  #[serde(rename = "holdingsTotalValue")]
  pub holdings_total_value: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Holding {
  #[serde(rename = "quantity")]
  pub quantity: f64,
  #[serde(rename = "manualClassification")]
  pub manual_classification: ManualClassification,
  #[serde(rename = "isMarketMover")]
  pub is_market_mover: bool,
  #[serde(rename = "oneDayPercentChangeSortIndex")]
  pub one_day_percent_change_sort_index: i64,
  #[serde(rename = "oneDayValueChange")]
  pub one_day_value_change: f64,
  #[serde(rename = "change")]
  pub change: f64,
  #[serde(rename = "description")]
  pub description: Option<String>,
  #[serde(rename = "source")]
  pub source: HoldingSource,
  #[serde(rename = "changeSortIndex")]
  pub change_sort_index: i64,
  #[serde(rename = "oneDayValueChangeSortIndex")]
  pub one_day_value_change_sort_index: i64,
  #[serde(rename = "marketType")]
  pub market_type: i64,
  #[serde(rename = "sourceAssetId")]
  pub source_asset_id: String,
  #[serde(rename = "external")]
  pub external: Option<String>,
  #[serde(rename = "holdingType")]
  pub holding_type: HoldingType,
  #[serde(rename = "price")]
  pub price: f64,
  #[serde(rename = "holdingPercentage")]
  pub holding_percentage: f64,
  #[serde(rename = "userAccountId")]
  pub user_account_id: i64,
  #[serde(rename = "priceSource")]
  pub price_source: PriceSource,
  #[serde(rename = "valueSortIndex")]
  pub value_sort_index: i64,
  #[serde(rename = "currency")]
  pub currency: Option<String>,
  #[serde(rename = "value")]
  pub value: f64,
  #[serde(rename = "oneDayPercentChange")]
  pub one_day_percent_change: f64,
  #[serde(rename = "originalDescription")]
  pub original_description: Option<String>,
  #[serde(rename = "cusip")]
  pub cusip: Option<String>,
  #[serde(rename = "accountName")]
  pub account_name: Option<String>,
  #[serde(rename = "originalTicker")]
  pub original_ticker: Option<String>,
  #[serde(rename = "originalCusip")]
  pub original_cusip: Option<String>,
  #[serde(rename = "ticker")]
  pub ticker: Option<String>,
  #[serde(rename = "exchange")]
  pub exchange: Option<Exchange>,
  #[serde(rename = "tradingRatio")]
  pub trading_ratio: Option<f64>,
  #[serde(rename = "type")]
  pub fund_type: Option<FundType>,
  #[serde(rename = "taxCost")]
  pub tax_cost: Option<f64>,
  #[serde(rename = "fundFees")]
  pub fund_fees: Option<f64>,
  #[serde(rename = "feesPerYear")]
  pub fees_per_year: Option<f64>,
  #[serde(rename = "costBasis")]
  pub cost_basis: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Exchange {
  #[serde(rename = "NASDAQ")]
  Nasdaq,
  #[serde(rename = "NYSE")]
  Nyse,
  #[serde(rename = "NYSE Arca")]
  NyseArca,
  #[serde(rename = "NYSE American")]
  NyseAmerican,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum HoldingType {
  #[serde(rename = "Cash")]
  Cash,
  #[serde(rename = "ETF")]
  Etf,
  #[serde(rename = "Fund")]
  Fund,
  #[serde(rename = "Other")]
  Other,
  #[serde(rename = "Stock")]
  Stock,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum ManualClassification {
  #[serde(rename = "RESTRICTED")]
  Restricted,
  #[serde(rename = "UNCLASSIFIED")]
  Unclassified,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum PriceSource {
  #[serde(rename = "MARKET")]
  Market,
  #[serde(rename = "PARTNER")]
  Partner,
  #[serde(rename = "USER")]
  User,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum FundType {
  #[serde(rename = "ETF")]
  Etf,
  #[serde(rename = "Mutual Fund")]
  MutualFund,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum HoldingSource {
  #[serde(rename = "PCAP")]
  Pcap,
  #[serde(rename = "USER")]
  User,
  #[serde(rename = "YODLEE")]
  Yodlee,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum HistoryBalance {
  Float(f64),
  String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum HistoryType {
  Balances,
  Networth,
  DailyChangeAmount,
  OneDaySummaries,
  CashFlows,
}

// impl From<&'static str> for HistoryType {
//   fn from(s: &'static str) -> Self {
//     match s {
//       "balances" => Self::Balances,
//       "networth" => Self::Networth,
//       "dailychangeamount" => Self::DailyChangeAmount,
//       "oneDaySummaries" => Self::OneDaySummaries,
//       "cashflows" => Self::CashFlows,
//     }
//   }
// }

impl AsRef<str> for HistoryType {
  fn as_ref(&self) -> &'static str {
    match self {
      Self::Balances => "balances",
      Self::Networth => "networth",
      Self::DailyChangeAmount => "dailychangeamount",
      Self::OneDaySummaries => "oneDaySummaries",
      Self::CashFlows => "cashflows",
    }
  }
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Histories {
  #[serde(rename = "networthSummary")]
  pub networth_summary: Option<HashMap<String, i64>>,
  #[serde(rename = "oneDaySummaries")]
  pub one_day_summaries: Option<OneDaySummaries>,
  #[serde(rename = "intervalType")]
  pub interval_type: Option<Interval>,
  pub histories: Option<Vec<History>>,
  #[serde(rename = "accountSummaries")]
  pub account_summaries: Option<Vec<AccountSummary>>,
  #[serde(rename = "networthHistories")]
  pub networth_histories: Option<Vec<NetworthHistory>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccountSummary {
  pub income: f64,
  #[serde(rename = "dateRangeBalanceValueChange")]
  pub date_range_balance_value_change: f64,
  #[serde(rename = "accountName")]
  pub account_name: String,
  #[serde(rename = "currentBalance")]
  pub current_balance: f64,
  #[serde(rename = "cashFlow")]
  pub cash_flow: f64,
  #[serde(rename = "siteName")]
  pub site_name: String,
  #[serde(rename = "oneDayBalancePercentageChange")]
  pub one_day_balance_percentage_change: i64,
  #[serde(rename = "oneDayBalanceValueChange")]
  pub one_day_balance_value_change: i64,
  pub expense: i64,
  #[serde(rename = "dateRangePerformanceValueChange")]
  pub date_range_performance_value_change: f64,
  #[serde(rename = "closedDate")]
  pub closed_date: String,
  #[serde(rename = "percentOfTotal")]
  pub percent_of_total: f64,
  #[serde(rename = "userAccountId")]
  pub user_account_id: i64,
  #[serde(rename = "oneDayPerformanceValueChange")]
  pub one_day_performance_value_change: i64,
  #[serde(rename = "dateRangeBalancePercentageChange")]
  pub date_range_balance_percentage_change: i64,
  #[serde(rename = "balanceAsOfEndDate")]
  pub balance_as_of_end_date: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct History {
  pub date: String,
  #[serde(rename = "aggregateDailyChangeAmount")]
  pub aggregate_daily_change_amount: Option<f64>,
  #[serde(rename = "aggregateCashIn")]
  pub aggregate_cash_in: Option<f64>,
  pub balances: HashMap<String, HistoryBalance>,
  #[serde(rename = "aggregateBalance")]
  pub aggregate_balance: f64,
  #[serde(rename = "aggregateCashOut")]
  pub aggregate_cash_out: Option<f64>,
  #[serde(rename = "dailyChangeAmount")]
  pub daily_change_amount: Option<HashMap<String, f64>>,
  #[serde(rename = "aggregateIncome")]
  pub aggregate_income: Option<f64>,
  pub cashflows: Option<HashMap<String, Cashflow>>,
  #[serde(rename = "aggregateExpense")]
  pub aggregate_expense: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Cashflow {
  pub income: f64,
  #[serde(rename = "cashIn")]
  pub cash_in: f64,
  pub expense: i64,
  #[serde(rename = "cashOut")]
  pub cash_out: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct NetworthHistory {
  pub date: String,
  #[serde(rename = "totalMortgage")]
  pub total_mortgage: i64,
  #[serde(rename = "totalOtherAssets")]
  pub total_other_assets: i64,
  #[serde(rename = "totalAssets")]
  pub total_assets: f64,
  #[serde(rename = "totalCredit")]
  pub total_credit: i64,
  #[serde(rename = "totalLoan")]
  pub total_loan: i64,
  #[serde(rename = "oneDayNetworthPercentageChange")]
  pub one_day_networth_percentage_change: f64,
  #[serde(rename = "totalLiabilities")]
  pub total_liabilities: i64,
  #[serde(rename = "totalOtherLiabilities")]
  pub total_other_liabilities: i64,
  #[serde(rename = "oneDayNetworthChange")]
  pub one_day_networth_change: f64,
  #[serde(rename = "totalEmpower")]
  pub total_empower: i64,
  #[serde(rename = "totalCash")]
  pub total_cash: i64,
  pub networth: f64,
  #[serde(rename = "totalInvestment")]
  pub total_investment: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OneDaySummaries {
  #[serde(rename = "aggregatedOneDayPercentageChange")]
  pub aggregated_one_day_percentage_change: i64,
  #[serde(rename = "aggregatedOneDayValueChange")]
  pub aggregated_one_day_value_change: i64,
}
