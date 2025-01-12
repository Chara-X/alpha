use tiny_http::{Response, Server};

fn main() {
    let server = Server::http("127.0.0.1:3000").unwrap();
    println!("Server running on http://127.0.0.1:3000");

    for mut request in server.incoming_requests() {
        // Print the request URL
        println!("Request URL: {}", request.url());

        // Print the request headers
        println!("Headers:");
        for header in request.headers() {
            println!("  {}: {}", header.field, header.value);
        }

        // Print the request body (if any)
        let mut body = String::new();
        if let Ok(_) = request.as_reader().read_to_string(&mut body) {
            println!("Body: {}", body);
        } else {
            println!("Body: [Failed to read body]");
        }

        // Respond with a simple message
        let response = Response::from_string("Request received!");
        request.respond(response).unwrap();
    }
}
