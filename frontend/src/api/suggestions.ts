import axios from "axios"

const ADD_SUGGESTION_BASE_URL = "http://localhost:9999/api/v1/query"
const GET_SUGGESTION_BASE_URL = "http://localhost:9999/api/v1/suggestions";

export const addSuggestion = async (searchQuery: string) => {
    const resp = await axios.post(`${ADD_SUGGESTION_BASE_URL}/${searchQuery}`)

    if (resp.status !== 200) {
        console.log('Error')
    }
}

interface GetSuggestionsResult {
    message: string
    data: string[]
}

export const getSuggestions = async (searchQuery: string): Promise<string[]> => {
    const resp = await axios.get(`${GET_SUGGESTION_BASE_URL}/${searchQuery}`)

    const response: GetSuggestionsResult = resp.data

    console.log(response.data[0])

    return response.data
}
