import "@typespec/http";
using TypeSpec.Http;
@service({
  title: "Pet Store",
})
@server("https://example.com", "Single server endpoint")
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
