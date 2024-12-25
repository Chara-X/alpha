package example

func Compress(b []byte) []byte {
	var output = []byte{}
	for i := 0; i < len(b); {
		var length, offset = 0, 0
		for j := 0; j < i; j++ {
			var l = 0
			for i+l < len(b) && b[i+l] == b[j+l] {
				l++
			}
			if l > length {
				offset = i - j
				length = l
			}
		}
		if length > 1 {
			output = append(output, byte(1))
			output = append(output, byte(offset))
			output = append(output, byte(length))
			i += length
		} else {
			output = append(output, byte(0))
			output = append(output, b[i])
			i++
		}
	}
	return output
}
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
