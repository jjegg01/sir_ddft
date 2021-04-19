import http.server
import socketserver
import mimetypes
import os, os.path

PORT = 8080

ROOTPATH = os.path.dirname(os.path.abspath(__file__))
WASMPATH = os.path.join(ROOTPATH, "../pkg/")

# Hijacking SimpleHTTPRequestHandler by manipulating the translate_path method
class MyHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, request, client_address, server):
        self.directories = server.mydirectories
        super().__init__(request, client_address, server)

    def translate_path(self, path):
        # Check all directories listed
        for directory in self.directories:
            self.directory = directory
            candidate = super().translate_path(path)
            if os.path.exists(candidate):
                break
        return candidate

with socketserver.TCPServer(("localhost", PORT), MyHTTPRequestHandler) as srv:
    srv.mydirectories = [ROOTPATH, WASMPATH]
    srv.serve_forever()
