-- package.cpath = package.cpath..";./target/debug/?.so"
package.cpath = package.cpath..";./target/release/?.so"
local coward = require('libcoward')

-- Now you can call functions from the module
local input = io.read()
local cmp = coward.complete(input)
for k, v in pairs(cmp) do
    print(k..". "..v)
end
