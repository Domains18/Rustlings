import { ApolloClient, HttpLink, InMemoryCache } from '@apollo/client'
import { API_URL } from './urls';

const httpLink = new HttpLink({
    uri: `${API_URL}/graphql`
});
const client = new ApolloClient({
    cache: new InMemoryCache(),
    link: httpLink
});
