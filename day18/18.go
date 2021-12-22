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
		node = &Node{
			Parent: nil,
			Left:   node,
			Right:  list[i],
		}
		node.Left.Parent = node
		node.Right.Parent = node
		node = reduce(node)
	}

	fmt.Println("---")

	result := magnitude(node)
	fmt.Println(result)
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
		node.split()
		node = reduce(node)
	}

	return node
}

func findExplode(node *Node, d int) *Node {
	if d >= 4 && node.Left == nil && node.Right == nil {
		return node
	}

	var n *Node
	if node.Left != nil {
		n = findExplode(node.Left, d+1)
	}
	if n == nil && node.Right != nil {
		n = findExplode(node.Right, d+1)
	}
	return n
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
			addToLeftRight(node.Parent.Left, n)
		} else {
			addToLeft(node.Parent, n)
		}
	}
}

func addToLeftRight(node *Node, n int) {
	if node.Right == nil {
		node.RightNum += n
	} else {
		addToLeftRight(node.Right, n)
	}
}

func addToRight(node *Node, n int) {
	if node.Parent != nil {
		if node.Parent.Right == nil {
			node.Parent.RightNum += n
		} else if node.Parent.Right != node {
			addToRightLeft(node.Parent.Right, n)
		} else {
			addToRight(node.Parent, n)
		}
	}
}

func addToRightLeft(node *Node, n int) {
	if node.Left == nil {
		node.LeftNum += n
	} else {
		addToRightLeft(node.Left, n)
	}
}

func (node *Node) split() {
	if node.Left != nil {
		node.Left.split()
	}
	if node.Right != nil {
		node.Right.split()
	}

	if node.LeftNum >= 10 {
		node.Left = &Node{
			Parent:   node,
			LeftNum:  node.LeftNum / 2,
			RightNum: node.LeftNum - (node.LeftNum / 2),
		}
		node.LeftNum = 0
	}

	if node.RightNum >= 10 {
		node.Right = &Node{
			Parent:   node,
			LeftNum:  node.RightNum / 2,
			RightNum: node.RightNum - (node.RightNum / 2),
		}
		node.RightNum = 0
	}
}

func magnitude(node *Node) int {
	fmt.Printf("%+v\n", node)
	res := 3*node.LeftNum + 2*node.RightNum

	if node.Left != nil {
		res += 3 * magnitude(node.Left)
	}
	if node.Right != nil {
		res += 2 * magnitude(node.Right)
	}

	return res
}
