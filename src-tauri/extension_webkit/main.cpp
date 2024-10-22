#include "webkitgtk-4.1/webkit2/webkit-web-extension.h"

static gboolean on_request(WebKitWebPage* page, WebKitURIRequest* request, WebKitURIResponse* response, gpointer user_data) {
  g_print("on_request\n");

  return false;
}

static void page_created(WebKitWebExtension* extension, WebKitWebPage* page, gpointer user_data) {
  g_print("page created\n");
  g_signal_connect(page, "send-request", G_CALLBACK(on_request), NULL);
}

void webkit_web_process_extension_initialize(WebKitWebExtension* extension) {
  g_signal_connect(extension, "page-created", G_CALLBACK(page_created), NULL);
}
