import test from 'ava'

import { readEpub } from '../index.js'

test('read epub', async (t) => {
  const epub = await readEpub('./fixtures/basic-v3plus2.epub')
  t.is(epub.metadata.title, 'Your title here')
})
