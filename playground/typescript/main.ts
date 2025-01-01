import axios from "axios";
import ky from "ky";

class Point {
    createdAt: number;
    x: number;
    y: number
    constructor(x: number, y: number) {
        this.createdAt = Date.now()
        this.x = x;
        this.y = y;
    }
}
type PointInstance = InstanceType<typeof Point>

function moveRight(point: PointInstance) {
    point.x += 5;
}

const point = new Point(3, 4);
moveRight(point);
point.x; // => 8
axios.get("https://jsonplaceholder.typicode.com/posts/1").then((response: any) => {
    console.log(response.data);
})
ky.get("https://jsonplaceholder.typicode.com/posts/1").json().then((data: any) => {
    console.log(data);
})
