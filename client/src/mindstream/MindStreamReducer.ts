import { Feed } from "../feeds/Feed"
import { MindStreamAction } from "./MindStreamActions"

export interface MindStreamState {
    previusFeeds: Feed[],
    feeds: Feed[],
    loading: boolean
    error?: any
}

const initState: MindStreamState = {
    previusFeeds: [],
    feeds: [],
    loading: false,
    error: undefined,
}

const MindStreamReducer = (state: MindStreamState = initState, action: MindStreamAction) => {
    switch (action.type) {
        case "LOAD_UNREADED_FEEDS": return { ...state, loading: true }
        case "LOAD_UNREADED_FEEDS_SUCCESS": return { ...state, feeds: action.feeds, loading: false }
        case "LOAD_UNREADED_FEEDS_ERROR": return { ...state, loading: false, error: action.error }
        case "READ_FEED": return { ...state, loading: true }
        case "READ_FEED_ERROR": return { ...state, loading: false, error: action.error }
        case "LOAD_UNREADED_FEEDS_BY_SOURCE": return { ...state, loading: true }
        case "LOAD_UNREADED_FEEDS_BY_SOURCE_SUCCESS": return { ...state, feeds: action.feeds, loading: false }
        case "LOAD_UNREADED_FEEDS_BY_SOURCE_ERROR": return { ...state, loading: false, error: action.error }

        case "NEXT_FEED": {
            const [first, ...rest] = state.feeds
            return { ...state, feeds: rest, previusFeeds: [first, ...state.previusFeeds] }
        }

        case "PREVIOUS_FEED": {
            const [first, ...rest] = state.previusFeeds
            if (first) {
                return { ...state, feeds: [first, ...state.feeds], previusFeeds: rest }
            } else {
                return state
            }
        }

        default: return state
    }
}

export default MindStreamReducer
