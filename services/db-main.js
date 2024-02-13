import { default as _postgres } from 'postgres'

import env from '../consts/env.js'

const {
  POSTGRES_HOST,
  POSTGRES_PORT,
  POSTGRES_USERNAME,
  POSTGRES_PASSWORD,
  POSTGRES_DATABASE
} = env

const sql = _postgres({
  host: POSTGRES_HOST,
  username: POSTGRES_USERNAME,
  password: POSTGRES_PASSWORD,
  database: POSTGRES_DATABASE,
  port: POSTGRES_PORT
})

export async function db(query, ...values) {
  try {
    const results = await sql(query, ...values)
    return {
      code: 0,
      message: 'Operation successful',
      data: results
    }
  } catch (error) {
    return {
      code: 1,
      message: error.message,
      data: null
    }
  }
}

// usage:
// import { db } from './services/db.js'
//
// const results = await db`
//   SELECT * FROM users WHERE id = ${userId}
// `
//
// if (results.code === 0) {
//   console.log(results.data)
// } else {
//   console.error(results.message)
// }
