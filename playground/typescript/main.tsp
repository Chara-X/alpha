import "@typespec/http";
using TypeSpec.Http;
@service({
  title: "APTS OpenAPI Docs",
})
@server("https://10.166.209.110", "OpenPalette cluster-cim2")
namespace PetStore;
/**
 * @returns 123123123123123123123123123123123
 */
@get op read(@path id: string): {
  /** the widget */
  @body
  widget: Widget;

  @header xRateLimitRemaining: numeric;
  @header xRateLimitReset: numeric;
} | {
  @statusCode _: 404;

  @body
  error: string;
};
model Widget {
  /** the widget id */
  id: string;

  /** the widget name */
  name: string;
}
