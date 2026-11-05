# Written for nishiki to reach the grammar units the third-party inputs miss.
// A line comment with two slashes.
/* A block comment
   over two lines. */

variable "region" {
  type    = string
  default = "eu-west-1" /* inline */
}

module network vpc "main" {
  source  = "./modules/vpc"
  cidr    = "10.0.0.0/16"
  subnets = [for i, s in var.subnets : cidrsubnet(var.cidr, 8, i) if s != ""]
  azs     = data.aws_availability_zones.available.names[*]
  ids     = aws_subnet.private.*.id
  first   = aws_subnet.private.0.id
  ratio   = 1.5e-3 * 2e10 / 4 % 3
  limit   = var.count <= 10 && var.count < 20 ? var.count - 1 : var.count + 1
  enabled = !var.disabled
  merged  = merge(var.tags...)
  lookup  = { for k, v in var.map : k => upper(v) }
  pairs   = [for k, v in var.map : k => v]
}

(parenthesised) = "a definition in parentheses"

locals {
  escaped = "tab\tquote\" backslash\\ unicodeé wide\U0001F600 \u0001F600"
  called  = provider::terraform::encode_tfvars({ a = 1 })
  nested  = provider::aws::arn_parse::sub("x")
  object = {
    (var.key) = "computed key"
    (local.prefix.name) : "colon"
    "quoted key" = true
    plain = null
    for k, v in var.extra : k => v
    nullable = 1
    x = [
      a if b
      c
    ]
  }
  template = <<-EOT
    %{~ for name in var.names ~}
    hello ${name}
    %{ if name == "x" }X%{ else }Y%{ endif }
    %{~ endfor ~}
  EOT
  member = var.nullable.truex.falsey
}

dynamic "setting" {
  for_each = var.settings
  content {
    namespace = setting.value["namespace"]
    map {
    }
  }
}
