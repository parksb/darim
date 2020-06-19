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
  static baseUrl = 'http://127.0.0.1:8080';

  private static async request<T>(
    method: HttpMethods,
    url: string,
    body?: string,
  ): Promise<T> {
    const rawResponse: Response = await fetch(url, {
      method,
      headers: {
        'Content-Type': 'application/json',
        'Access-Control-Allow-Credentials': 'true',
      },
      body,
      credentials: 'include',
    });
    const response: ResponseData<T> = await rawResponse.json();
    return response.data;
  }

  static get<T>(url: string): Promise<T> {
    return Http.request(HttpMethods.GET, url);
  }

  static post<T, S>(url: string, body: T): Promise<S> {
    const jsonBody = JSON.stringify(body);
    return Http.request(HttpMethods.POST, url, jsonBody);
  }

  static patch<T, S>(url: string, body: T): Promise<S> {
    const jsonBody = JSON.stringify(body);
    return Http.request(HttpMethods.PATCH, url, jsonBody);
  }
}

export default Http;
