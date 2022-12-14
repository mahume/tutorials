import Vuex from 'vuex'

const createStore = () => {
  return new Vuex.Store({
    state: {
      loadedPosts: []
    },
    mutations: {
      setPosts(state, posts) {
        state.loadedPosts = posts
      }
    },
    actions: {
      nuxtServerInit(vuexContext, context) {
          return new Promise((resolve, reject) => {
            setTimeout(() => {
                vuexContext.commit('setPosts', [
                  {id: '1', title: 'First Post', previewText: 'This is our first post!', thumbnail: 'https://www.computersciencedegreehub.com/wp-content/uploads/2016/02/what-is-coding-1024x683.jpg'},
                  {id: '2', title: 'Second Post', previewText: 'This is our second post!', thumbnail: 'https://www.computersciencedegreehub.com/wp-content/uploads/2016/02/what-is-coding-1024x683.jpg'},
                  {id: '3', title: 'Third Post', previewText: 'This is our third post!', thumbnail: 'https://www.computersciencedegreehub.com/wp-content/uploads/2016/02/what-is-coding-1024x683.jpg'},
                ])
                resolve();
            }, 1500)
          })
      },
      setPosts(vuexContext, posts) {
        vuexContext.commit('setPosts', posts)
      }
    },
    getters: {
      loadedPosts(state) {
        return state.loadedPosts
      }
    }
  })
}

export default createStore;
