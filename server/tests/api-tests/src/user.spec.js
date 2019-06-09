const chakram = require('chakram');
const expect = require('chai').expect;

describe('User API', () => {
    it('should allow user creation', async () => {
        const requestBody = {
            name: 'BeThere'
        };

        const response = await chakram.post('http://localhost:8080/user', requestBody);
        const responseBody = response.body;

        expect(responseBody).to.have.property('uuid');
        expect(responseBody).to.have.property('name').that.equals(requestBody.name);
    });
});
