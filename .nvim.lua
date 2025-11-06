local function list_four_digit_folders_and_add_template()
	local current_dir = vim.fn.getcwd()

	-- The broad pattern: starts with a digit.
	local broad_pattern = current_dir .. "/[0-9]*"
	local all_items_string = vim.fn.glob(broad_pattern, true, false)
	local final_list = { "year_template" }

	if all_items_string == "" or all_items_string == -1 then
		return final_list
	end

	local all_paths = vim.split(all_items_string, "\n", { trimempty = true })

	-- 🚨 Updated Lua regex pattern: exactly four digits.
	-- '^%d%d%d%d$' is a cleaner way to write '^[0-9][0-9][0-9][0-9]$'
	-- %d is a shortcut for [0-9] in Lua patterns.
	local strict_year_pattern = "^%d%d%d%d$"

	for _, path in ipairs(all_paths) do
		local name = vim.fn.fnamemodify(path, ":t")

		-- Apply the strict Lua regex check for exactly four digits
		if string.match(name, strict_year_pattern) then
			table.insert(final_list, name)
		end
	end

	return final_list
end

vim.lsp.config("rust_analyzer", {
	settings = {
		["rust-analyzer"] = {
			cachePriming = {
				enable = false,
			},
			files = {
				exclude = list_four_digit_folders_and_add_template(),
			},
		},
	},
})
