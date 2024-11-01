package main


import "fmt"

func main() {
	a := 1.2
	b := 1.0
	c := -1.0
	u := 2.6
	v := 2.0   /// defining values as variables to see if it relates to memory
	fmt.Println("By 64 floats: ", a +  c * b, u + c * v)
	
	a_f32 := float32(1.2)
	b_f32 := float32(1.0)
	c_f32 := float32(-1.0)
	u_f32 := float32(2.6)
	v_f32 := float32(2.0)   /// defining values as variables to see if it relates to memory
	fmt.Println("\tBy 32 floats: ", a_f32 +  c_f32 * b_f32, u_f32 + c_f32 * v_f32)
}
