package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

type Node struct {
	Parent   *Node
	Left     *Node
	Right    *Node
	LeftNum  int
	RightNum int
}

func main() {
	file, err := os.Open("./example.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	list := []*Node{}

	for scanner.Scan() {
		line := scanner.Text()
		list = append(list, parseList(line, nil))
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	node := list[0]
	for i := 1; i < len(list); i++ {
		node := &Node{
			Parent:   nil,
			Left:     node,
			Right:    list[i],
			LeftNum:  -1,
			RightNum: -1,
		}
		node.Left.Parent = node
		node.Right.Parent = node
		node = reduce(node)
	}

	fmt.Printf("%+v", node)
}

func parseList(in string, parent *Node) *Node {
	node := &Node{Parent: parent}

	if len(in) == 5 {
		a0, _ := strconv.Atoi(string(in[1]))
		a1, _ := strconv.Atoi(string(in[3]))
		node.LeftNum = a0
		node.RightNum = a1
	} else if in[1] == '[' {
		i := findCloseIndex(in[1:]) + 1
		node.Left = parseList(in[1:i+1], node)
		if in[i+2] == '[' {
			node.Right = parseList(in[i+2:len(in)-1], node)
		} else {
			a1, _ := strconv.Atoi(string(in[1+2]))
			node.RightNum = a1
		}

	} else {
		a0, _ := strconv.Atoi(string(in[1]))
		node.LeftNum = a0
		if in[3] == '[' {
			node.Right = parseList(in[3:len(in)-1], node)
		} else {
			a1, _ := strconv.Atoi(string(in[3]))
			node.RightNum = a1
		}
	}

	return node
}

func findCloseIndex(in string) int {
	d := 0

	for i, _ := range in {
		if in[i] == ']' {
			d--
			if d == 0 {
				return i
			}
		} else if in[i] == '[' {
			d++
		}
	}

	return 0
}

func reduce(node *Node) *Node {
	n := findExplode(node, 0)
	if n != nil {
		node.explode(n)
		node = reduce(node)
	}
	node.split()

	return node
}

func findExplode(node *Node, d int) *Node {
	if d == 4 {
		return node
	}

	if node.Left != nil {
		return findExplode(node.Left, d+1)
	}
	if node.Right != nil {
		return findExplode(node.Right, d+1)
	}
	return nil
}

func (node *Node) explode(n *Node) {
	addToLeft(n, n.LeftNum)
	addToRight(n, n.RightNum)
	if n == n.Parent.Left {
		n.Parent.Left = nil
		n.Parent.LeftNum = 0
	} else {
		n.Parent.Right = nil
		n.Parent.RightNum = 0
	}
}

func addToLeft(node *Node, n int) {
	if node.Parent != nil {
		if node.Parent.Left == nil {
			node.Parent.LeftNum += n
		} else if node.Parent.Left != node {
			node.Parent.Left.RightNum += n
		} else {
			addToLeft(node.Parent, n)
		}
	}
}

func addToRight(node *Node, n int) {
	if node.Parent != nil {
		if node.Parent.Right == nil {
			node.Parent.RightNum += n
		} else if node.Parent.Right != node {
			node.Parent.Right.LeftNum += n
		} else {
			addToRight(node.Parent, n)
		}
	}
}

func (node *Node) split() {

}
