interface ResponseData<T> {
  data: T | null;
  error: string | null;
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

    if (!response.data) {
      throw new Error(rawResponse.status.toString());
    }

    return response.data;
  }

  static get<T>(url: string): Promise<T> {
    return Http.request(HttpMethods.GET, url);
  }

  static post<T, S>(url: string, body: T): Promise<S> {
    const jsonBody = JSON.stringify(body);
    return Http.request(HttpMethods.POST, url, jsonBody);
  }

  static postWithoutBody<T>(url: string): Promise<T> {
    return Http.request(HttpMethods.POST, url);
  }

  static patch<T, S>(url: string, body: T): Promise<S> {
    const jsonBody = JSON.stringify(body);
    return Http.request(HttpMethods.PATCH, url, jsonBody);
  }

  static delete<T>(url: string): Promise<T> {
    return Http.request(HttpMethods.DELETE, url);
  }
}

export default Http;
