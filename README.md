# data2img

A service to convert base64 to image to bypass GitHub's data uri limitation.

> Because of vercel limitations, data base64 encoded should not exceed `12k` in length.

> If you want to display it on GitHub, because GitHub's camo service limits the length 
> to about `8k` and will encode the url twice, it is recommended that the length 
> should not exceed `6k`.

> With zstd's maximum compression, svg files up to about `20k` can be displayed properly. 
> You can use `svgo` to process the files before sending them.

## Usage

### Request

`<base_url>/?type=<data_type>&data=<data>`

#### Arguments

| Argument  | Required |  Type  |                                                                               Description                                                                               |
|:---------:|:--------:|:------:|:-----------------------------------------------------------------------------------------------------------------------------------------------------------------------:|
|   data    |   Yes    | string |                                                                           The data to convert                                                                           |
| data_type |    No    | string | The type of the data, can be `brotli`, `deflate`, `zstd`, `gzip`, `text`.<br/> if not provided, default to `text`<br/> For `text` type should not encode data as base64 |
|    url    |    No    | string |                                                                     The url of the data to convert.                                                                     |

> Must provide either `data` or `url` argument.