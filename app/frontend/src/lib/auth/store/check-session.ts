import { Ok, Err, Result } from 'neverthrow';
import { SessionError } from '$lib/auth/store/error';

/**
 * セッションの有効性をチェックする関数
 * @returns セッションが有効な場合はOk、無効な場合はErrを返す
 */
export const session = async (): Promise<void> => {
	// TODO: ドメインは環境変数定義
	const endpoint = 'http://localhost:8788/api/v1/auth/session';

	try {
		console.log('Checking session...');
		const result = await fetch(endpoint, {
			method: 'GET',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json'
			}
		});

		console.log('Session check response:', result);

		if (!result.ok) {
			console.error('Session check failed with status:', result.status);
			// ローカルストレージのログイン状態をクリア
			localStorage.removeItem('isLoggedIn');
			localStorage.removeItem('jwt');
			throw new Error(`Session check failed with status: ${result.status}`);
		}

		// セッションが有効な場合、ログイン状態を保存
		localStorage.setItem('isLoggedIn', 'true');
		console.log('Session is valid, isLoggedIn set to true');

		// レスポンスデータを取得（必要に応じて）
		const data = await result.json().catch(() => ({}));
		console.log('Session data:', data);

		return;
	} catch (error) {
		console.error('Session check error:', error);
		// エラーを再スロー
		throw error;
	}
};

/**
 * セッションの有効性をチェックする関数（Result型を返す）
 * @returns セッションが有効な場合はOk、無効な場合はErrを返す
 */
export const checkSession = async (): Promise<Result<void, SessionError>> => {
	try {
		await session();
		return new Ok(undefined);
	} catch (error: unknown) {
		console.error('checkSession error:', error);
		const errorMessage = error instanceof Error ? error.message : 'Session check failed';
		return new Err(new SessionError(errorMessage));
	}
};
