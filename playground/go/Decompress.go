package main

func Decompress(input []byte) []byte {
	var output = []byte{}
	for i := 0; i < len(input); {
		if input[i] == 0 {
			output = append(output, input[i+1])
			i += 2
		} else {
			var offset = int(input[i+1])
			var length = int(input[i+2])
			for j := 0; j < length; j++ {
				output = append(output, output[len(output)-offset])
			}
			i += 3
		}
	}
	return output
}
