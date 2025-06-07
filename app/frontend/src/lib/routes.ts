export type RouteString = string;

export const loginPage = (): RouteString => {
	return `/login`;
};

export const memomeshByDatePage = (routeDate: string): RouteString => {
	return `/memomesh/${routeDate}`;
};

export const memomeshByIdPage = (routeDate: string, id: string): RouteString => {
	return `/memomesh/${routeDate}/${id}`;
};
