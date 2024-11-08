function calculate(number: number) {
    while ( number != 1 ) {
        if ( number % 2 !== 0 ) {
            number = number * 3 + 1 
            console.log(number)
        } else if (number % 2 === 0) {
            number = number / 2 
            console.log(number)
        } else {
            console.log(number)
        }    
    }
}

var num: number = Number(process.argv[2])
calculate(num)