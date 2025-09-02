local color = {}
local any = nil::any

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

function color.new(r:number?, g:number?, b:number?, a:number?): color
    return any
end

function color.rgb(r:number?, g:number?, b:number?): color
    return any
end

function color.rgba(r:number?, g:number?, b:number?, a:number?)
    return any
end

return color