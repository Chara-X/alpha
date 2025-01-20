import "@typespec/http";
using TypeSpec.Http;
@service({
  title: "APTS OpenAPI Docs",
})
@server("https://10.166.209.110", "OpenPalette cluster-cim2")
namespace PetStore;
@doc("""
  Get status info for the service.
  The status includes the current version of the service.
  The status value may be one of:
  - `ok`: the service is operating normally
  - `degraded`: the service is operating in a degraded state
  - `down`: the service is not operating
  """)
@tag("Status")
@route("/status")
@get
op status(): string;
