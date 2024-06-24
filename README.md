# data2img

A service to convert base64 to image to bypass GitHub's data uri limitation.

## Usage

### Request

`<base_url>/?type=<data_type>&data=<data>`

#### Arguments

| Argument  | Required |  Type  |                                                                               Description                                                                               |
|:---------:|:--------:|:------:|:-----------------------------------------------------------------------------------------------------------------------------------------------------------------------:|
|   data    |   Yes    | string |                                                                           The data to convert                                                                           |
| data_type |    No    | string | The type of the data, can be `brotli`, `deflate`, `zstd`, `gzip`, `text`.<br/> if not provided, default to `text`<br/> For `text` type should not encode data as base64 |