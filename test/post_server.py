import json
from http.server import BaseHTTPRequestHandler, HTTPServer


class SimpleRequestHandler(BaseHTTPRequestHandler):
    def do_POST(self):
        content_length = int(self.headers["Content-Length"])
        post_data = self.rfile.read(content_length)

        try:
            # 解析 JSON 数据
            json_data = json.loads(post_data.decode("utf-8"))

            # 打印 JSON 数据
            print("Received JSON data:")
            print(json_data)

            self.send_response(200)
            self.send_header("Content-type", "application/json")
            self.end_headers()
            response_message = json.dumps(
                {"message": "JSON data received successfully"}
            )
            self.wfile.write(response_message.encode("utf-8"))

        except Exception as e:
            self.send_response(500)
            self.send_header("Content-type", "application/json")
            self.end_headers()
            response_message = json.dumps({"error": str(e)})
            self.wfile.write(response_message.encode("utf-8"))


if __name__ == "__main__":
    server_address = ("", 5000)
    httpd = HTTPServer(server_address, SimpleRequestHandler)
    print("Server running on port 5000...")
    httpd.serve_forever()
