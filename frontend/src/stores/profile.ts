import { defineStore } from "pinia";
import { reactive } from "vue";

export interface TeamInfo {
    id: number,
    name: string,
    description: string,
}

export interface ProfileState {
    current_team: undefined | TeamInfo
}

export const useProfileStore = defineStore('profile-store', () => {
    const profile = reactive<ProfileState>({
        current_team: undefined
    });

    const setCurrentTeam = (currentTeamInfo: any) => {
        profile.current_team = currentTeamInfo
    }

    const getCurrentTeam = (): undefined | TeamInfo => {
        return profile.current_team
    }

    return {
        profile, setCurrentTeam, getCurrentTeam
    }
})