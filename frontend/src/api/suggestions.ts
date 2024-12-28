import axios from "axios"

const ADD_SUGGESTION_BASE_URL = "http://localhost:9999/api/v2/query"
const GET_SUGGESTION_BASE_URL = "http://localhost:9999/api/v2/suggestions";

export const addSuggestion = async (searchQuery: string) => {
    const resp = await axios.post(`${ADD_SUGGESTION_BASE_URL}/${searchQuery}`)

    if (resp.status !== 200) {
        console.log('Error')
    }
}

interface GetSuggestionsResult {
    message: string
    data: SuggestionsData
}

interface SuggestionsData {
    time_taken: number
    total_results: number
    suggestions: string[]
}

export const getSuggestions = async (searchQuery: string): Promise<GetSuggestionsResult> => {
    const resp = await axios.get(`${GET_SUGGESTION_BASE_URL}/${searchQuery}`)

    const response: GetSuggestionsResult = resp.data

    return response
}
