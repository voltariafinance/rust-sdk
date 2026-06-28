pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DrawdownCreatePayload {
    /// The amount for the drawdown.
    pub amount: DrawdownCreatePayloadAmount,
    /// The date for the drawdown. If not provided, defaults to the current date and time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drawdown_date: Option<DateTime<FixedOffset>>,
}

impl DrawdownCreatePayload {
    pub fn builder() -> DrawdownCreatePayloadBuilder {
        <DrawdownCreatePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DrawdownCreatePayloadBuilder {
    amount: Option<DrawdownCreatePayloadAmount>,
    drawdown_date: Option<DateTime<FixedOffset>>,
}

impl DrawdownCreatePayloadBuilder {
    pub fn amount(mut self, value: DrawdownCreatePayloadAmount) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn drawdown_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.drawdown_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DrawdownCreatePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](DrawdownCreatePayloadBuilder::amount)
    pub fn build(self) -> Result<DrawdownCreatePayload, BuildError> {
        Ok(DrawdownCreatePayload {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            drawdown_date: self.drawdown_date,
        })
    }
}
