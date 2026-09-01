use http_body_util::BodyExt;
use serde::de::DeserializeOwned;

pub trait IntoValue {
    fn into_value<T> (self) -> impl Future<Output = T>
    where
        T: DeserializeOwned;
}

impl IntoValue for axum::http::Response<axum::body::Body> {
    async fn into_value<T> (self) -> T
    where
        T: DeserializeOwned,
    {
        let body_res = self.into_body().collect().await.unwrap();

        serde_json::from_slice(&body_res.to_bytes()).unwrap()
    } 
}