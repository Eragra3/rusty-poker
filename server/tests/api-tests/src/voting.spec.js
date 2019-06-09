const chakram = require('chakram');
const expect = require('chai').expect;


describe('Voting API', () => {

    let host_uuid;
    let voter_uuid;

    beforeEach(async () => {
        let response = await chakram.post('http://localhost:8080/user', { 'name': 'Host user' });
        host_uuid = response.body.uuid;

        response = await chakram.post('http://localhost:8080/user', { 'name': 'Voter 1' });
        voter_uuid = response.body.uuid;
    });

    it('should allow voting creation', async () => {
        const requestBody = {
            host_uuid: host_uuid,
            title: 'Add new products service'
        };

        const response = await chakram.post('http://localhost:8080/voting', requestBody);
        expect(response.response.statusCode).to.equal(200);

        const responseBody = response.body;

        expect(responseBody).to.have.property('start_datetime');
        expect(responseBody).to.have.property('state').that.equals('Waiting');
        expect(responseBody).to.have.property('title').that.equals(requestBody.title);
        expect(responseBody).to.have.property('uuid').that.is.not.empty;
        expect(responseBody).to.have.property('votes').that.is.an('array').that.is.empty;
    });

    it('should allow participant to join', async () => {
        const requestBody = {
            host_uuid: host_uuid,
            title: 'Add new products service'
        };
        
        let response = await chakram.post('http://localhost:8080/voting', requestBody);
        expect(response.response.statusCode).to.equal(200);

        const voting = response.body;
        expect(voting).to.have.property('uuid').that.is.not.empty;

        const voting_uuid = response.body.uuid;

        const participantRequestBody = {
            user_uuid: voter_uuid
        };

        response = await chakram.post(`http://localhost:8080/voting/${voting_uuid}/join`, participantRequestBody);
        expect(response.response.statusCode).to.equal(200);
        const vote = response.body;
        
        expect(vote).to.have.property('value').that.equals('Pending');
    });
});
