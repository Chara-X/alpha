import axios from "axios";
import ky from "ky";
class Point {
    createdAt;
    x;
    y;
    constructor(x, y) {
        this.createdAt = Date.now();
        this.x = x;
        this.y = y;
    }
}
function moveRight(point) {
    point.x += 5;
}
const point = new Point(3, 4);
moveRight(point);
point.x; // => 8
axios.get("https://jsonplaceholder.typicode.com/posts/1").then((response) => {
    console.log(response.data);
});
ky.get("https://jsonplaceholder.typicode.com/posts/1").json().then((data) => {
    console.log(data);
});