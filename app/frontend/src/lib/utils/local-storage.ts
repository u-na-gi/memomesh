const isSsr = () => typeof window !== 'undefined';

export const getItemFromLocalStorage = (key: string): string => {
	if (!isSsr()) {
		return '';
	}
	const result = localStorage.getItem(key);
	if (result === null) {
		return '';
	}
	return result;
};

export const setItemToLocalStorage = (key: string, value: string): void => {
	if (!isSsr()) {
		return;
	}
	localStorage.setItem(key, value);
};
