export type RouteString = string;

export const loginPage = (): RouteString => {
	return `/login`;
};

export const memomeshByDatePage = (routeDate: string): RouteString => {
	return `/memomesh/${routeDate}`;
};
