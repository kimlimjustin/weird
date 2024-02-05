import { readFile as _readFile } from 'fs/promises'

export default async function openFile(path) {
  try {
    return await _readFile(path, 'utf8')
  } catch (error) {
    console.error(
      'Error reading file',
      path,
      error
    )
  }
}