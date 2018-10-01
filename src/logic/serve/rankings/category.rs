use std::mem::transmute;
use int_hash::IntHashMap;

use super::super::super::super::cache::cache;
use super::super::super::super::cache::cache::VoteCount;
use super::super::super::super::cache::cache::CategoryPeriodPollRankings;
use super::super::super::super::server::codes;
use super::super::super::super::data::byte_counts::ByteCounts;

// NOTE: max page size must fin into u16
const PAGE_SIZE: usize = 1024;

const INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES: usize =
// space for the leading header byte
    1 +
        // space for category cache index (if any
        4 +
        PAGE_SIZE +
        // space for category ids & vote counts
        PAGE_SIZE * (4 + 3) +
        // space for the byte counts
        PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

const INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES: usize =
// space for the leading header byte
    1 +
        // space for category cache index (if any
        4 +
        // space for category ids & vote counts
        PAGE_SIZE * (3 + 3)
        // space for the byte counts
        + PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

const INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES: usize =
// space for the leading header byte
    1 +
        // space for category cache index (if any
        4 +
        // space for category ids & vote counts
        PAGE_SIZE * (2 + 3)
        // space for the byte counts
        + PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

const INITIAL_RESPONSE_VECTOR_SIZE_1_POLL_BYTES: usize =
// space for the leading header byte
    1 +
        // space for category cache index (if any
        4 +
        // space for category ids & vote counts
        PAGE_SIZE * (1 + 3)
        // space for the byte counts
        + PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

pub fn get_todays_category_rankings_by_global_id(
    vcDayId: u32,
    blockIndex: u32,
    globalCategoryId: u64,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_CACHE_PERIOD_IDS.todaysVcDayId,
        vcDayId,
        cache::CATEGORY_TODAYS_INDEX_MAP,
        cache::TODAYS_CATEGORY_POLL_RANKINGS,
        globalCategoryId,
        blockIndex,
        cache::TODAYS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_todays_category_rankings_by_cache_index(
    vcDayId: u32,
    blockIndex: u32,
    categoryCacheIndex: u32,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_CACHE_PERIOD_IDS.todaysVcDayId,
        vcDayId,
        cache::TODAYS_CATEGORY_POLL_RANKINGS,
        categoryCacheIndex,
        blockIndex,
        cache::TODAYS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_yesterdays_category_rankings_by_global_id(
    vcDayId: u32,
    blockIndex: u32,
    globalCategoryId: u64,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_CACHE_PERIOD_IDS.yesterdaysVcDayId,
        vcDayId,
        cache::CATEGORY_YESTERDAYS_INDEX_MAP,
        cache::YESTERDAYS_CATEGORY_POLL_RANKINGS,
        globalCategoryId,
        blockIndex,
        cache::YESTERDAYS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_yesterdays_category_rankings_by_cache_index(
    vcDayId: u32,
    blockIndex: u32,
    categoryCacheIndex: u32,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_CACHE_PERIOD_IDS.yesterdaysVcDayId,
        vcDayId,
        cache::YESTERDAYS_CATEGORY_POLL_RANKINGS,
        categoryCacheIndex,
        blockIndex,
        cache::YESTERDAYS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_day_b4_yesterdays_category_rankings_by_global_id(
    vcDayId: u32,
    blockIndex: u32,
    globalCategoryId: u64,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_CACHE_PERIOD_IDS.dayB4YesterdaysVcDayId,
        vcDayId,
        cache::CATEGORY_DAY_B4_YESTERDAYS_INDEX_MAP,
        cache::DAY_B4_YESTERDAYS_CATEGORY_POLL_RANKINGS,
        globalCategoryId,
        blockIndex,
        cache::DAY_B4_YESTERDAYS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_day_b4_yesterdays_category_rankings_by_cache_index(
    vcDayId: u32,
    blockIndex: u32,
    categoryCacheIndex: u32,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_CACHE_PERIOD_IDS.dayB4YesterdaysVcDayId,
        vcDayId,
        cache::DAY_B4_YESTERDAYS_CATEGORY_POLL_RANKINGS,
        categoryCacheIndex,
        blockIndex,
        cache::DAY_B4_YESTERDAYS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_this_weeks_category_rankings_by_global_id(
    vcWeekId: u32,
    blockIndex: u32,
    globalCategoryId: u64,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_CACHE_PERIOD_IDS.thisWeeksVcWeekId,
        vcDayId,
        cache::CATEGORY_THIS_WEEKS_INDEX_MAP,
        cache::THIS_WEEKS_CATEGORY_POLL_RANKINGS,
        globalCategoryId,
        blockIndex,
        cache::THIS_WEEKS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_this_weeks_category_rankings_by_cache_index(
    vcWeekId: u32,
    blockIndex: u32,
    categoryCacheIndex: u32,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_CACHE_PERIOD_IDS.thisWeeksVcWeekId,
        vcDayId,
        cache::THIS_WEEKS_CATEGORY_POLL_RANKINGS,
        categoryCacheIndex,
        blockIndex,
        cache::THIS_WEEKS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_last_weeks_category_rankings_by_global_id(
    vcWeekId: u32,
    blockIndex: u32,
    globalCategoryId: u64,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_CACHE_PERIOD_IDS.lastWeeksVcWeekId,
        vcDayId,
        cache::CATEGORY_LAST_WEEKS_INDEX_MAP,
        cache::LAST_WEEKS_CATEGORY_POLL_RANKINGS,
        globalCategoryId,
        blockIndex,
        cache::LAST_WEEKS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_last_weeks_category_rankings_by_cache_index(
    vcWeekId: u32,
    blockIndex: u32,
    categoryCacheIndex: u32,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_CACHE_PERIOD_IDS.lastWeeksVcWeekId,
        vcDayId,
        cache::LAST_WEEKS_CATEGORY_POLL_RANKINGS,
        categoryCacheIndex,
        blockIndex,
        cache::LAST_WEEKS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_this_months_category_rankings_by_global_id(
    vcMonthId: u32,
    blockIndex: u32,
    globalCategoryId: u64,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_CACHE_PERIOD_IDS.thisMonthsVcMonthId,
        vcDayId,
        cache::CATEGORY_THIS_MONTHS_INDEX_MAP,
        cache::THIS_MONTHS_CATEGORY_POLL_RANKINGS,
        globalCategoryId,
        blockIndex,
        cache::THIS_MONTHS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_this_months_category_rankings_by_cache_index(
    vcMonthId: u32,
    blockIndex: u32,
    categoryCacheIndex: u32,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_CACHE_PERIOD_IDS.thisMonthsVcMonthId,
        vcDayId,
        cache::THIS_MONTHS_CATEGORY_POLL_RANKINGS,
        categoryCacheIndex,
        blockIndex,
        cache::THIS_MONTHS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_last_months_category_rankings_by_global_id(
    vcMonthId: u32,
    blockIndex: u32,
    globalCategoryId: u64,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_CACHE_PERIOD_IDS.lastMonthsVcMonthId,
        vcDayId,
        cache::CATEGORY_LAST_MONTHS_INDEX_MAP,
        cache::LAST_MONTHS_CATEGORY_POLL_RANKINGS,
        globalCategoryId,
        blockIndex,
        cache::LAST_MONTHS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_last_months_category_rankings_by_cache_index(
    vcMonthId: u32,
    blockIndex: u32,
    categoryCacheIndex: u32,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_CACHE_PERIOD_IDS.lastMonthsVcMonthId,
        vcDayId,
        cache::LAST_MONTHS_CATEGORY_POLL_RANKINGS,
        categoryCacheIndex,
        blockIndex,
        cache::LAST_MONTHS_POLL_ID_BYTE_COUNTS[38],
    );
}

fn get_category_rankings_by_global_id(
    currentPeriodId: u32,
    expectedPeriodId: u32,
    categoryIndexMap: IntHashMap<u64, u32>,
    givenPeriodCategoryPollRankings: Vec<Vec<VoteCount>>,
    globalCategoryId: u64,
    blockIndex: u32,
    maxPollNumberBytes: u8,
) -> Vec<u8> {
    if currentPeriodId != expectedPeriodId {
        return codes::INVALID_PERIOD_ID_RESPONSE;
    }

    let firstRecordIndex = PAGE_SIZE * blockIndex;

    match categoryIndexMap.get(&globalCategoryId) {
        None => {
            return codes::INVALID_GLOBAL_CATEGORY_ID_RESPONSE;
        }
        Some(categoryCacheIndex) => {
            return get_category_rankings_with_category_cache_index(
                firstRecordIndex, *categoryCacheIndex,
                givenPeriodCategoryPollRankings, maxPollNumberBytes);
        }
    }
}

fn get_category_rankings_by_cache_index(
    currentPeriodId: u32,
    expectedPeriodId: u32,
    givenPeriodCategoryPollRankings: Vec<Vec<VoteCount>>,
    categoryCacheIndex: u32,
    blockIndex: u32,
    maxPollNumberBytes: u8,
) -> Vec<u8> {
    if currentPeriodId != expectedPeriodId {
        return codes::INVALID_PERIOD_ID_RESPONSE;
    }

    let firstRecordIndex = PAGE_SIZE * blockIndex;

    match givenPeriodCategoryPollRankings.voteCountsByCategoryIndex.get(categoryCacheIndex) {
        None => {
            return codes::INVALID_CATEGORY_CACHE_INDEX_RESPONSE;
        }
        Some(_) => {
            return get_category_rankings(
                firstRecordIndex, categoryCacheIndex,
                givenPeriodCategoryPollRankings, maxPollNumberBytes);
        }
    }
}

#[inline]
fn get_category_rankings_with_category_cache_index(
    firstRecordIndex: usize,
    categoryCacheIndex: u32,
    givenPeriodCategoryPollRankings: Vec<Vec<VoteCount>>,
    maxPollNumberBytes: u8,
) -> Vec<u8> {
    let voteCountsForCategory = givenPeriodCategoryPollRankings[categoryCacheIndex];
    let categoryCacheIndexBytes: [u8; 4] = unsafe {
        std::mem::transmute(*categoryCacheIndex);
    };


    match maxPollNumberBytes {
        1 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_1_POLL_BYTES);
            response.push(0b00000001);
            response.extend_from_slice(&categoryCacheIndexBytes);

            return get1ByteRecentPolls(voteCountsForCategory, firstRecordIndex, response);
        }
        2 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES);
            response.push(0b00000010);
            response.extend_from_slice(&categoryCacheIndexBytes);

            return get2ByteRecentPolls(voteCountsForCategory, firstRecordIndex, response);
        }
        3 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES);
            response.push(0b00000011);
            response.extend_from_slice(&categoryCacheIndexBytes);

            return get3ByteRecentPolls(voteCountsForCategory, firstRecordIndex, response);
        }
        4 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES);
            response.push(0b00000000);
            response.extend_from_slice(&categoryCacheIndexBytes);

            return get4ByteRecentPolls(voteCountsForCategory, firstRecordIndex, response);
        }
    }
//            return codes::INVALID_CATEGORY_RESPONSE;
}

#[inline]
fn get_category_rankings(
    firstRecordIndex: usize,
    categoryIndex: u32,
    givenPeriodCategoryPollRankings: Vec<Vec<VoteCount>>,
    maxPollNumberBytes: u8,
) -> Vec<u8> {
    let voteCountsForCategory = givenPeriodCategoryPollRankings[categoryIndex];

    match maxPollNumberBytes {
        1 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_1_POLL_BYTES);
            response.push(0b00000001);
            return get1ByteRecentPolls(voteCountsForCategory, firstRecordIndex, response);
        }
        2 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES);
            response.push(0b00000010);
            return get2ByteRecentPolls(voteCountsForCategory, firstRecordIndex, response);
        }
        3 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES);
            response.push(0b00000011);
            return get3ByteRecentPolls(voteCountsForCategory, firstRecordIndex, response);
        }
        4 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES);
            response.push(0b00000000);
            return get4ByteRecentPolls(voteCountsForCategory, firstRecordIndex, response);
        }
    }
//            return codes::INVALID_CATEGORY_RESPONSE;
}

#[inline]
fn get4ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut voteCountsSizes = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.pollTypeAndTz);

                let tzAndPeriodPollIdBytes: [u8; 4] = unsafe {
                    // Poll Id in the period of a given time zone
                    std::mem::transmute(*voteCount.tzAndPeriodPollId);
                };
                response.extend_from_slice(&tzAndPeriodPollIdBytes);

                let countBytes: [u8; 4] = unsafe {
                    std::mem::transmute(*voteCount.count);
                };
                if countBytes[0] != 0 {
                    response.extend_from_slice(&countBytes);
                    voteCountsSizes.add4();
                } else if countBytes[1] != 0 {
                    response.extend_from_slice(&countBytes[1..3]);
                    voteCountsSizes.add3();
                } else if countBytes[2] != 0 {
                    response.extend_from_slice(&countBytes[2..3]);
                    voteCountsSizes.add2();
                } else {
                    response.push(countBytes[3]);
                    voteCountsSizes.add1();
                }
            }
        }
    }
    voteCountsSizes.append(response);

    return response;
}

#[inline]
fn get3ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut voteCountsSizes = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.pollTypeAndTz);

                let tzAndPeriodPollIdBytes: [u8; 4] = unsafe {
                    // Poll Id in the period of a given time zone
                    std::mem::transmute(*voteCount.tzAndPeriodPollId);
                };
                response.extend_from_slice(&tzAndPeriodPollIdBytes[1..3]);

                let countBytes: [u8; 4] = unsafe {
                    std::mem::transmute(*voteCount.count);
                };
                if countBytes[0] != 0 {
                    response.extend_from_slice(&countBytes);
                    voteCountsSizes.add4();
                } else if countBytes[1] != 0 {
                    response.extend_from_slice(&countBytes[1..3]);
                    voteCountsSizes.add3();
                } else if countBytes[2] != 0 {
                    response.extend_from_slice(&countBytes[2..3]);
                    voteCountsSizes.add2();
                } else {
                    response.push(countBytes[3]);
                    voteCountsSizes.add1();
                }
            }
        }
    }
    voteCountsSizes.append(response);

    return response;
}

#[inline]
fn get2ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut voteCountsSizes = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.pollTypeAndTz);

                let tzAndPeriodPollIdBytes: [u8; 4] = unsafe {
                    // Poll Id in the period of a given time zone
                    std::mem::transmute(*voteCount.tzAndPeriodPollId);
                };
                response.extend_from_slice(&tzAndPeriodPollIdBytes[2..3]);

                let countBytes: [u8; 4] = unsafe {
                    std::mem::transmute(*voteCount.count);
                };
                if countBytes[0] != 0 {
                    response.extend_from_slice(&countBytes);
                    voteCountsSizes.add4();
                } else if countBytes[1] != 0 {
                    response.extend_from_slice(&countBytes[1..3]);
                    voteCountsSizes.add3();
                } else if countBytes[2] != 0 {
                    response.extend_from_slice(&countBytes[2..3]);
                    voteCountsSizes.add2();
                } else {
                    response.push(countBytes[3]);
                    voteCountsSizes.add1();
                }
            }
        }
    }
    voteCountsSizes.append(response);

    return response;
}

#[inline]
fn get1ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut voteCountsSizes = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.pollTypeAndTz);

                let tzAndPeriodPollIdBytes: [u8; 4] = unsafe {
                    // Poll Id in the period of a given time zone
                    std::mem::transmute(*voteCount.tzAndPeriodPollId);
                };
                response.push(tzAndPeriodPollIdBytes[3]);

                let countBytes: [u8; 4] = unsafe {
                    std::mem::transmute(*voteCount.count);
                };
                if countBytes[0] != 0 {
                    response.extend_from_slice(&countBytes);
                    voteCountsSizes.add4();
                } else if countBytes[1] != 0 {
                    response.extend_from_slice(&countBytes[1..3]);
                    voteCountsSizes.add3();
                } else if countBytes[2] != 0 {
                    response.extend_from_slice(&countBytes[2..3]);
                    voteCountsSizes.add2();
                } else {
                    response.push(countBytes[3]);
                    voteCountsSizes.add1();
                }
            }
        }
    }
    voteCountsSizes.append(response);

    return response;
}
