package main

import "fmt"

func main() {
	var number int = 837799
	for number != 1 {
		if number % 2 != 0 {
			number = number * 3 + 1 
			fmt.Println(number)
		} else {
            number = number / 2 
            fmt.Println(number)
		}
	} 
}

