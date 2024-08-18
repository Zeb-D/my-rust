var https = require('https');
var REMOTE_CLOUD_BASE_PATH = '/v1';
var REMOTE_CLOUD_HOSTNAME = process.env.EUFY_HOSTNAME;
var REMOTE_CLOUD_HOSTNAME_AUTH = process.env.EUFY_HOSTNAME_AUTH;
var REMOTE_CLOUD_PORT = process.env.EUFY_PORT;
var ALEXA_REGION = process.env.ALEXA_REGION; // NA , EU, FE , etc.

// Payload Version
const PAYLOAD_VERSION_3 = "3";
const PAYLOAD_VERSION_2 = "2";

// namespaces
const NAMESPACE_CONTROL_V2 = "Alexa.ConnectedHome.Control";
const NAMESPACE_CONTROL_V3 = "Alexa";
const NAMESPACE_DISCOVERY_V2 = "Alexa.ConnectedHome.Discovery";
const NAMESPACE_DISCOVERY_V3 = "Alexa.Discovery";
const NAMESPACE_AUTHORIZE_V3 = "Alexa.Authorization";

// discovery
const RESPONSE_DISCOVER_V2 = "DiscoverAppliancesResponse";
const RESPONSE_DISCOVER_V3 = "Discover.Response";

// Errors
const ERROR_AUTH_ERROR = "INVALID_AUTHORIZATION_CREDENTIAL";
const ERROR_SERVER_ERROR = "INTERNAL_ERROR";

/**
 * Utility functions.
 */
function log(title, msg) {
	console.log('*************** ' + title + ' *************');
	console.log(JSON.stringify(msg));
	console.log('*************** ' + title + ' End*************');
}

var createMessageId = function () {
	var d = new Date().getTime();
	var uuid = 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
		var r = (d + Math.random() * 16) % 16 | 0;
		d = Math.floor(d / 16);
		return (c == 'x' ? r : (r & 0x3 | 0x8)).toString(16);
	});
	return uuid;
}// createMessageId

var createHeader = function (namespace, name, payloadVersion) {
	return {
		"messageId": createMessageId(),
		"namespace": namespace,
		"name": name,
		"payloadVersion": payloadVersion
	};
}// createHeader

var createDirective = function (header, payload) {
	return {
		"header": header,
		"payload": payload
	};
}// createDirective

/**
 * Generate a response message for V3 API
 *
 * @param {array_object} properties
 * @param {Object} payload
 * @returns {Object}
 */
function generateResponseV3(header, payload) {
	return {
		event: {
			header: header,
			payload: payload
		}
	};
}

/**
 * Generate Error response message for V3 API
 *
 * @param {array_object} properties
 * @param {Object} payload
 * @returns {Object}
 */
function generateErrorResponseV3(payload, endpoint, correlationToken) {
	return {
		event: {
			header: {
				namespace: 'Alexa',
				name: 'ErrorResponse',
				payloadVersion: PAYLOAD_VERSION_3,
				messageId: createMessageId(),
				correlationToken: correlationToken
			},
			endpoint: endpoint,
			payload: payload
		}
	};
}

var handleUnexpectedInfo = function (fault) {
	var header = createHeader(NAMESPACE_CONTROL_V2, "UnexpectedInformationReceivedError", PAYLOAD_VERSION_2);
	var payload = {
		"faultingParameter": fault
	};
	return createDirective(header, payload);
}// handleUnexpectedInfo


/**
 * This method is invoked when we receive a "Discovery" message from Alexa Smart Home Skill.
 * We are expected to respond back with a list of appliances that we have discovered for a given
 * customer.
 */
var handleDiscoveryV2 = function (event, context, callback) {

	const options = {
		hostname: REMOTE_CLOUD_HOSTNAME,
		port: REMOTE_CLOUD_PORT,
		path: REMOTE_CLOUD_BASE_PATH + '/voice/alexa/home_skill/discovery',
		headers: {
			accept: '*/*',
			token: event.payload.accessToken.trim()
		}
	};

	https.get(options, (response) => {

		const payload = {};

		response.on('data', (chunk) => {
			const respData = JSON.parse(chunk);
			log('response data body', respData);

			const respCode = respData.res_code;

			if (respCode === 401) {
				// notify Alexa the access token expired
				callback(null, {'header': createHeader(NAMESPACE_CONTROL_V2, "ExpiredAccessTokenError", PAYLOAD_VERSION_2)});
				return;
			}

			if (respData.payload) {
				payload.discoveredAppliances = respData.payload.discoveredAppliances;
			}
		});

		response.on('end', () => {
			const header = createHeader(NAMESPACE_DISCOVERY_V2, RESPONSE_DISCOVER_V2, PAYLOAD_VERSION_2);
			callback(null, createDirective(header, payload));
		});

	}).on('error', (e) => {
		log('Error', e.message);
		callback(null, createDirective(NAMESPACE_DISCOVERY_V2, "DependentServiceUnavailableError"));
	}).end();
}

/**
 * Control events are processed here.
 * This is called when Alexa requests an action (IE turn off appliance).
 */
var handleControlV2 = function (event, context, callback) {

	const options = {
		hostname: REMOTE_CLOUD_HOSTNAME,
		port: REMOTE_CLOUD_PORT,
		path: REMOTE_CLOUD_BASE_PATH + '/voice/alexa/home_skill/control',
		method: 'POST',
		headers: {
			accept: '*/*',
			token: event.payload.accessToken.trim()
		}
	};

	const req = https.request(options, (response) => {
		// set a default response as the unavailable service
		const actionResp = {
			'header': createHeader(NAMESPACE_CONTROL_V2, "DependentServiceUnavailableError", PAYLOAD_VERSION_2)
		};

		if (response.statusCode === 200) {
			response.on('data', (chunk) => {
				const respData = JSON.parse(chunk);
				log('response data body', respData);

				if (respData.res_code === 401) {
					// notify Alexa the access token expired
					callback(null, {'header': createHeader(NAMESPACE_CONTROL_V2, "ExpiredAccessTokenError")});
					return;
				}

				// process the success response data
				if (respData.header) {
					actionResp.header = respData.header
					actionResp.payload = respData.payload
				}
			});
		}

		response.on('end', () => {
			callback(null, actionResp);
		});

	}).on('error', (e) => {
		log('Error', e.message);
		callback(null, handleUnexpectedInfo(e.message));

	});
	req.write(JSON.stringify(event));
	req.end();
}


/**
 * This method is invoked when we receive a "Discovery" message from Alexa Smart Home Skill, For V3 API
 * We are expected to respond back with a list of appliances that we have discovered for a given
 * customer.
 */
var handleDiscoveryV3 = function ({directive}, context, callback) {

	const options = {
		hostname: REMOTE_CLOUD_HOSTNAME_AUTH,
		port: REMOTE_CLOUD_PORT,
		path: REMOTE_CLOUD_BASE_PATH + '/voice/alexa/home_skill/v3/discovery',
		method: 'POST',
		headers: {
			accept: '*/*',
			token: directive.payload.scope.token
		}
	};
	log('Discover HTTP Request', directive);
	let chunks = [];
	const req = https.request(options, (response) => {
		const payload = {};
		response.on('data', (chunk) => {
			chunks.push(chunk);
		});

		response.on('end', () => {
			let data   = Buffer.concat(chunks);
			log('Discover HTTP Response', data);
			const respData = JSON.parse(data);
			const respCode = respData.res_code;
			if (respCode === 401) {
				// notify Alexa the access token expired
				callback(null, generateErrorResponseV3({type: ERROR_AUTH_ERROR}));
				return;
			}
			if (respData.event && respData.event.payload) {
				payload.endpoints = respData.event.payload.endpoints;
			}

			const header = createHeader(NAMESPACE_DISCOVERY_V3, RESPONSE_DISCOVER_V3, PAYLOAD_VERSION_3);
			const alexaResp = generateResponseV3(header, payload);
			callback(null, alexaResp);
		});

	}).on('error', (e) => {
		log('ERROR', e.message);
		callback(null, generateErrorResponseV3(generateErrorResponseV3({type: ERROR_SERVER_ERROR})));
	});
	req.write(JSON.stringify(directive));
	req.end();

};

/**
 * This method is invoked when we receive a "Authorize" message from Alexa Smart Home Skill, For V3 API
 * We are expected to respond back with a list of appliances that we have discovered for a given
 * customer.
 */
var handleAuthorizeV3 = function ({directive}, context, callback) {

	const options = {
		hostname: REMOTE_CLOUD_HOSTNAME_AUTH,
		port: REMOTE_CLOUD_PORT,
		path: REMOTE_CLOUD_BASE_PATH + '/voice/alexa/home_skill/v3/authorize',
		method: 'POST',
		headers: {
			accept: '*/*',
			alexaRegion: ALEXA_REGION,
			token: directive.payload.grantee.token
		}
	};
	log('Authorize HTTP Request', directive);
	let chunks = [];
	const req = https.request(options, (response) => {
		response.on('data', (chunk) => {
			chunks.push(chunk);
		});

		response.on('end', () => {
			let data   = Buffer.concat(chunks);
			const respData = JSON.parse(data);
			log('Authorize HTTP Response', respData);
			const respCode = respData.res_code;
			if (respCode === 401) {
				// notify Alexa the access token expired
				callback(null, generateErrorResponseV3({type: ERROR_AUTH_ERROR}));
				return;
			}
			const actionResp = {
				event: respData.event
			};
			log('Authorize Alexa Response', actionResp);
			callback(null, actionResp);
		});

	}).on('error', (e) => {
		log('ERROR', e.message);
		callback(null, generateErrorResponseV3(generateErrorResponseV3({type: ERROR_SERVER_ERROR})));
	});
	req.write(JSON.stringify(directive));
	req.end();

};

/**
 * Control events are processed here, For V3 API.
 * This is called when Alexa requests an action (IE turn off appliance).
 */
var handleControlV3 = function ({directive}, context, callback) {

	const options = {
		hostname: REMOTE_CLOUD_HOSTNAME,
		port: REMOTE_CLOUD_PORT,
		path: REMOTE_CLOUD_BASE_PATH + '/voice/alexa/home_skill/v3/control',
		method: 'POST',
		headers: {
			accept: '*/*',
			token: directive.endpoint.scope.token
		}
	};

	log('Control HTTP Request', directive);
	let chunks = [];
	const req = https.request(options, (response) => {
		// set a default response as the unavailable service
		const actionResp = {
			//'header': createHeader(NAMESPACE_CONTROL_V2, "DependentServiceUnavailableError", PAYLOAD_VERSION_3)
		};
		response.on('data', (chunk) => {
			chunks.push(chunk);
		});

		response.on('end', () => {
			let data = Buffer.concat(chunks);
			log('Control HTTP Response', data);
			const respData = JSON.parse(data);
			if (respData.res_code === 401) {
				// notify Alexa the access token expired
				callback(null, generateErrorResponseV3({type: ERROR_AUTH_ERROR}));
				return;
			}

			// process the success response data
			if (respData.context) {
				actionResp.context = respData.context;
				actionResp.event = respData.event;
			}
			callback(null, actionResp);
		});

	}).on('error', (e) => {
		log('ERROR', e.message);
		callback(null, generateErrorResponseV3(generateErrorResponseV3({type: ERROR_SERVER_ERROR})));
	});
	req.write(JSON.stringify(directive));
	req.end();
};


/**
 * Get Request API version
 * @param request
 */
function getRequestVersion(request) {
	if (request.directive && request.directive.header && request.directive.header.payloadVersion) {
		return request.directive.header.payloadVersion;
	} else if (request.header && request.header.payloadVersion) {
		return request.header.payloadVersion;
	} else {
		log("ERROR", "Invalid Request Version")
		return "-1";
	}
}

/**
 * Main entry point targeting for SmartHome Skill API v3 & v2 compatible.
 * Incoming events from Alexa Lighting APIs are processed via this method.
 */
exports.handler = function (request, context, callback) {
	console.log('Receive SmartHome Request ', JSON.stringify(request));
	const payloadVersion = getRequestVersion(request);
	if (payloadVersion === PAYLOAD_VERSION_3) {
		log("REQUEST", "Receive v3 directive!");
		const directive = request.directive;
		try {
			switch (directive.header.namespace) {
				case NAMESPACE_DISCOVERY_V3:
					log("DISCOVER", directive.header.namespace);
					handleDiscoveryV3(request, context, callback);
					break;
				case NAMESPACE_AUTHORIZE_V3:
					log("AUTHORIZE", directive.header.namespace);
					handleAuthorizeV3(request, context, callback);
					break;
				default:
					log("CONTROL", directive.header.namespace);
					handleControlV3(request, context, callback);
			}
		} catch (error) {
			log("ERROR", JSON.stringify(error));
		}
	} else {
		const requestedNamespace = request.header.namespace;
		try {
			switch (requestedNamespace) {
				case NAMESPACE_DISCOVERY_V2:
					handleDiscoveryV2(request, context, callback);
					break;

				case NAMESPACE_CONTROL_V2:
					handleControlV2(event, context, callback);
					break;

				default:
					log('ERROR', 'No supported namespace: ' + requestedNamespace);
					callback(null, handleUnexpectedInfo(requestedNamespace));
					break;
			}
		} catch (error) {
			log("ERROR", JSON.stringify(error));
		}
	}
};
