/* eslint-disable */

export const AllTypesProps: Record<string,any> = {
	Query:{
		add:{

		},
		add2:{

		}
	}
}

export const ReturnTypes: Record<string,any> = {
	Query:{
		add:"User",
		add2:"User"
	},
	Subscription:{
		count:"User"
	},
	User:{
		name:"String",
		related:"User"
	}
}

export const Ops = {
query: "Query" as const,
	subscription: "Subscription" as const
}