"""Serveur d'aperçu design : sert ui/ et injecte un hôte Tauri simulé dans chaque page HTML.

Outil de développement uniquement : il vit hors de ui/, que Tauri embarque tel quel.
Usage : python3 tools/design-preview/serve.py [port]
"""

import http.server
from http import HTTPStatus
import pathlib
import sys

TOOL = pathlib.Path(__file__).resolve().parent
ROOT = TOOL.parent.parent / "ui"
PREFIX = "/_preview/"
DEFAULT_PORT = 4173
INJECT = b'<head>\n<script src="/_preview/mock-tauri.js"></script>'


class Handler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=str(ROOT), **kwargs)

    def end_headers(self):
        self.send_header("Cache-Control", "no-store")
        super().end_headers()

    def translate_path(self, path):
        clean = path.split("?", 1)[0].split("#", 1)[0]
        if clean.startswith(PREFIX):
            target = (TOOL / clean[len(PREFIX):]).resolve()
            return str(target if target.is_relative_to(TOOL) else TOOL / "_")
        return super().translate_path(path)

    def do_GET(self):
        page = self.ui_page()
        if page is None:
            super().do_GET()
        else:
            self.send_with_mock(page)

    def ui_page(self):
        path = self.path.split("?", 1)[0].split("#", 1)[0]
        if path.startswith(PREFIX):
            return None
        if path.endswith("/"):
            path += "index.html"
        target = (ROOT / path.lstrip("/")).resolve()
        inside = target.is_relative_to(ROOT) and target.is_file()
        return target if inside and target.suffix == ".html" else None

    def send_with_mock(self, page):
        body = page.read_bytes().replace(b"<head>", INJECT, 1)
        self.send_response(HTTPStatus.OK)
        self.send_header("Content-Type", "text/html; charset=utf-8")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)


if __name__ == "__main__":
    port = int(sys.argv[1]) if len(sys.argv) > 1 else DEFAULT_PORT
    http.server.ThreadingHTTPServer(("127.0.0.1", port), Handler).serve_forever()
