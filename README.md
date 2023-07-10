Rewrote a go implementation in rust using std.

Original.
package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"time"
)

func isPrime(num int) bool {
	for i := 2; i < num/2; i++ {
		if num%i == 0 {
			return false

		}

	}
	return true
}

func calcPrime(num int) (int, int) {
	for i := 2; i < num; i++ {
		if isPrime(i) && num%i == 0 {

			return i, num / i

		}

	}
	return 1, num
}

func main() {
	start := time.Now()

	file, err := os.Open("test.txt")
	outputFile, err := os.Create("res.txt")

	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	logger := bufio.NewWriter(outputFile)

	for scanner.Scan() {
		line := scanner.Text()
		// println(line)
		num, _ := strconv.Atoi(line)

		a, b := calcPrime(num)

		logger.WriteString(fmt.Sprintln(a, b))
	}

	logger.Flush()

	finish := time.Now()
	fmt.Println("Succ", finish.Sub(start))
}

