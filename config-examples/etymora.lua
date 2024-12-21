local configs = require("lspconfig.configs")
local lspconfig = require("lspconfig")
local util = require("lspconfig/util")

configs.etymora = {
  default_config = {
    cmd = { "etymora" },
    -- filetypes = { "markdown" },
    root_dir = util.root_pattern(".git", "Cargo.toml"),
    init_options = {
      dict_config = "example",
    },
  },
  docs = {
    description = [[
      https://github.com/satler-git/etymora

      Language Server for Dictionaries.
    ]],
  },
}

lspconfig.etymora.setup({})
