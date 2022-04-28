import { serverBaseUrl } from '../constants';

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
  private static async request<T>(
    method: HttpMethods,
    url: string,
    body?: string,
    accessToken?: string,
  ): Promise<T> {
    const rawResponse: Response = await fetch(url, {
      method,
      headers: this.composeHeaders(accessToken),
      body,
      credentials: 'include',
    });

    const response: ResponseData<T> = await rawResponse.json();

    if (!response.data) {
      throw new Error(rawResponse.status.toString());
    }

    return response.data;
  }

  static async get<T>(url: string, accessToken: string): Promise<T> {
    try {
      return await Http.request(HttpMethods.GET, url, undefined, accessToken);
    } catch(e) {
      if (e instanceof Error && e.message === '401') {
        const refreshedAccessToken = await Http.refreshAccessToken();
        return await Http.get(url, refreshedAccessToken)
      } else {
        throw e;
      }
    }
  }

  static async post<T, S>(url: string, body: T, accessToken?: string): Promise<S> {
    try {
      const jsonBody = JSON.stringify(body);
      return await Http.request(HttpMethods.POST, url, jsonBody, accessToken);
    } catch(e) {
      if (accessToken && e instanceof Error && e.message === '401') {
        const refreshedAccessToken = await Http.refreshAccessToken();
        return await Http.post(url, body, refreshedAccessToken)
      } else {
        throw e;
      }
    }
  }

  static async postWithoutBody<T>(url: string, accessToken?: string): Promise<T> {
    try {
      return await Http.request(HttpMethods.POST, url, undefined, accessToken);
    } catch(e) {
      if (accessToken && e instanceof Error && e.message === '401') {
        const refreshedAccessToken = await Http.refreshAccessToken();
        return await Http.postWithoutBody(url, refreshedAccessToken)
      } else {
        throw e;
      }
    }
  }

  static async patch<T, S>(url: string, body: T, accessToken: string): Promise<S> {
    try {
      const jsonBody = JSON.stringify(body);
      return await Http.request(HttpMethods.PATCH, url, jsonBody, accessToken);
    } catch(e) {
      if (accessToken && e instanceof Error && e.message === '401') {
        const refreshedAccessToken = await Http.refreshAccessToken();
        return await Http.patch(url, body, refreshedAccessToken)
      } else {
        throw e;
      }
    }
  }

  static async delete<T>(url: string, accessToken?: string): Promise<T> {
    try {
      return await Http.request(HttpMethods.DELETE, url, undefined, accessToken);
    } catch(e) {
      if (accessToken && e instanceof Error && e.message === '401') {
        const refreshedAccessToken = await Http.refreshAccessToken();
        return await Http.delete(url, refreshedAccessToken)
      } else {
        throw e;
      }
    }
  }

  private static async refreshAccessToken(): Promise<string> {
    const url = `${serverBaseUrl}/auth/token/access`;
    return Http.postWithoutBody<string>(url);
  }

  private static composeHeaders(accessToken?: string): HeadersInit {
    const headers: HeadersInit = {
      'Content-Type': 'application/json',
      'Access-Control-Allow-Credentials': 'true',
    };

    if (accessToken) {
      headers['Authorization'] = `Bearer ${accessToken}`;
    }

    return headers;
  }

}

export default Http;
