local graphics = {
    color = {}
}

local color = graphics.color
local any = nil :: any

type baseColor = {
    r: number,
    g: number,
    b: number,
    a: number
}

export type color = typeof(
    setmetatable(
        {}::baseColor,
        color
    )
)

function graphics.rectangle(pos:vector, size:vector, color:color, anchor:vector)
    return any
end

function color.new(r:number?, g:number?, b:number?, a:number?): color
    return any
end

function color.rgb(r:number?, g:number?, b:number?, a:number?): color
    return any
end

function color.unpack(color:color): (number,number,number,number)
    return any, any, any, any
end

return graphics