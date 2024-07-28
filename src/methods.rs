use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MethodCard {
    Leitner(LeitnerCardProperties),
}

impl Default for MethodCard {
    fn default() -> Self {
        MethodCard::Leitner(LeitnerCardProperties::default())
    }
}

#[derive(Serialize, Deserialize, Default)]
struct BoxIdx(usize);

#[derive(Serialize, Deserialize, Default)]
pub struct LeitnerCardProperties {
    box_idx: BoxIdx,
    next_review: Option<LeitnerReview>,
    last_review: Option<LeitnerReview>,
    review_history: Vec<LeitnerReview>,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
struct LeitnerReview {
    #[serde_as(as = "DisplayFromStr")]
    date: NaiveDate,
    answer: Option<LeitnerAnswers>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum LeitnerAnswers {
    Correct,
    Incorret,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MethodConfig {
    Leitner(LeitnerConfigProperties),
}

impl Default for MethodConfig {
    fn default() -> Self {
        Self::Leitner(LeitnerConfigProperties::default())
    }
}

#[derive(Serialize, Deserialize)]
pub struct LeitnerConfigProperties {
    boxes: (BoxProp, BoxProp, BoxProp, BoxProp, BoxProp),
}

impl Default for LeitnerConfigProperties {
    fn default() -> Self {
        Self {
            boxes: (
                BoxProp::new(vec![1]),
                BoxProp::new(vec![2, 3]),
                BoxProp::new(vec![7]),
                BoxProp::new(vec![14]),
                BoxProp::new(vec![30]),
            ),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct BoxProp {
    delta_days: Vec<u16>,
}

impl BoxProp {
    fn new(delta_days: Vec<u16>) -> Self {
        Self { delta_days }
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use chrono::NaiveDate;

    use super::{LeitnerAnswers, LeitnerReview};

    #[test]
    fn test_naivedate_serialization() -> Result<()> {
        let date = NaiveDate::from_ymd_opt(2024, 7, 27).expect("Naive date invalid.");
        let review = LeitnerReview {
            date,
            answer: Some(LeitnerAnswers::Correct),
        };
        let serialized = toml::to_string(&review)?;
        assert_eq!(serialized, "date = \"2024-07-27\"\nanswer = \"correct\"\n");
        Ok(())
    }

    #[test]
    fn test_naivedate_deserialization() -> Result<()> {
        let review_str = "date = \"2024-07-27\"\nanswer = \"correct\"\n";
        let deserialized: LeitnerReview = toml::from_str(review_str)?;
        let date = NaiveDate::from_ymd_opt(2024, 7, 27).expect("Naive date invalid.");
        assert_eq!(deserialized.date, date);
        assert_eq!(deserialized.answer, Some(LeitnerAnswers::Correct));
        Ok(())
    }

    #[test]
    fn test_naivedate_deserialization_invalid_date() -> Result<()> {
        let review_str = "date = \"2024-13-27\"\nanswer = \"correct\"\n";
        let deserialized_res: Result<LeitnerReview, toml::de::Error> = toml::from_str(review_str);
        assert!(
            deserialized_res.is_err(),
            "Deserialization of an invalid date must return an error."
        );
        Ok(())
    }
}
