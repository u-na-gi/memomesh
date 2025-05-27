import type { LoginRequest, LoginResponse } from '$lib/types/src/api/auth/login/login';
import zod from 'zod';
import { Ok, Err, Result } from 'neverthrow';
import { LoginError, LogoutError } from './error';
import { goto } from '$app/navigation';

const loginRequestSchema = zod.object({
	username: zod.string().min(1, 'Username is required'),
	password: zod.string().min(1, 'Password is required')
});

const loginResponseSchema = zod.object({
	jwt: zod.string()
});

export const login = async (req: LoginRequest): Promise<Result<LoginResponse, LoginError>> => {
	try {
		// apiを叩く
		const parsedReq = loginRequestSchema.parse(req);
		// TODO: ドメインは環境変数定義
		const endpoint = 'http://localhost:8788/api/v1/auth/login';
		const result = await fetch(endpoint, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(parsedReq),
			credentials: 'include'
		});

		const data = await result.json();
		console.log('data', data);
		console.log('result', result);
		if (!result.ok) {
			return new Err(new LoginError('Login failed'));
		}

		const parsedData: LoginResponse = loginResponseSchema.parse(data);

		// ログイン状態を保存
		localStorage.setItem('isLoggedIn', 'true');
		localStorage.setItem('jwt', parsedData.jwt); // JWTを保存

		return new Ok(parsedData);
	} catch (error) {
		console.error('Login error:', error);
		return new Err(new LoginError('Login failed'));
	}
};

export const logout = async (): Promise<Result<void, LogoutError>> => {
	try {
		// ログアウトエンドポイントを呼び出し
		const endpoint = 'http://localhost:8788/api/v1/auth/logout';
		const result = await fetch(endpoint, {
			method: 'POST',
			credentials: 'include'
		});

		if (!result.ok) {
			return new Err(new LogoutError('Logout failed'));
		}

		// ログイン状態をクリア
		localStorage.removeItem('isLoggedIn');
		localStorage.removeItem('jwt');

		// ログイン画面にリダイレクト
		goto('/login');

		return new Ok(undefined);
	} catch (error) {
		console.error('Logout error:', error);
		return new Err(new LogoutError('Logout failed'));
	}
};
