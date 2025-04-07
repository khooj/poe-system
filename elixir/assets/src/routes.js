const Routes = (function() {
  const routes = [{"name":"index","path":"/","controller":"PoeSystemWeb.IndexController","params":[],"action":"index","method":"GET"},{"name":"poe1.index","path":"/poe1","controller":"PoeSystemWeb.Poe1Controller","params":[],"action":"index","method":"GET"},{"name":"poe1.new.new","path":"/poe1/new/:id","controller":"PoeSystemWeb.Poe1Controller","params":["id"],"action":"new","method":"POST"},{"name":"poe1.extract.extract","path":"/poe1/extract","controller":"PoeSystemWeb.Poe1Controller","params":[],"action":"extract","method":"POST"},{"name":"poe1.preview.preview","path":"/poe1/preview/:id","controller":"PoeSystemWeb.Poe1Controller","params":["id"],"action":"preview","method":"GET"},{"name":"poe1.preview.update_preview","path":"/poe1/preview","controller":"PoeSystemWeb.Poe1Controller","params":[],"action":"update_preview","method":"PATCH"},{"name":"poe1.build.get_build","path":"/poe1/build/:id","controller":"PoeSystemWeb.Poe1Controller","params":["id"],"action":"get_build","method":"GET"}];

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

if (typeof module !== 'undefined' && module.exports) {
  module.exports = Routes;
} else {
  window.Routes = Routes;
}
