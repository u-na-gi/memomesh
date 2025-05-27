export const getDateTime = () => {
	const timeZone = Intl.DateTimeFormat().resolvedOptions().timeZone;
	const now = new Date();

	// 日付部分
	const dateFormatter = new Intl.DateTimeFormat('en-CA', {
		timeZone,
		year: 'numeric',
		month: '2-digit',
		day: '2-digit'
	});

	// 時刻部分
	const timeFormatter = new Intl.DateTimeFormat('en-GB', {
		timeZone,
		hour: '2-digit',
		minute: '2-digit',
		second: '2-digit',
		hour12: false // 24時間表記
	});

	// 組み合わせる
	const date = dateFormatter.format(now); // "2025-04-28"
	const time = timeFormatter.format(now); // "13:45:30"

	const dateTime = `${date} ${time}`;

	return dateTime;
};

export const getUtcDateRange = (days: number): { start: string; end: string } => {
	const now = new Date();

	const startDate = new Date(Date.UTC(now.getUTCFullYear(), now.getUTCMonth(), now.getUTCDate()));
	startDate.setUTCDate(startDate.getUTCDate() - days + 1); // 過去n日分（今日含む）

	const format = (date: Date): string => {
		const year = date.getUTCFullYear();
		const month = String(date.getUTCMonth() + 1).padStart(2, '0');
		const day = String(date.getUTCDate()).padStart(2, '0');
		return `${year}-${month}-${day}`;
	};

	return {
		start: format(startDate),
		end: format(now)
	};
};
