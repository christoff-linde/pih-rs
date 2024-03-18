provider "aws" {
  region = var.region
}

data "aws_caller_identity" "current" {}

data "aws_iam_policy_document" "assume_role" {
  statement {
    effect = "Allow"

    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }
    actions = ["sts:AssumeRole"]
  }

}

resource "aws_iam_role" "pih-rs-lambda-role" {
  name               = "pih-rs-lambda-role"
  assume_role_policy = data.aws_iam_policy_document.assume_role.json
}

# data "archive_file" "lambda" {
#   type        = "zip"
#   source_file = "${path.module}/lambda.zip"
#   output_path = "${path.module}/lambda.zip"
# }

resource "aws_lambda_function" "pih-rs" {
  #   filename      = "${path.module}/../../target/lambda/pih-rs/bootstrap.zip"
  filename      = "${path.module}/bootstrap.zip"
  function_name = "pih-rs"
  role          = aws_iam_role.pih-rs-lambda-role.arn
  handler       = "lambda.handler"
  timeout       = 10

  runtime       = "provided.al2"
  architectures = ["arm64"]

  logging_config {
    log_format = "JSON"
  }
}

resource "aws_lambda_function_url" "test_latest" {
  function_name      = aws_lambda_function.pih-rs.function_name
  authorization_type = "NONE"
}



# resource "aws_lambda_function_url" "test_live" {
#   function_name      = aws_lambda_function.pih-rs.function_name
#   qualifier          = "my_alias"
#   authorization_type = "AWS_IAM"

#   cors {
#     allow_credentials = true
#     allow_origins     = ["*"]
#     allow_methods     = ["*"]
#     allow_headers     = ["date", "keep-alive"]
#     expose_headers    = ["keep-alive", "date"]
#     max_age           = 86400
#   }
# }
