/* eslint-disable */
/* tslint:disable */
/*
 * ---------------------------------------------------------------
 * ## THIS FILE WAS GENERATED VIA SWAGGER-TYPESCRIPT-API        ##
 * ##                                                           ##
 * ## AUTHOR: acacode                                           ##
 * ## SOURCE: https://github.com/acacode/swagger-typescript-api ##
 * ---------------------------------------------------------------
 */

export interface ChangeAvatarRequest {
  /** @format binary */
  avatar: File;
}

export interface ChangeAvatarResponse {
  /** @example "http://127.0.0.1:3335/files/conexipro-dev/72e1f1ea-7958-484b-97c2-b91d842e60c8.png" */
  avatar: string;
}

export interface ChangeUsernameRequest {
  /** @example "conexipro" */
  username: string | null;
}

export interface ChangeUsernameResponse {
  /** @example "username updated successfully" */
  message: string;
  /** @example "true" */
  success: boolean;
}

export interface Info {
  /** @example "facebook" */
  info_key: string | null;
  /** @example "contact" */
  info_type?: string | null;
  /** @example "https://facebook.com/zinkyaw" */
  info_value: string | null;
}

/** Token */
export interface JwtToken {
  /** @format date-time */
  expired_date: string;
  /** @example "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..." */
  token: string;
  /** @example "Bearer" */
  token_type: string;
}

/** User Modal */
export interface User {
  /** @example "http://127.0.0.1:3335/files/conexipro-dev/72e1f1ea-7958-484b-97c2-b91d842e60c8.png" */
  avatar?: string | null;
  /** @format date-time */
  created_at?: string | null;
  /** @example "AJ" */
  display_name?: string | null;
  /** @example "aj.zinkyaw@gmail.com" */
  email?: string | null;
  /** @example "Zin Kyaw" */
  first_name: string;
  /** @example "Kyaw" */
  last_name?: string | null;
  /** @example "" */
  middle_name?: string | null;
  /** @example "UVsflAVCCSR1aaB1dzIh1TPdPG63" */
  uid: string;
  /** @format date-time */
  updated_at?: string | null;
  /** @example "zinkyawkyaw" */
  username: string;
}

export type UserDetailResponse = User & {
  /** @example "[{ "info_key": "facebook", "info_value": "https://facebook.com/zinkyaw", "info_type": "contact" }]" */
  infos: UserInfo[];
};

/** User Modal */
export interface UserInfo {
  /** @example "phone" */
  info_key: string;
  /** @example "contact" */
  info_type?: string | null;
  /** @example "+66620350322" */
  info_value?: string | null;
}

export interface UserInfoUpdateRequest {
  infos?: Info[] | null;
}

export interface UserLoginRequest {
  /** @example "eyJhbGciOiJSUzI1NiIsImtpZCI6ImYwOGU2Z..." */
  token: string | null;
}

export interface UserLoginResponse {
  success: boolean;
  token: JwtToken;
  /** User Modal */
  user: User;
}

export interface UserUpdateRequest {
  /** @example "AJ" */
  display_name?: string | null;
  /** @example "Zin Kyaw" */
  first_name: string | null;
  /** @example "Kyaw" */
  last_name?: string | null;
  /** @example "" */
  middle_name?: string | null;
}

export type QueryParamsType = Record<string | number, any>;
export type ResponseFormat = keyof Omit<Body, "body" | "bodyUsed">;

export interface FullRequestParams extends Omit<RequestInit, "body"> {
  /** set parameter to `true` for call `securityWorker` for this request */
  secure?: boolean;
  /** request path */
  path: string;
  /** content type of request body */
  type?: ContentType;
  /** query params */
  query?: QueryParamsType;
  /** format of response (i.e. response.json() -> format: "json") */
  format?: ResponseFormat;
  /** request body */
  body?: unknown;
  /** base url */
  baseUrl?: string;
  /** request cancellation token */
  cancelToken?: CancelToken;
}

export type RequestParams = Omit<FullRequestParams, "body" | "method" | "query" | "path">;

export interface ApiConfig<SecurityDataType = unknown> {
  baseUrl?: string;
  baseApiParams?: Omit<RequestParams, "baseUrl" | "cancelToken" | "signal">;
  securityWorker?: (securityData: SecurityDataType | null) => Promise<RequestParams | void> | RequestParams | void;
  customFetch?: typeof fetch;
}

export interface HttpResponse<D extends unknown, E extends unknown = unknown> extends Response {
  data: D;
  error: E;
}

type CancelToken = Symbol | string | number;

export enum ContentType {
  Json = "application/json",
  FormData = "multipart/form-data",
  UrlEncoded = "application/x-www-form-urlencoded",
  Text = "text/plain",
}

export class HttpClient<SecurityDataType = unknown> {
  public baseUrl: string = "{BaseUrl}";
  private securityData: SecurityDataType | null = null;
  private securityWorker?: ApiConfig<SecurityDataType>["securityWorker"];
  private abortControllers = new Map<CancelToken, AbortController>();
  private customFetch = (...fetchParams: Parameters<typeof fetch>) => fetch(...fetchParams);

  private baseApiParams: RequestParams = {
    credentials: "same-origin",
    headers: {},
    redirect: "follow",
    referrerPolicy: "no-referrer",
  };

  constructor(apiConfig: ApiConfig<SecurityDataType> = {}) {
    Object.assign(this, apiConfig);
  }

  public setSecurityData = (data: SecurityDataType | null) => {
    this.securityData = data;
  };

  protected encodeQueryParam(key: string, value: any) {
    const encodedKey = encodeURIComponent(key);
    return `${encodedKey}=${encodeURIComponent(typeof value === "number" ? value : `${value}`)}`;
  }

  protected addQueryParam(query: QueryParamsType, key: string) {
    return this.encodeQueryParam(key, query[key]);
  }

  protected addArrayQueryParam(query: QueryParamsType, key: string) {
    const value = query[key];
    return value.map((v: any) => this.encodeQueryParam(key, v)).join("&");
  }

  protected toQueryString(rawQuery?: QueryParamsType): string {
    const query = rawQuery || {};
    const keys = Object.keys(query).filter((key) => "undefined" !== typeof query[key]);
    return keys
      .map((key) => (Array.isArray(query[key]) ? this.addArrayQueryParam(query, key) : this.addQueryParam(query, key)))
      .join("&");
  }

  protected addQueryParams(rawQuery?: QueryParamsType): string {
    const queryString = this.toQueryString(rawQuery);
    return queryString ? `?${queryString}` : "";
  }

  private contentFormatters: Record<ContentType, (input: any) => any> = {
    [ContentType.Json]: (input: any) =>
      input !== null && (typeof input === "object" || typeof input === "string") ? JSON.stringify(input) : input,
    [ContentType.Text]: (input: any) => (input !== null && typeof input !== "string" ? JSON.stringify(input) : input),
    [ContentType.FormData]: (input: any) =>
      Object.keys(input || {}).reduce((formData, key) => {
        const property = input[key];
        formData.append(
          key,
          property instanceof Blob
            ? property
            : typeof property === "object" && property !== null
              ? JSON.stringify(property)
              : `${property}`,
        );
        return formData;
      }, new FormData()),
    [ContentType.UrlEncoded]: (input: any) => this.toQueryString(input),
  };

  protected mergeRequestParams(params1: RequestParams, params2?: RequestParams): RequestParams {
    return {
      ...this.baseApiParams,
      ...params1,
      ...(params2 || {}),
      headers: {
        ...(this.baseApiParams.headers || {}),
        ...(params1.headers || {}),
        ...((params2 && params2.headers) || {}),
      },
    };
  }

  protected createAbortSignal = (cancelToken: CancelToken): AbortSignal | undefined => {
    if (this.abortControllers.has(cancelToken)) {
      const abortController = this.abortControllers.get(cancelToken);
      if (abortController) {
        return abortController.signal;
      }
      return void 0;
    }

    const abortController = new AbortController();
    this.abortControllers.set(cancelToken, abortController);
    return abortController.signal;
  };

  public abortRequest = (cancelToken: CancelToken) => {
    const abortController = this.abortControllers.get(cancelToken);

    if (abortController) {
      abortController.abort();
      this.abortControllers.delete(cancelToken);
    }
  };

  public request = async <T = any, E = any>({
    body,
    secure,
    path,
    type,
    query,
    format,
    baseUrl,
    cancelToken,
    ...params
  }: FullRequestParams): Promise<HttpResponse<T, E>> => {
    const secureParams =
      ((typeof secure === "boolean" ? secure : this.baseApiParams.secure) &&
        this.securityWorker &&
        (await this.securityWorker(this.securityData))) ||
      {};
    const requestParams = this.mergeRequestParams(params, secureParams);
    const queryString = query && this.toQueryString(query);
    const payloadFormatter = this.contentFormatters[type || ContentType.Json];
    const responseFormat = format || requestParams.format;

    return this.customFetch(`${baseUrl || this.baseUrl || ""}${path}${queryString ? `?${queryString}` : ""}`, {
      ...requestParams,
      headers: {
        ...(requestParams.headers || {}),
        ...(type && type !== ContentType.FormData ? { "Content-Type": type } : {}),
      },
      signal: (cancelToken ? this.createAbortSignal(cancelToken) : requestParams.signal) || null,
      body: typeof body === "undefined" || body === null ? null : payloadFormatter(body),
    }).then(async (response) => {
      const r = response.clone() as HttpResponse<T, E>;
      r.data = null as unknown as T;
      r.error = null as unknown as E;

      const data = !responseFormat
        ? r
        : await response[responseFormat]()
            .then((data) => {
              if (r.ok) {
                r.data = data;
              } else {
                r.error = data;
              }
              return r;
            })
            .catch((e) => {
              r.error = e;
              return r;
            });

      if (cancelToken) {
        this.abortControllers.delete(cancelToken);
      }

      if (!response.ok) throw data;
      return data;
    });
  };
}

/**
 * @title conexipro
 * @version 0.1.0
 * @license
 * @baseUrl {BaseUrl}
 */
export class Api<SecurityDataType extends unknown> extends HttpClient<SecurityDataType> {
  auth = {
    /**
     * No description
     *
     * @tags Auth
     * @name Login
     * @summary User login or registraion using firebase token
     * @request POST:/auth/login
     */
    login: (data: UserLoginRequest, params: RequestParams = {}) =>
      this.request<UserLoginResponse, any>({
        path: `/auth/login`,
        method: "POST",
        body: data,
        type: ContentType.Json,
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @tags Auth
     * @name RefreshUserToken
     * @summary Refresh user new token
     * @request GET:/auth/refresh
     * @secure
     */
    refreshUserToken: (params: RequestParams = {}) =>
      this.request<UserLoginResponse, any>({
        path: `/auth/refresh`,
        method: "GET",
        secure: true,
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @tags Auth
     * @name FetchUser
     * @summary Fetch user information
     * @request GET:/auth/user
     * @secure
     */
    fetchUser: (params: RequestParams = {}) =>
      this.request<UserDetailResponse, any>({
        path: `/auth/user`,
        method: "GET",
        secure: true,
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @tags User
     * @name UpdateUser
     * @summary update user information
     * @request POST:/auth/user
     * @secure
     */
    updateUser: (data: UserUpdateRequest, params: RequestParams = {}) =>
      this.request<User, any>({
        path: `/auth/user`,
        method: "POST",
        body: data,
        secure: true,
        type: ContentType.Json,
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @tags User
     * @name ChangeAvatar
     * @summary change user avatar image
     * @request PUT:/auth/user/avatar
     * @secure
     */
    changeAvatar: (data: ChangeAvatarRequest, params: RequestParams = {}) =>
      this.request<ChangeAvatarResponse, any>({
        path: `/auth/user/avatar`,
        method: "PUT",
        body: data,
        secure: true,
        type: ContentType.FormData,
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @tags User
     * @name UpdateUserInfos
     * @summary update user informations
     * @request POST:/auth/user/infos
     * @secure
     */
    updateUserInfos: (data: UserInfoUpdateRequest, params: RequestParams = {}) =>
      this.request<UserDetailResponse, any>({
        path: `/auth/user/infos`,
        method: "POST",
        body: data,
        secure: true,
        type: ContentType.Json,
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @tags User
     * @name ChangeUsername
     * @summary change username
     * @request PUT:/auth/user/username
     * @secure
     */
    changeUsername: (data: ChangeUsernameRequest, params: RequestParams = {}) =>
      this.request<ChangeUsernameResponse, any>({
        path: `/auth/user/username`,
        method: "PUT",
        body: data,
        secure: true,
        type: ContentType.Json,
        format: "json",
        ...params,
      }),
  };
}
