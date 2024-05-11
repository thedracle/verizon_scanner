use anyhow::Result;
use serde::{Deserialize, Serialize};
use reqwest::Response;

#[derive(Debug, Serialize, Deserialize)]
pub struct Qualification {
    gigqualified: String,
    fiosqualified: String,
    hsiqualified: String,
    posturl: String,
    #[serde(rename = "visitId")]
    visit_id: String,
    #[serde(rename = "visitorId")]
    visitor_id: String,
    #[serde(rename = "commonLq")]
    common_lq: String,
    lbo: String,
    captcha: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualificationDetails {
    data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmartCartDetails {
    #[serde(rename = "smartCartAvailable")]
    smart_cart_available: bool,
    #[serde(rename = "creditCheckCompleted")]
    credit_check_completed: bool,
    #[serde(rename = "cartRetrieved")]
    cart_retrieved: bool,
    #[serde(rename = "smartCartMissMatch")]
    smart_cart_miss_match: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    services: Vec<String>,
    #[serde(rename = "smartCartDetails")]
    smart_cart_details: SmartCartDetails,
    #[serde(rename = "pendingOrder")]
    pending_order: String,
    #[serde(rename = "inService")]
    in_service: String,
    #[serde(rename = "hoaFlag")]
    hoa_flag: String,
    #[serde(rename = "isLennarEligible")]
    is_lennar_eligible: String,
    #[serde(rename = "tarCode")]
    tar_code: String,
    cpnelg: String,
    #[serde(rename = "fiosSelfInstall")]
    fios_self_install: String,
    #[serde(rename = "fiosReady")]
    fios_ready: String,
    #[serde(rename = "quantumEligible")]
    quantum_eligible: String,
    #[serde(rename = "wirelessPlanType")]
    wireless_plan_type: String,
    #[serde(rename = "mvStopOrder")]
    mv_stop_order: bool,
    #[serde(rename = "fiveG")]
    five_g: bool,
    #[serde(rename = "addressNotFound")]
    address_not_found: bool,
    #[serde(rename = "qualified4GHome")]
    qualified4_ghome: bool,
    #[serde(rename = "ofsECD")]
    ofs_ecd: String,
    #[serde(rename = "isOFSApproaching")]
    is_ofsapproaching: String,
    #[serde(rename = "ofsPresaleEligible")]
    ofs_presale_eligible: String,
    #[serde(rename = "engineeringInterval")]
    engineering_interval: String,
    #[serde(rename = "isInActiveONTPresent")]
    is_in_active_ontpresent: String,
    #[serde(rename = "ontPonType")]
    ont_pon_type: String,
    qualified: String,
    #[serde(rename = "isFCP")]
    is_fcp: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostValues {
    #[serde(rename = "vendorName")]
    vendor_name: String,
    #[serde(rename = "campaignCode")]
    campaign_code: String,
    #[serde(rename = "targetUrl")]
    target_url: String,
    config: Config,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "addressInfo")]
    address_info: AddressInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressInfo {
    #[serde(rename = "addressId")]
    address_id: String,
    city: String,
    state: String,
    #[serde(rename = "zipCode")]
    zip_code: String,
    #[serde(rename = "addressLine1")]
    address_line1: String,
    #[serde(rename = "addressLine2")]
    address_line2: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedAddress {
    #[serde(rename = "addressString")]
    address_string: String,
    state: String,
    #[serde(rename = "addressID")]
    address_id: String,
    #[serde(rename = "zipCode")]
    zip_code: String,
    city: String,
    #[serde(rename = "baseAddress")]
    base_address: bool,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct FiosResponse {
    qualification: Qualification,
    #[serde(rename = "qualificationDetails")]
    qualification_details: QualificationDetails,
    #[serde(rename = "postValues")]
    post_values: PostValues,
    #[serde(rename = "multipleAddressMatch")]
    multiple_address_match: String,
    #[serde(rename = "parsedAddress")]
    parsed_address: Vec<ParsedAddress>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerInfo {
    #[serde(rename = "accountNumber")]
    account_number: String,
    #[serde(rename = "cartCreator")]
    cart_creator: String,
    #[serde(rename = "processingMTN")]
    processing_mtn: String,
    #[serde(rename = "registerNumber")]
    register_number: i64,
    #[serde(rename = "multipleAddressQualification")]
    multiple_address_qualification: bool,
    #[serde(rename = "isDigitalBulkQualification")]
    is_digital_bulk_qualification: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub qualified: bool,
    #[serde(rename = "qualified4GHome")]
    pub qualified4_ghome: bool,
    #[serde(rename = "aqCaseId")]
    pub aq_case_id: String,
    #[serde(rename = "addressLine1")]
    pub address_line1: String,
    #[serde(rename = "addressLine2")]
    pub address_line2: String,
    pub city: String,
    pub state: String,
    #[serde(rename = "zipCode")]
    pub zip_code: String,
    #[serde(rename = "addressFromAccount")]
    pub address_from_account: String,
    #[serde(rename = "eventCorrelationId")]
    pub event_correlation_id: String,
    #[serde(rename = "currentFloorNumber")]
    pub current_floor_number: String,
    #[serde(rename = "floorPlanAvailable")]
    pub floor_plan_available: bool,
    #[serde(rename = "uberPinEligible")]
    pub uber_pin_eligible: bool,
    #[serde(rename = "fiosQualified")]
    pub fios_qualified: Option<bool>,
    #[serde(rename = "fiosResponse")]
    pub fios_response: Option<FiosResponse>,
    #[serde(rename = "errorMap")]
    pub error_map: Option<String>,
    #[serde(rename = "displayStreetSelection")]
    pub display_street_selection: bool,
    #[serde(rename = "processStepOut")]
    pub process_step_out: String,
    #[serde(rename = "customerInfo")]
    pub customer_info: CustomerInfo,
    #[serde(rename = "moveQualified")]
    pub move_qualified: bool,
    #[serde(rename = "planChangeRequired")]
    pub plan_change_required: bool,
    #[serde(rename = "existingFWA")]
    pub existing_fwa: bool,
    #[serde(rename = "existingMobility")]
    pub existing_mobility: bool,
    #[serde(rename = "parsedAddressResponse")]
    pub parsed_address_response: bool,
    #[serde(rename = "tanglewoodQualified")]
    pub tanglewood_qualified: bool,
    #[serde(rename = "HSI")]
    pub hsi: Option<bool>,
    #[serde(rename = "qualifiedCBand")]
    pub qualified_cband: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerizonResponse {
    pub output: Output,
}

#[derive(Debug, Serialize)]
pub struct VerizonRequest {
    address1: String,
    city: String,
    state: String,
    zipcode: String,
    #[serde(rename = "ntasAddrID")]
    ntas_addr_id: String,
    #[serde(rename = "locusID")]
    locus_id: String,
    address2: String,
    #[serde(rename = "scEmail")]
    sc_email: String,
    #[serde(rename = "scMtn")]
    sc_mtn: String,
    flowtype: String,
    #[serde(rename = "captchaChangeNeeded")]
    captcha_change_needed: bool,
    #[serde(rename = "loopQual")]
    loop_qual: bool,
    #[serde(rename = "includeCband")]
    include_cband: bool,
    #[serde(rename = "cBandOnly")]
    c_band_only: bool,
    #[serde(rename = "superBowlPromo")]
    super_bowl_promo: bool,
    #[serde(rename = "isLoadTest")]
    is_load_test: bool,
    #[serde(rename = "preOrder")]
    pre_order: bool,
    #[serde(rename = "isRevist")]
    is_revist: bool,
    referrer: String,
    #[serde(rename = "vendorId")]
    vendor_id: String,
    cmp: String,
    #[serde(rename = "outletId")]
    outlet_id: String,
    #[serde(rename = "locationCode")]
    location_code: String,
    #[serde(rename = "cbandPro")]
    cband_pro: String,
    #[serde(rename = "buildingId")]
    building_id: String,
    #[serde(rename = "prefState")]
    pref_state: String,
}

pub async fn lookup_verizon_service(address: &str, city: &str, state: &str, zip: &str) -> Result<Response> {
    let client = reqwest::Client::new();
    let url = "https://www.verizon.com/soe/digital/prospect/qualificationservice/nse/check5GAvailability";

    let request = VerizonRequest {
        address1: address.to_string(),
        city: city.to_string(),
        state: state.to_string(),
        zipcode: zip.to_string(),
        ntas_addr_id: "".to_string(),
        locus_id: "".to_string(),
        address2: "".to_string(),
        sc_email: "".to_string(),
        sc_mtn: "".to_string(),
        flowtype: "LQ2.0".to_string(),
        captcha_change_needed: true,
        loop_qual: true,
        include_cband: true,
        c_band_only: false,
        super_bowl_promo: false,
        is_load_test: false,
        pre_order: true,
        is_revist: false,
        referrer: "".to_string(),
        vendor_id: "".to_string(),
        cmp: "".to_string(),
        outlet_id: "".to_string(),
        location_code: "".to_string(),
        cband_pro: "Y".to_string(),
        building_id: "".to_string(),
        pref_state: state.to_string(),
    };

    Ok(client.post(url)
        .header("Content-Type", "application/json")
        .header("Accept", "*/*")
        .header("Channelid", "VZW-DOTCOM")
        .header("User-Agent", "curl/7.87.1")
        .body(serde_json::to_string(&request).unwrap())
        .send().await?)
}
