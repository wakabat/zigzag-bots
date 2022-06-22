#![allow(dead_code)]

/// Data structures for ZigZag Exchange API as documented in the link below:
/// https://github.com/ZigZagExchange/backend/blob/0df93198ae3278e7e70cef75911f2d1fa4b2c7b0/README.md
/// For now, this module only supports zksync deployments, starknet support will be added
/// at a later time.
use serde::{Deserialize, Serialize};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};
pub use zksync::zksync_types::{Order as ZksyncOrder, H256};

pub type ChainId = u32;
pub type FillId = u32;
pub type OrderId = u32;
pub type UserId = String;
pub type Market = String;
pub type Price = f64;
pub type Amount = f64;
pub type Fee = f64;
pub type Timestamp = u64;
pub type Date = String;
pub type Token = String;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(test, derive(strum_macros::EnumIter))]
pub enum OrderStatus {
    #[serde(rename = "c")]
    Canceled,
    #[serde(rename = "o")]
    Open,
    #[serde(rename = "e")]
    Expired,
    #[serde(rename = "m")]
    Matched,
    #[serde(rename = "r")]
    Rejected,
    #[serde(rename = "f")]
    Filled,
    #[serde(rename = "b")]
    Broadcasted,
    #[serde(rename = "pf")]
    PartialFill,
    #[serde(rename = "pm")]
    PartialMatch,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Side {
    #[serde(rename = "b")]
    Buy,
    #[serde(rename = "s")]
    Sell,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "op", content = "args", rename_all = "lowercase")]
pub enum Operation {
    Login(LoginArgs),
    Submitorder3(Box<Submitorder3Args>),
    Indicateliq2(Indicateliq2Args),
    Fillrequest(Box<FillrequestArgs>),
    Userordermatch(Box<UserordermatchArgs>),
    Orderreceiptreq(OrderreceiptreqArgs),
    Orderreceipt(Order),
    Fillreceiptreq(FillreceiptreqArgs),
    Fillreceipt(Fill),
    Orders(OrdersArgs),
    Fills(FillsArgs),
    // Orderstatus(OrderstatusArgs),
    Fillstatus(FillstatusArgs),
    Liquidity2(Liquidity2Args),
    Refreshliquidity(RefreshliquidityArgs),
    Lastprice(LastpriceArgs),
    Marketsummary(MarketsummaryArgs),
    Subscribemarket(SubscribemarketArgs),
    Unsubscribemarket(UnsubscribemarketArgs),
    Userorderack(Order),
    Cancelall(CancelallArgs),
    Requestquote(RequestquoteArgs),
    Quote(QuoteArgs),
    Marketinfo(MarketinfoArgs),
    Marketinfo2(Marketinfo2Args),
    Marketreq(MarketreqArgs),
    Dailyvolumereq(DailyvolumereqArgs),
    Dailyvolume(DailyvolumeArgs),
    Error(ErrorArgs),
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct LoginArgs {
    chain_id: ChainId,
    user_id: UserId,
}

// TODO: Order from zksync_types do not derive PartialEq trait, maybe we should
// define a new zksync order type?
#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
pub struct Submitorder3Args {
    chain_id: ChainId,
    market: Market,
    zk_order: ZksyncOrder,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct Indicateliq2Args {
    chain_id: ChainId,
    market: Market,
    liquidity: Vec<Liquidity>,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct Liquidity {
    side: Side,
    price: Price,
    base_quantity: Amount,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    expires: Option<Timestamp>,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
pub struct FillrequestArgs {
    chain_id: ChainId,
    order_id: OrderId,
    fill_order: ZksyncOrder,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
pub struct UserordermatchArgs {
    chain_id: ChainId,
    // TODO: verify if those should be plain order, or zksync order
    taker_order: ZksyncOrder,
    maker_order: ZksyncOrder,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct OrderreceiptreqArgs {
    chain_id: ChainId,
    order_id: OrderId,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct Order {
    chain_id: ChainId,
    id: OrderId,
    market: Market,
    side: Side,
    price: Price,
    base_quantity: Amount,
    quote_quantity: Amount,
    expires: Timestamp,
    user_id: UserId,
    order_status: OrderStatus,
    remaining: Amount,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    tx_hash: Option<H256>,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct FillreceiptreqArgs {
    chain_id: ChainId,
    order_id: OrderId,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct Fill {
    chain_id: ChainId,
    id: FillId,
    market: Market,
    side: Side,
    price: Price,
    base_quantity: Amount,
    fill_status: OrderStatus,
    tx_hash: H256,
    taker_user_id: UserId,
    maker_user_id: UserId,
    fee_amount: Fee,
    fee_token: Token,
    timestamp: Date,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RemainingOrError {
    Remaining(Amount),
    Error(String),
}

// TODO: depending on OrderStatus, this could have different values attached,
// deal with this type later.
#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct OrderUpdate {
    chain_id: ChainId,
    order_id: OrderId,
    status: OrderStatus,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct OrdersArgs {
    orders: Vec<Order>,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct FillsArgs {
    fills: Vec<Fill>,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct OrderstatusArgs {
    updates: Vec<OrderUpdate>,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct FillStatus {
    chain_id: ChainId,
    full_id: FillId,
    status: OrderStatus,
    tx_hash: H256,
    remaining: Amount,
    fee_amount: Fee,
    fee_token: Token,
    timestamp: Timestamp,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct FillstatusArgs {
    statuses: Vec<FillStatus>,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct Liquidity2Args {
    chain_id: ChainId,
    market: Market,
    liquidity: Vec<Liquidity>,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct RefreshliquidityArgs {
    chain_id: ChainId,
    market: Market,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct PriceUpdate {
    market: Market,
    price: Price,
    price_change: Price,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    quote_volume: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    base_volume: Option<Amount>,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct LastpriceArgs {
    updates: Vec<PriceUpdate>,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct MarketsummaryArgs {
    // chain_id: ChainId,
    market: Market,
    price: Price,
    high_24: Price,
    low_24: Price,
    price_change: Price,
    base_volume: Amount,
    quote_volume: Amount,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct SubscribemarketArgs {
    chain_id: ChainId,
    market: Market,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct UnsubscribemarketArgs {
    chain_id: ChainId,
    market: Market,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct CancelorderArgs {
    chain_id: ChainId,
    order_id: OrderId,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct CancelallArgs {
    chain_id: ChainId,
    user_id: UserId,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct RequestquoteArgs {
    chain_id: ChainId,
    market: Market,
    side: Side,
    base_quantity: Amount,
    quote_quantity: Amount,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct QuoteArgs {
    chain_id: ChainId,
    market: Market,
    side: Side,
    base_quantity: Amount,
    price: Price,
    quote_quantity: Amount,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    id: u32,
    address: String,
    symbol: String,
    decimals: u32,
    enabled_for_fees: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MarketInfo {
    base_asset_id: u32,
    quote_asset_id: u32,
    base_fee: Price,
    quote_fee: Price,
    // min_size: Amount,
    // max_size: Amount,
    zigzag_chain_id: ChainId,
    price_precision_decimal: u32,
    base_asset: Asset,
    quote_asset: Asset,
    // id: String,
    alias: Market,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct MarketinfoArgs {
    market_info: MarketInfo,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct Marketinfo2Args {
    market_infos: Vec<MarketInfo>,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct MarketreqArgs {
    chain_id: ChainId,
    detailed: bool,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct DailyvolumereqArgs {
    chain_req: u32,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct Volume {
    chain_id: ChainId,
    market: Market,
    date: Date,
    base_volume: Amount,
    quote_volume: Amount,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct DailyvolumeArgs {
    volumes: Vec<Volume>,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq)]
pub struct ErrorArgs {
    operation: String,
    error: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{from_str, json, to_string, to_value};
    use strum::IntoEnumIterator;

    #[test]
    fn test_serialize_login() {
        let op = Operation::Login(LoginArgs {
            chain_id: 1000,
            user_id: "27334".into(),
        });
        let sop = to_value(&op).expect("to_value");
        assert_eq!(
            sop,
            json!({
                "op": "login",
                "args": [1000, "27334"],
            })
        );
    }

    #[test]
    fn test_deserialize_login() {
        let s = r##"{ "op": "login", "args": [1000, "27334"] }"##;
        let op: Operation = from_str(s).expect("from_str");
        assert!(matches!(op,
                         Operation::Login(LoginArgs{ chain_id, user_id })
                             if chain_id == 1000 && user_id == "27334"));
    }

    #[test]
    fn test_serialize_order_status() {
        let s = OrderStatus::PartialFill;
        let ss = to_value(&s).expect("to_value");
        assert_eq!(ss, json!("pf"));
    }

    #[test]
    fn test_deserialize_order_status() {
        let s = r##""f""##;
        let s: OrderStatus = from_str(s).expect("from_strr");
        assert_eq!(s, OrderStatus::Filled);
    }

    #[test]
    fn test_roundtrip_order_status() {
        for s in OrderStatus::iter() {
            let t = to_string(&s).expect("to_string");
            let s2 = from_str(&t).expect("from_str");
            assert_eq!(s, s2);
        }
    }

    #[test]
    fn test_serialize_liquidity() {
        let l = Liquidity {
            side: Side::Buy,
            price: 3100.0,
            base_quantity: 1.2322,
            expires: Some(1642677967),
        };
        let v = to_value(&l).expect("to_value");
        assert_eq!(v, json!(["b", 3100.0, 1.2322, 1642677967,]));
        let l = Liquidity {
            side: Side::Buy,
            price: 3100.0,
            base_quantity: 1.2322,
            expires: None,
        };
        let v = to_value(&l).expect("to_value");
        assert_eq!(v, json!(["b", 3100.0, 1.2322,]));
    }

    #[test]
    fn test_deserialize_liquidity() {
        let s = r##"["s", 3300, 0.2822, 1642677969]"##;
        let l: Liquidity = from_str(s).expect("from_str");
        assert_eq!(
            l,
            Liquidity {
                side: Side::Sell,
                // Ideally we shouldn't test for float's equalness this way,
                // but allow me to be lazy for a bit.
                price: 3300.0,
                base_quantity: 0.2822,
                expires: Some(1642677969),
            }
        );
        let s2 = r##"["s", 3300, 0.2822]"##;
        let l2: Liquidity = from_str(s2).expect("from_str");
        assert_eq!(
            l2,
            Liquidity {
                side: Side::Sell,
                price: 3300.0,
                base_quantity: 0.2822,
                expires: None,
            }
        );
    }

    #[test]
    fn test_deserialize_order_receipt() {
        let s = r##"
{ 
  "op": "orderreceipt", 
  "args": [
    1000,
    40,
    "ETH-USDT",
    "s",
    3370.93,
    0.1,
    337.093,
    4294967295,
    "23",
    "f",
    0,
    "0x600ad64c7a931753bbd3ad24cc21efb8513de1dab67daf25b934db8d01f91ed9"
  ] 
}
        "##
        .trim();
        let op: Operation = from_str(s).expect("from_str");
        if let Operation::Orderreceipt(order) = op {
            assert_eq!("23", order.user_id);
            assert_f64_near!(order.price, 3370.93);
            assert!(!order.tx_hash.unwrap().is_zero());
        } else {
            panic!("Invalid op type: {:?}", op);
        }
    }

    #[test]
    fn test_deserialize_remaining_or_error() {
        let r: RemainingOrError = from_str("1").expect("from_str");
        assert_eq!(r, RemainingOrError::Remaining(1.0));
        let r: RemainingOrError = from_str("\"Not enough balance\"").expect("from_str");
        assert_eq!(r, RemainingOrError::Error("Not enough balance".into()));
    }

    // #[test]
    // fn test_deserialize_order_updates() {
    // let s = r##"
    // {
    // "op": "orderstatus",
    // "args": [
    // [
    // [
    // 1000,
    // 5,
    // "m",
    // 4700.23,
    // "0x5c633d31817a9b95973670733aed5feb8255d67f36f74517462063659bcd7dd0",
    // 1
    // ],
    // [
    // 1000,
    // 890013,
    // "f",
    // "0x51c23f8bcb7aa2cc64c8da28827df6906b8bdc53818eaf398f5198a6850310f0",
    // "Not enough balance"
    // ]
    // ]
    // ]
    // }
    // "##
    // .trim();
    // let op: Operation = from_str(s).expect("from_str");
    // if let Operation::Orderstatus(OrderstatusArgs { updates }) = op {
    // assert_eq!(updates.len(), 2);
    // assert_eq!(
    // updates[0].remaining_or_error,
    // RemainingOrError::Remaining(1.0)
    // );
    // assert_eq!(
    // updates[1].remaining_or_error,
    // RemainingOrError::Error("Not enough balance".into())
    // );
    // } else {
    // panic!("Invalid op type: {:?}", op);
    // }
    // }

    #[test]
    fn test_deserialize_marketinfo2() {
        let s = r##"
{
  "op": "marketinfo2",
  "args": [
    [
      {
        "baseAssetId": 65,
        "quoteAssetId": 1,
        "baseFee": 1,
        "quoteFee": 1,
        "minSize": 1,
        "maxSize": 100,
        "zigzagChainId": 1,
        "pricePrecisionDecimal": 6,
        "baseAsset": {
          "id": 65,
          "address": "0x19ebaa7f212b09de2aee2a32d40338553c70e2e3",
          "symbol": "ARTM",
          "decimals": 18,
          "enabledForFees": false
        },
        "quoteAsset": {
          "id": 1,
          "address": "0x6b175474e89094c44da98b954eedeac495271d0f",
          "symbol": "DAI",
          "decimals": 18,
          "enabledForFees": true
        },
        "id": "nORHCLNmmeS5Cp5or2Xt4gMMovgfVsbwYXA941zq0ks",
        "alias": "ARTM-DAI"
      }
    ]
  ]
}
        "##
        .trim();
        let op: Operation = from_str(s).expect("from_str");
        if let Operation::Marketinfo2(info) = op {
            assert_eq!(info.market_infos.len(), 1);
            assert_eq!(info.market_infos[0].alias, "ARTM-DAI");
            assert_eq!(info.market_infos[0].base_asset.decimals, 18);
            assert!(info.market_infos[0].quote_asset.enabled_for_fees);
        } else {
            panic!("Invalid op type: {:?}", op);
        }
    }
}
