local M = {}

-- See https://github.com/tjdevries/rofl.nvim/blob/632c10f2ec7c56882a3f7eda8849904bcac6e8af/lua/rofl.lua
local binary_path = vim.fn.fnamemodify(
  vim.api.nvim_get_runtime_file("lua/colorscheme_changer.lua", false)[1], ":h:h")
  .. "/target/debug/colorscheme-changer"

if vim.fn.executable(binary_path) == 0 then
  binary_path = vim.fn.fnamemodify(
    vim.api.nvim_get_runtime_file("lua/colorscheme_changer.lua", false)[1], ":h:h")
    .. "/target/release/colorscheme-changer"
end

local function start()
  if M.job_id ~= nil then return end
  M.job_id = vim.fn.jobstart({ binary_path }, { rpc = true })
end

local function notify(method, ...)
  start()
  vim.rpcnotify(M.job_id, method, ...)
end

function M.run()
  notify('start')
end

return M
