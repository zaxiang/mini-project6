use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};
use aws_sdk_dynamodb::{Client, types::AttributeValue};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract information from the request
    let ap_class = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("ap_class"));

    // Create a message to return
    // Find the data in the DynamoDB database by accessing the number_of_students column
    let message = match ap_class {
        Some(ap_class) => {
            // Create DynamoDB client with default configuration
            let config = aws_config::load_from_env().await;
            let client = Client::new(&config);

            // Query DynamoDB to get the number of students for the given AP class
            let item = client
                .get_item()
                .table_name("StudentAPClasses")
                .key("ap_class", AttributeValue::S(ap_class.to_string()))
                .send()
                .await?;

            match &item.item {
                Some(item_map) => {
                    if let Some(AttributeValue::N(number_of_students)) = item_map.get("number_of_students") {
                        format!("The number of students in {} for the AP class {} is {}.", ap_class, ap_class, number_of_students)
                    } else {
                        // Handle case where "number_of_students" key is not found or not a number
                        "Data for the AP class is not found or is not a number".to_string()
                    }
                }
                None => format!("No data found for the AP class {}.", ap_class),
            }
        }
        None => "Please provide an AP class in the URL as a query parameter".to_string(),
    };

    // Return the response
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize tracing
    tracing::init_default_subscriber();

    // Start the Lambda function
    run(service_fn(function_handler)).await
}