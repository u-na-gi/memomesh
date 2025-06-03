import { CreateMemoRequest } from '$lib/types/src/api/memo/create';
import { CountNotesByDateList } from '$lib/types/src/api/memo/get_count_by_date';
import { env } from '$env/dynamic/public';
import { MemoList, type QueryParameters } from '$lib/types/src/api/memo/get_data';

export const getDataByDate = async (
	from_date: string,
	to_date: string
): Promise<CountNotesByDateList> => {
	const exdpoint = `${env.PUBLIC_API_BASE_URL}/memo/count-notes-by-date?from_date=${from_date}&to_date=${to_date}`;

	const result = await fetch(exdpoint, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${localStorage.getItem('jwt') || ''}`
		}
	});

	if (!result.ok) {
		throw new Error('Failed to fetch data');
	}
	console.log('Response status:', result.status);
	console.log('Response headers:', result.headers.get('Content-Type'));
	console.log('Response URL:', result.url);
	console.log('Response data', result);
	const data = await result.arrayBuffer();
	const uint8 = new Uint8Array(data);
	const decode = CountNotesByDateList.decode(uint8);
	console.log('Fetched data:', decode);

	return decode;
};

export const createMemo = async (reqest: CreateMemoRequest): Promise<void> => {
	const req = CreateMemoRequest.create(reqest);
	const bytes = CreateMemoRequest.encode(req).finish();
	const endpoint = 'http://0.0.0.0:8788/api/v1/memo/create';

	const result = await fetch(endpoint, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/octet-stream',
			Authorization: `Bearer ${localStorage.getItem('jwt') || ''}`
		},
		body: bytes.buffer as ArrayBuffer
	});

	if (!result.ok) {
		throw new Error('Failed to create memo');
	}
	console.log('Memo created successfully:', result.status);
};

export const getMemoByDate = async (query: QueryParameters): Promise<MemoList> => {
	const endpoint = `${env.PUBLIC_API_BASE_URL}/memo/data?date=${query.date}`;

	const result = await fetch(endpoint, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${localStorage.getItem('jwt') || ''}`
		}
	});

	if (!result.ok) {
		throw new Error('Failed to fetch memo by date');
	}
	console.log('Response status:', result.status);
	console.log('Response headers:', result.headers.get('Content-Type'));
	console.log('Response URL:', result.url);

	const data = await result.arrayBuffer();
	const uint8 = new Uint8Array(data);
	const decode = MemoList.decode(uint8);
	return decode;
};
