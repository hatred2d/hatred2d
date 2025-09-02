local hatred = require('@hatred/hatred')
local graphics = require('@hatred/graphics')

local color = graphics.color

function hatred.init()
    print("hello world!")
end 

function hatred.draw()
    graphics.rectangle(vector.create(10, 10), vector.create(30,30), color.rgb(255,255,255))
end

function hatred.update(dt:number)

end