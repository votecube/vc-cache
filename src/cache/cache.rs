use super::super::data::prepend::list::PrependList;
use int_hash::IntHashMap;
use std::collections::HashMap;

pub static mut data: Vec<u32> = Vec::new();

/**
 * Global time period ids across timezones, maintained at the same time as data is moved
 * in timezone chunks between, current and past (and future).
 *
 * Used to verify client requests, to make sure that their requests are still valid.
 */
pub static mut lastMonthIds: [u32; 38] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut thisMonthIds: [u32; 38] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut nextMonthIds: [u32; 38] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut lastWeekIds: [u32; 38] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut thisWeekIds: [u32; 38] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut nextWeekIds: [u32; 38] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut dayB4YesterdayIds: [u32; 38] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut yesterdayIds: [u32; 38] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut tomorrowIds: [u32; 38] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut dayAfterTomorrowIds: [u32; 38] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];


pub static mut LOCATION_TIMEZONE_MAP: LsbShiftTree<usize> = LsbShiftTree::new();
pub static mut LOCATIONS_BY_TIMEZONE: Vec<u32> = Vec::new();

// Keeps track of when a timezone is being modified
pub static mut TIMEZONE_MODIFICATION_FLAGS: Vec<boolean> = Vec::with_capacity(38);

pub static mut NEXT_MONTHS_LOCATION_POLLS: Vec<LocationPollPrependLists> = Vec::with_capacity(38);
pub static mut NEXT_WEEKS_LOCATION_POLLS: Vec<u32> = Vec::new();
pub static mut TOMORROWS_LOCATION_POLLS: Vec<u32> = Vec::new();
pub static mut DAY_AFTER_TOMORROWS_LOCATION_POLLS: Vec<u32> = Vec::new();




/**
 *  Random access Category and Location Id maps, needed by initial lookup from clients.  The
 *  stored index is then used to access the VoteCount nested arrays.
 */
pub static mut CATEGORY_LAST_MONTH_INDEX_MAP: IntHashMap<u64, usize> = IntHashMap::with_capacity(2000);
pub static mut CATEGORY_THIS_MONTH_INDEX_MAP: IntHashMap<u64, usize> = IntHashMap::with_capacity(2000);
pub static mut CATEGORY_LAST_WEEK_INDEX_MAP: IntHashMap<u64, usize> = IntHashMap::with_capacity(2000);
pub static mut CATEGORY_THIS_WEEK_INDEX_MAP: IntHashMap<u64, usize> = IntHashMap::with_capacity(2000);
pub static mut CATEGORY_DAY_BEFORE_YESTERDAY_INDEX_MAP: IntHashMap<u64, usize> = IntHashMap::with_capacity(2000);
pub static mut CATEGORY_YESTERDAY_INDEX_MAP: IntHashMap<u64, usize> = IntHashMap::with_capacity(2000);
pub static mut CATEGORY_TODAY_INDEX_MAP: IntHashMap<u64, usize> = IntHashMap::with_capacity(2000);

pub static mut LOCATION_LAST_MONTH_INDEX_MAP: IntHashMap<u64, LocationPeriodIds> = IntHashMap::with_capacity(2000);
pub static mut LOCATION_THIS_MONTH_INDEX_MAP: IntHashMap<u64, LocationPeriodIds> = IntHashMap::with_capacity(2000);
pub static mut LOCATION_LAST_WEEK_INDEX_MAP: IntHashMap<u64, LocationPeriodIds> = IntHashMap::with_capacity(2000);
pub static mut LOCATION_THIS_WEEK_INDEX_MAP: IntHashMap<u64, LocationPeriodIds> = IntHashMap::with_capacity(2000);
pub static mut LOCATION_DAY_BEFORE_YESTERDAY_INDEX_MAP: IntHashMap<u64, LocationPeriodIds> = IntHashMap::with_capacity(2000);
pub static mut LOCATION_YESTERDAY_INDEX_MAP: IntHashMap<u64, LocationPeriodIds> = IntHashMap::with_capacity(2000);
pub static mut LOCATION_TODAY_INDEX_MAP: IntHashMap<u64, LocationPeriodIds> = IntHashMap::with_capacity(2000);

/**
 *  The location/based poll rankings nested arrays by:
 *      Timezone Id
 *          Location Id
 *  Internally each LocationPollRanking contains another array by:
 *      Category Id
 *
 *  Location and Location+Category Ids are initially looked up via the Random Access maps.
 *  Subsequently, the client knows the time period specific ids and uses them for direct access.
 */
pub static mut LAST_MONTH_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>> = Vec::with_capacity(38);
pub static mut THIS_MONTH_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>> = Vec::with_capacity(38);

pub static mut LAST_WEEK_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>> = Vec::with_capacity(38);
pub static mut THIS_WEEK_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>> = Vec::with_capacity(38);

pub static mut DAY_BEFORE_YESTERDAY_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>> = Vec::with_capacity(38);
pub static mut YESTERDAY_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>> = Vec::with_capacity(38);
pub static mut TODAY_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>> = Vec::with_capacity(38);


/**
 * Poll rankings by Category.
 * Q: Global category lookups are meant to cross timezone boundaries but how to maintain that?
 *
 * 1)  Maintain only per-location/per-category rankings
 *
 * 2)  Dynamically add and remove polls from category rankings as the go in and out of scope for each
 * day (probably too hard at the moment).
 *
 * 3)  Maintain only previous period rankings (doable now) - Implementing
 *
 * 3a)  Actually, today's category rankings can be made available after UTC-8 (West Coast) passes
 * its poll add deadline (10pm) for the next day.  At that point there are still 9-10 hours left
 * in the day in Japan (depending on daylight savings).
 */
pub static mut LAST_MONTH_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>> = Vec::new();
pub static mut THIS_MONTH_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>> = Vec::new();

pub static mut LAST_WEEK_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>> = Vec::new();
pub static mut THIS_WEEK_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>> = Vec::new();

pub static mut DAY_BEFORE_YESTERDAY_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>> = Vec::new();
pub static mut YESTERDAY_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>> = Vec::new();
pub static mut TODAY_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>> = Vec::new();


/**
 * Random access current poll maps, needed for count and sum increments by the voting servers.
 *    Indexed by global PollIds
 */
pub static mut TODAY_1_D_POLL_MAP: IntHashMap<u64, OneDPoll> = IntHashMap::with_capacity(2000);
pub static mut TODAY_2_D_POLL_MAP: IntHashMap<u64, TwoDPoll> = IntHashMap::with_capacity(2000);
pub static mut TODAY_3_D_POLL_MAP: IntHashMap<u64, ThreeDPoll> = IntHashMap::with_capacity(2000);
pub static mut THIS_WEEK_1_D_POLL_MAP: IntHashMap<u64, OneDPoll> = IntHashMap::with_capacity(2000);
pub static mut THIS_WEEK_2_D_POLL_MAP: IntHashMap<u64, TwoDPoll> = IntHashMap::with_capacity(2000);
pub static mut THIS_WEEK_3_D_POLL_MAP: IntHashMap<u64, ThreeDPoll> = IntHashMap::with_capacity(2000);
pub static mut THIS_MONTH_1_D_POLL_MAP: IntHashMap<u64, OneDPoll> = IntHashMap::with_capacity(2000);
pub static mut THIS_MONTH_2_D_POLL_MAP: IntHashMap<u64, TwoDPoll> = IntHashMap::with_capacity(2000);
pub static mut THIS_MONTH_3_D_POLL_MAP: IntHashMap<u64, ThreeDPoll> = IntHashMap::with_capacity(2000);

/**
* Polls array by in-cache index, by timezone.
*  The actual poll counts are stored here.  They are accessed by the clients when they need
*  sums and counts for a particular poll.
*/
pub static mut TODAY_1_D_POLLS: Vec<Vec<OneDPoll>> = Vec::with_capacity(38);
pub static mut TODAY_2_D_POLLS: Vec<Vec<TwoDPoll>> = Vec::with_capacity(38);
pub static mut TODAY_3_D_POLLS: Vec<Vec<ThreeDPoll>> = Vec::with_capacity(38);
pub static mut YESTERDAY_1_D_POLLS: Vec<Vec<OneDPoll>> = Vec::with_capacity(38);
pub static mut YESTERDAY_2_D_POLLS: Vec<Vec<TwoDPoll>> = Vec::with_capacity(38);
pub static mut YESTERDAY_3_D_POLLS: Vec<Vec<ThreeDPoll>> = Vec::with_capacity(38);
pub static mut DAY_B4_YESTERDAY_1_D_POLLS: Vec<Vec<OneDPoll>> = Vec::with_capacity(38);
pub static mut DAY_B4_YESTERDAY_2_D_POLLS: Vec<Vec<TwoDPoll>> = Vec::with_capacity(38);
pub static mut DAY_B4_YESTERDAY_3_D_POLLS: Vec<Vec<ThreeDPoll>> = Vec::with_capacity(38);
pub static mut THIS_WEEK_1_D_POLLS: Vec<Vec<OneDPoll>> = Vec::with_capacity(38);
pub static mut THIS_WEEK_2_D_POLLS: Vec<Vec<TwoDPoll>> = Vec::with_capacity(38);
pub static mut THIS_WEEK_3_D_POLLS: Vec<Vec<ThreeDPoll>> = Vec::with_capacity(38);
pub static mut LAST_WEEK_1_D_POLLS: Vec<Vec<OneDPoll>> = Vec::with_capacity(38);
pub static mut LAST_WEEK_2_D_POLLS: Vec<Vec<TwoDPoll>> = Vec::with_capacity(38);
pub static mut LAST_WEEK_3_D_POLLS: Vec<Vec<ThreeDPoll>> = Vec::with_capacity(38);
pub static mut THIS_MONTH_1_D__POLLS: Vec<Vec<OneDPoll>> = Vec::with_capacity(38);
pub static mut THIS_MONTH_2_D_POLLS: Vec<Vec<TwoDPoll>> = Vec::with_capacity(38);
pub static mut THIS_MONTH_3_D_POLLS: Vec<Vec<ThreeDPoll>> = Vec::with_capacity(38);
pub static mut LAST_MONTH_1_D__POLLS: Vec<Vec<OneDPoll>> = Vec::with_capacity(38);
pub static mut LAST_MONTH_2_D_POLLS: Vec<Vec<TwoDPoll>> = Vec::with_capacity(38);
pub static mut LAST_MONTH_3_D_POLLS: Vec<Vec<ThreeDPoll>> = Vec::with_capacity(38);



/**
 * Underlying data structures
 */

/**
 * Random access data structure needed for initial lookup of a Location+Category poll rankings.
 * Contains time period specific array index of the Location
 *      and a map (by Global Id) of the category indexes for same time period
 */
pub struct LocationPeriodIds {
    locationIndex: u32,
    categoryIndexMap: IntHashMap<u64, u32>,
}

impl LocationPeriodIds {
    pub fn new(
        locationIndex: u32,
        numCategories: usize,
    ) -> LocationPeriodIds {
        LocationPeriodIds {
            locationIndex,
            categoryIndexMap: IntHashMap::with_capacity(numCategories),
        }
    }
}


/**
Split by timezone:
*/

/**
 *  Vote count data structure needed for looking up Poll Rankings by Vote Count
 *  Contains ranked vote counts for a particular location
 *      and an array (by time period+location specific category index) of location+category
 *          ranked vote counts
 */
pub struct LocationPollRankings<'a> {
    location: &'a Vec<VoteCount>,
    categoryLocations: &'a Vec<Vec<VoteCount>>,
}

/**
 *  Ordered list of latest added polls (in a future time period).
 *     Contains time ordered polls (in order of creation) for a particular location
 *         and a map/tree (by Global Category Id) of time ordered polls for location+category
 */
pub struct LocationPollPrependLists<'a> {
    location: &'a PrependList<'a>,
    categoryLocations: &'a LsbShiftTree<PrependList<'a>>,
}


/**
 *  We need a dynamically sized data structure for adding polls.  The data structure should be
 *  memory efficient but even more importantly should be computationally efficient.  HashMap
 *  is limited by its need to re-hash.  Hence this is a custom ...
 *
 *  Least Significant digit (bit shift operation based) tree.  It consists of branch and leaf nodes
 *  of variable depth.  It grows by occasionally adding a root node to a (sub)-branch, and otherwise
 *  adding child branches.  It is only fully locked up for read access when the root node is being
 *  replaced due to addition.
 *
 *  The final leafs are arrays of Global Poll Ids (defaulting to 1024)
 *
 *  It is computationally efficient (especially with higher branch counts) because navigation
 *  from a tree node to a tree node is based on bit shifting of the least significant digits
 *  and because of higher branch factors (defaults to 8 branches per node).
 *
 *  It is reasonably memory efficient and is acceptable from that point of view because it is only
 *  used for the future periods.
 */
pub struct LsbShiftTree<T> {}

impl<T> LsbShiftTree<T> {
    pub fn new() -> LsbShiftTree<T> {
        PollTree {}
    }
}


/**
 * Transmission details - for future poll time ordered lists a single header with the number of
 * bytes per id is acceptable.  This is because the global poll ids will have close ids (due to
 * creation order) and can be assumed to take up a roughly equal amount of bits for storage.
 * A page level byte counter can be used to pre-compute it (at insertion time).
 *
 * Note for current/past periods same type counter can be used for per timezone/period, computed
 * at creation of the period.
 */



/*
 * With 64bit Dimension Direction Sums:
 *
 * At least upper 3 bytes in sums will be free, we can use this space for
 * additional threshold counts and flags.  Also the total sum of free
 * bytes will be at least 6 to 18.  This could be used to store additional
 * information about the poll.
 *
 * For example, the positional configuration of a 3D poll can be encoded
 * into a number of configurations.  Lets assume that it would take 2 bytes
 * (64K configurations).  In the
 *
 * With 32 bit sums, they will loose precision after about 300M polls (given
 * that vote could take up to 30 spaces (2*3*5), so may need overflow bytes
 * to keep track of overflow and additional computation is needed:
 *
 * let newVal = oldVal + 24
 * if(newVal < oldVal) {
 *  overflow += 1;
 * }
 *
 * Note for pipe compression having 8u + u32 is actually faster, because
 * only 5 bytes need to be checked and serialized vs 8
 */

/**
 * Count of votes contains:
 *   PollType + Timezone - unified in a byte
 *   Id of the poll for that Timezone+period
 *   count of votes
 *   TODO: revisit poll count size if and when needed (perhaps adding an overflow bit)
 */
pub struct VoteCount {
    /**
    First 5 bits are for timezone, last 3 for for Type
    */
    pollTypeAndTz: u8,
    tzAndPeriodPollId: u32,
    count: u32,
}

/*
 * Poll sums and counts for a 3 dimensional poll.
 */
pub struct ThreeDPoll {
    globalPollId: u64,
    dim1dir1Over: u8,
    dim1dir2Over: u8,
    dim2dir1Over: u8,
    dim2dir2Over: u8,
    dim3dir1Over: u8,
    dim3dir2Over: u8,
    dim1dir1Sum: u32,
    dim1dir2Sum: u32,
    dim2dir1Sum: u32,
    dim2dir2Sum: u32,
    dim3dir1Sum: u32,
    dim3dir2Sum: u32,
    voteCount: VoteCount,
}

/*
 * Poll sums and counts for a 2 dimensional poll.
 */
pub struct TwoDPoll {
    pollId: u64,
    dim1dir1Over: u8,
    dim1dir2Over: u8,
    dim2dir1Over: u8,
    dim2dir2Over: u8,
    dim1dir1Sum: u32,
    dim1dir2Sum: u32,
    dim2dir1Sum: u32,
    dim2dir2Sum: u32,
    voteCount: VoteCount,
}

/*
 * Poll sums and counts for a 1 dimensional poll.
 */
pub struct OneDPoll {
    pollId: u64,
    dim1dir1Over: u8,
    dim1dir2Over: u8,
    dim1dir1Sum: u32,
    dim1dir2Sum: u32,
    voteCount: VoteCount,
}