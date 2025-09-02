local hatred = {}

type assignable = {
    draw: (()->())?,
    update: ((dt:number)->())?,
    init: (()->())?,
}

return hatred :: typeof(hatred) & assignable