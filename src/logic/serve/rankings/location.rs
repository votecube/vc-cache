use std::mem::transmute;
use int_hash::IntHashMap;

use super::super::super::super::server::codes;
use super::super::super::super::cache::cache;
use super::super::super::super::cache::cache::VoteCount;
use super::super::super::super::cache::cache::LocationPollRankings;
use super::super::super::super::data::byte_counts::ByteCounts;

use super::location_and_loc_category::get1ByteRecentPolls;
use super::location_and_loc_category::get2ByteRecentPolls;
use super::location_and_loc_category::get3ByteRecentPolls;
use super::location_and_loc_category::get4ByteRecentPolls;

pub fn get_todays_location_rankings_by_global_id(
    vcDayId: u32,
    timezoneId: u32,
    blockIndex: u32,
    globalLocationId: u64,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
        None => {
            return codes::INVALID_TIMEZONE_ID_RESPONSE;
        }
        Some(cachePeriodIds) => {
            cachePeriodIds
        }
    };

    return get_location_rankings_by_global_id(
        currentPeriodIds.todaysVcDayId,
        vcDayId,
        timezoneId,
        cache::LOCATION_TODAYS_INDEX_MAP,
        cache::TODAYS_LOCATION_POLL_RANKINGS,
        globalLocationId,
        blockIndex,
        cache::TODAYS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_todays_location_rankings_by_cache_index(
    vcDayId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_rankings_by_cache_index(
        currentPeriodIds.todaysVcDayId,
        vcDayId,
        timezoneId,
        cache::TODAYS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        blockIndex,
        cache::TODAYS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_yesterdays_location_rankings_by_global_id(
    vcDayId: u32,
    timezoneId: u32,
    blockIndex: u32,
    globalLocationId: u64,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_rankings_by_global_id(
        currentPeriodIds.yesterdaysVcDayId,
        vcDayId,
        timezoneId,
        cache::LOCATION_YESTERDAYS_INDEX_MAP,
        cache::YESTERDAYS_LOCATION_POLL_RANKINGS,
        globalLocationId,
        blockIndex,
        cache::YESTERDAYS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_yesterdays_location_rankings_by_cache_index(
    vcDayId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_rankings_by_cache_index(
        currentPeriodIds.yesterdaysVcDayId,
        vcDayId,
        timezoneId,
        cache::YESTERDAYS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        blockIndex,
        cache::YESTERDAYS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_day_b4_yesterdays_location_rankings_by_global_id(
    vcDayId: u32,
    timezoneId: u32,
    blockIndex: u32,
    globalLocationId: u64,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_rankings_by_global_id(
        currentPeriodIds.dayB4YesterdaysVcDayId,
        vcDayId,
        timezoneId,
        cache::LOCATION_DAY_B4_YESTERDAYS_INDEX_MAP,
        cache::DAY_B4_YESTERDAYS_LOCATION_POLL_RANKINGS,
        globalLocationId,
        blockIndex,
        cache::DAY_B4_YESTERDAYS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_day_b4_yesterdays_location_rankings_by_cache_index(
    vcDayId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_rankings_by_cache_index(
        currentPeriodIds.dayB4YesterdaysVcDayId,
        vcDayId,
        timezoneId,
        cache::DAY_B4_YESTERDAYS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        blockIndex,
        cache::DAY_B4_YESTERDAYS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_this_weeks_location_rankings_by_global_id(
    vcWeekId: u32,
    timezoneId: u32,
    blockIndex: u32,
    globalLocationId: u64,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_rankings_by_global_id(
        currentPeriodIds.thisWeeksVcWeekId,
        vcDayId,
        timezoneId,
        cache::LOCATION_THIS_WEEKS_INDEX_MAP,
        cache::THIS_WEEKS_LOCATION_POLL_RANKINGS,
        globalLocationId,
        blockIndex,
        cache::THIS_WEEKS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_this_weeks_location_rankings_by_cache_index(
    vcWeekId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_rankings_by_cache_index(
        currentPeriodIds.thisWeeksVcWeekId,
        vcDayId,
        timezoneId,
        cache::THIS_WEEKS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        blockIndex,
        cache::THIS_WEEKS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_last_weeks_location_rankings_by_global_id(
    vcWeekId: u32,
    timezoneId: u32,
    blockIndex: u32,
    globalLocationId: u64,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_rankings_by_global_id(
        currentPeriodIds.lastWeeksVcWeekId,
        vcDayId,
        timezoneId,
        cache::LOCATION_LAST_WEEKS_INDEX_MAP,
        cache::LAST_WEEKS_LOCATION_POLL_RANKINGS,
        globalLocationId,
        blockIndex,
        cache::LAST_WEEKS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_last_weeks_location_rankings_by_cache_index(
    vcWeekId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_rankings_by_cache_index(
        currentPeriodIds.lastWeeksVcWeekId,
        vcDayId,
        timezoneId,
        cache::LAST_WEEKS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        blockIndex,
        cache::LAST_WEEKS_LOCATION_POLL_RANKINGS[timezoneId],
    );
}

pub fn get_this_months_location_rankings_by_global_id(
    vcMonthId: u32,
    timezoneId: u32,
    blockIndex: u32,
    globalLocationId: u64,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_rankings_by_global_id(
        currentPeriodIds.thisMonthsVcMonthId,
        vcDayId,
        timezoneId,
        cache::LOCATION_THIS_MONTHS_INDEX_MAP,
        cache::THIS_MONTHS_LOCATION_POLL_RANKINGS,
        globalLocationId,
        blockIndex,
        cache::THIS_MONTHS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_this_months_location_rankings_by_cache_index(
    vcMonthId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_rankings_by_cache_index(
        currentPeriodIds.thisMonthsVcMonthId,
        vcDayId,
        timezoneId,
        cache::THIS_MONTHS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        blockIndex,
        cache::THIS_MONTHS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_last_months_location_rankings_by_global_id(
    vcMonthId: u32,
    timezoneId: u32,
    blockIndex: u32,
    globalLocationId: u64,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_rankings_by_global_id(
        currentPeriodIds.lastMonthsVcMonthId,
        vcDayId,
        timezoneId,
        cache::LOCATION_LAST_MONTHS_INDEX_MAP,
        cache::LAST_MONTHS_LOCATION_POLL_RANKINGS,
        globalLocationId,
        blockIndex,
        cache::LAST_MONTHS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_last_months_location_rankings_by_cache_index(
    vcMonthId: u32,
    timezoneId: u32,
    blockIndex: u32,
    locationCacheIndex: u32,
) -> Vec<u8> {
    let currentPeriodIds: cache::CachePeriodIds =
        match cache::PER_TIMEZONE__CACHE_PERIOD_IDS(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(cachePeriodIds) => {
                cachePeriodIds
            }
        };

    return get_location_rankings_by_cache_index(
        currentPeriodIds.lastMonthsVcMonthId,
        vcDayId,
        timezoneId,
        cache::LAST_MONTHS_LOCATION_POLL_RANKINGS,
        locationCacheIndex,
        blockIndex,
        cache::LAST_MONTHS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

fn get_location_rankings_by_global_id(
    currentPeriodId: u32,
    expectedPeriodId: u32,
    timezoneId: u32,
    locationIndexMap: IntHashMap<u64, cache::LocationPeriodIds>,
    givenPeriodLocationPollRankings: Vec<Vec<LocationPollRankings>>,
    globalLocationId: u64,
    blockIndex: u32,
    maxPollNumberBytes: u8,
) -> Vec<u8> {
    if currentPeriodId != expectedPeriodId {
        return codes::INVALID_PERIOD_ID_RESPONSE;
    }

    let locationPeriodIds: cache::LocationPeriodIds = match locationIndexMap.get(*globalLocationId) {
        None => {
            return codes::INVALID_GLOBAL_LOCATION_ID_RESPONSE;
        }
        Some(locationPeriodIds) => {
            locationPeriodIds
        }
    };

    let locationCacheIndex: u32 = locationPeriodIds.locationCacheIndex;
    let locationPollRankings = givenPeriodLocationPollRankings[timezoneId][locationCacheIndex];
    let firstRecordIndex = PAGE_SIZE * blockIndex;

    return get_location_rankings_with_location_cache_index(
        timezoneId, firstRecordIndex, locationCacheIndex,
        locationPollRankings, maxPollNumberBytes);
}

fn get_location_rankings_by_cache_index(
    currentPeriodId: u32,
    expectedPeriodId: u32,
    timezoneId: u32,
    givenPeriodLocationPollRankings: Vec<Vec<LocationPollRankings>>,
    locationCacheIndex: u32,
    blockIndex: u32,
    maxPollNumberBytes: u8,
) -> Vec<u8> {
    if currentPeriodId != expectedPeriodId {
        return codes::INVALID_PERIOD_ID_RESPONSE;
    }

    let locationPollRankings: LocationPollRankings
    = match givenPeriodLocationPollRankings[timezoneId].get(locationCacheIndex) {
        None => {
            return codes::INVALID_LOCATION_CACHE_INDEX_RESPONSE;
        }
        Some(locationPollRankings) => {
            locationPollRankings
        }
    };

    let firstRecordIndex = PAGE_SIZE * blockIndex;

    return get_location_rankings(
        timezoneId, firstRecordIndex, categoryCacheIndex,
        locationPollRankings, maxPollNumberBytes);
}

#[inline]
fn get_location_rankings_with_location_cache_index(
    timezoneId: u32,
    firstRecordIndex: usize,
    locationCacheIndex: u32,
    locationPollRankings: LocationPollRankings,
    maxPollNumberBytes: u8,
) -> Vec<u8> {
    let voteCountsForLocation = locationPollRankings.location;
    let locationCacheIndexBytes: [u8; 4] = unsafe {
        std::mem::transmute(*locationCacheIndex);
    };

    match maxPollNumberBytes {
        1 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_1_POLL_BYTES);
            response.push(0b00000001);
            response.extend_from_slice(&locationCacheIndexBytes);

            return get1ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
        2 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES);
            response.push(0b00000010);
            response.extend_from_slice(&locationCacheIndexBytes);

            return get2ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
        3 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES);
            response.push(0b00000011);
            response.extend_from_slice(&locationCacheIndexBytes);

            return get3ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
        4 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES);
            response.push(0b00000000);
            response.extend_from_slice(&locationCacheIndexBytes);

            return get4ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
    }
//            return codes::INVALID_CATEGORY_RESPONSE;
}

#[inline]
fn get_location_rankings(
    timezoneId: u32,
    firstRecordIndex: usize,
    locationCacheIndex: u32,
    locationPollRankings: LocationPollRankings,
    maxPollNumberBytes: u8,
) -> Vec<u8> {
    let voteCountsForLocation = locationPollRankings.location;

    match maxPollNumberBytes {
        1 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_1_POLL_BYTES);
            response.push(0b00000001);

            return get1ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
        2 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES);
            response.push(0b00000010);

            return get2ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
        3 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES);
            response.push(0b00000011);

            return get3ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
        4 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES);
            response.push(0b00000000);

            return get4ByteRecentPolls(*voteCountsForLocation, firstRecordIndex, response);
        }
    }
//            return codes::INVALID_CATEGORY_RESPONSE;
}
