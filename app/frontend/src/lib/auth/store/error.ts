export class AuthError extends Error {
	constructor(message: string) {
		super(message);
		this.name = 'AuthError';
	}
}

export class LoginError extends AuthError {
	constructor(message: string) {
		super(message);
		this.name = 'LoginError';
	}
}

export class LogoutError extends AuthError {
	constructor(message: string) {
		super(message);
		this.name = 'LogoutError';
	}
}

export class SessionError extends AuthError {
	constructor(message: string) {
		super(message);
		this.name = 'SessionError';
	}
}
