const Routes = (function() {
  const routes = [{"name":"index","path":"/","controller":"PoeSystemWeb.IndexController","params":[],"action":"index","method":"GET"},{"name":"poe1.index","path":"/poe1","controller":"PoeSystemWeb.Poe1Controller","params":[],"action":"index","method":"GET"},{"name":"poe1.new.new","path":"/poe1/new","controller":"PoeSystemWeb.Poe1Controller","params":[],"action":"new","method":"POST"},{"name":"poe1.build.get_build","path":"/poe1/build/:id","controller":"PoeSystemWeb.Poe1Controller","params":["id"],"action":"get_build","method":"GET"},{"name":"poe1.sse.subscribe","path":"/poe1/sse","controller":"PoeSystemWeb.SseController","params":[],"action":"subscribe","method":"POST"},{"name":"api.extract.extract","path":"/api/extract","controller":"PoeSystemWeb.Poe1Controller","params":[],"action":"extract","method":"POST"},{"name":"dev.dashboard.css-:md5.css","path":"/dev/dashboard/css-:md5","controller":"Phoenix.LiveDashboard.Assets","params":["md5"],"action":"css","method":"GET"},{"name":"dev.dashboard.js-:md5.js","path":"/dev/dashboard/js-:md5","controller":"Phoenix.LiveDashboard.Assets","params":["md5"],"action":"js","method":"GET"},{"name":"dev.dashboard.home","path":"/dev/dashboard","controller":"Phoenix.LiveView.Plug","params":[],"action":"home","method":"GET"},{"name":"dev.dashboard.page","path":"/dev/dashboard/:page","controller":"Phoenix.LiveView.Plug","params":["page"],"action":"page","method":"GET"},{"name":"dev.dashboard.page","path":"/dev/dashboard/:node/:page","controller":"Phoenix.LiveView.Plug","params":["node","page"],"action":"page","method":"GET"}];

  function replaceParams(path, params = {}) {
    let result = path;
    const routeParams = {...params};
    delete routeParams._query;

    // Keep track of used route parameters
    const usedParams = new Set();

    Object.keys(routeParams).forEach(key => {
      if (result.includes(`:${key}`)) {
        result = result.replace(`:${key}`, String(routeParams[key]));
        usedParams.add(key);
      }
    });

    const queryParams = {...params};
    const explicitQueryParams = queryParams._query || {};
    delete queryParams._query;

    // Remove used route parameters from query params
    usedParams.forEach(key => delete queryParams[key]);

    const allQueryParams = {...queryParams, ...explicitQueryParams};
    const queryString = Object.keys(allQueryParams).length
      ? '?' + new URLSearchParams(Object.fromEntries(
          Object.entries(allQueryParams).filter(([_, v]) => v != null)
        )).toString()
      : '';

    return result + queryString;
  }

  function route(name, params = {}) {
    const route = routes.find(r => r.name === name);
    if (!route) throw new Error(`Route '${name}' not found`);

    return replaceParams(route.path, params);
  }

  function path(name, params = {}) {
    return route(name, params);
  }

  function method(name) {
    const route = routes.find(r => r.name === name);
    if (!route) throw new Error(`Route '${name}' not found`);
    return route.method;
  }

  function hasRoute(name) {
    return routes.some(r => r.name === name);
  }

  return {
    routes,
    route,
    path,
    method,
    hasRoute,
    replaceParams
  };
})();

export default Routes;
