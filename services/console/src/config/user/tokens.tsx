import FieldKind from "../../components/field/kind";
import type { Params } from "../../util/url";
import { validResourceName, validU32 } from "../../util/valid";
import { Button, Card, Display, Operation } from "../types";
import { addPath, parentPath, viewUuidPath } from "../util";
import {authUser} from "../../util/auth.ts";

const user = authUser();

const TOKEN_FIELDS = {
	name: {
		type: "text",
		placeholder: "Token Name",
		icon: "fas fa-stroopwafel",
		help: "Must be a non-empty string",
		validate: validResourceName,
	},
	ttl: {
		type: "number",
		placeholder: "525600",
		icon: "fas fa-stopwatch",
		help: "Must be an integer greater than zero",
		validate: validU32,
	},
};

const tokensConfig = {
	[Operation.LIST]: {
		operation: Operation.LIST,
		header: {
			title: "API Tokens",
			buttons: [
				{
					kind: Button.ADD,
					title: "API Token",
					path: addPath,
				},
				{ kind: Button.REFRESH },
			],
		},
		table: {
			url: (_params: Params) => `/v0/users/${user?.user?.name}/tokens`,
			add: {
				prefix: (
					<div>
						<h4>🐰 Create your first API token!</h4>
						<p>
							It's easy to create an API token.
							<br />
							Tap below to get started.
						</p>
					</div>
				),
				path: addPath,
				text: "Add an API Token",
			},
			row: {
				key: "name",
				items: [{}, {}, {}, {}],
				button: {
					text: "View",
					path: viewUuidPath,
				},
			},
			name: "tokens",
		},
	},
	[Operation.ADD]: {
		operation: Operation.ADD,
		header: {
			title: "Add API Token",
			path: parentPath,
			path_to: "API Tokens",
		},
		form: {
			url: (_params: Params) => `/v0/users/${user?.user?.name}/tokens`,
			fields: [
				{
					kind: FieldKind.INPUT,
					label: "Name",
					key: "name",
					value: "",
					valid: null,
					validate: true,
					config: TOKEN_FIELDS.name,
				},
				{
					kind: FieldKind.NUMBER,
					label: "Time to Live (TTL) (seconds)",
					key: "ttl",
					value: "",
					valid: true,
					validate: true,
					nullable: true,
					config: TOKEN_FIELDS.ttl,
				},
			],
			path: parentPath,
		},
	},
	[Operation.VIEW]: {
		operation: Operation.VIEW,
		header: {
			key: "name",
			path: parentPath,
			path_to: "API Tokens",
			buttons: [{ kind: Button.REFRESH }],
		},
		deck: {
			url: (params: Params) =>
				`/v0/users/${user?.user?.name}/tokens/${params?.token}`,
			cards: [
				{
					kind: Card.FIELD,
					label: "API Token Name",
					key: "name",
					display: Display.RAW,
					is_allowed: (_params: Params) => true,
					field: {
						kind: FieldKind.INPUT,
						label: "Name",
						key: "name",
						value: "",
						valid: null,
						validate: true,
						config: TOKEN_FIELDS.name,
					},
				},
				{
					kind: Card.FIELD,
					label: "API Token UUID",
					key: "uuid",
					display: Display.RAW,
				},
				{
					kind: Card.FIELD,
					label: "API Token",
					key: "token",
					display: Display.RAW,
				},
				{
					kind: Card.FIELD,
					label: "API Token Creation",
					key: "creation",
					display: Display.RAW,
				},
				{
					kind: Card.FIELD,
					label: "API Token Expiration",
					key: "expiration",
					display: Display.RAW,
				},
			],
		},
	},
};

export default tokensConfig;
