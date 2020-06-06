interface ResponseData<T> {
  data: T;
}

enum HttpMethods {
  GET = 'GET',
  POST = 'POST',
  PATCH = 'PATCH',
  DELETE = 'DELETE',
}

class Http {
  private static async request<T>(
    method: HttpMethods,
    url: string,
    body?: string,
  ): Promise<T> {
    const raw_response: Response = await fetch(url, {
      method,
      headers: {
        'Content-Type': 'application/json',
        'Access-Control-Allow-Credentials': 'true',
      },
      body,
      credentials: 'include',
    });
    const response: ResponseData<T> = await raw_response.json();
    return response.data;
  }

  static get<T>(url: string): Promise<T> {
    return Http.request(HttpMethods.GET, url);
  }

  static post<T, S>(url: string, body: T): Promise<S> {
    const json_body = JSON.stringify(body);
    return Http.request(HttpMethods.POST, url, json_body);
  }

  static patch<T, S>(url: string, body: T): Promise<S> {
    const json_body = JSON.stringify(body);
    return Http.request(HttpMethods.PATCH, url, json_body);
  }
}

export default Http;
