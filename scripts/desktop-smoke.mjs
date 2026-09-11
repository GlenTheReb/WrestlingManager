// Real Windows WebView2 + Tauri + SQLite. No mocked IPC or browser save substitute.
import { spawn } from 'node:child_process';
import { mkdir, mkdtemp, access, writeFile } from 'node:fs/promises';
import { createServer } from 'node:net';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { chromium, expect } from '@playwright/test';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const binary = path.join(root, 'target', 'debug', 'wm-desktop.exe');
const artifacts = path.join(root, '.artifacts');
await access(binary);
await mkdir(artifacts, { recursive: true });
const isolated = await mkdtemp(path.join(artifacts, 'desktop-smoke-'));
const savesDirectory = path.join(isolated, 'saves');
const errors = [];
let launchNumber = 0;

async function freePort() {
  const server = createServer();
  await new Promise((resolve) => server.listen(0, '127.0.0.1', resolve));
  const port = server.address().port;
  await new Promise((resolve, reject) =>
    server.close((error) => (error ? reject(error) : resolve())),
  );
  return port;
}

async function launch() {
  const launchId = ++launchNumber;
  const port = await freePort();
  const process = spawn(binary, [], {
    cwd: root,
    windowsHide: true,
    env: {
      ...globalThis.process.env,
      WM_SAVE_DIR: savesDirectory,
      WEBVIEW2_USER_DATA_FOLDER: path.join(isolated, `webview-${launchId}`),
      WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS: `--remote-debugging-port=${port}`,
    },
    stdio: ['ignore', 'pipe', 'pipe'],
  });
  let diagnostic = '';
  let spawnError;
  let connectionError;
  process.on('error', (error) => {
    spawnError = error;
  });
  process.stdout.on('data', (chunk) => {
    diagnostic += chunk.toString();
  });
  process.stderr.on('data', (chunk) => {
    diagnostic += chunk.toString();
  });
  let browser;
  try {
    const deadline = Date.now() + 60_000;
    while (Date.now() < deadline) {
      if (spawnError) throw spawnError;
      if (process.exitCode !== null)
        throw new Error(`Desktop exited: ${diagnostic.slice(-2500)}`);
      try {
        browser = await chromium.connectOverCDP(`http://127.0.0.1:${port}`, {
          timeout: Math.min(5000, deadline - Date.now()),
        });
        break;
      } catch (error) {
        connectionError = error;
        await new Promise((resolve) => setTimeout(resolve, 250));
      }
    }
    if (!browser)
      throw new Error(
        `WebView2 debugger connection failed: ${connectionError?.message}\nDesktop output: ${diagnostic.slice(-2500)}`,
      );
    const context = browser.contexts()[0];
    let page = context.pages()[0];
    if (!page) page = await context.waitForEvent('page');
    page.on('pageerror', (error) => errors.push(error.message));
    await expect(
      page.getByRole('heading', { name: 'Save library', exact: true }),
    ).toBeVisible();
    return { process, browser, page };
  } catch (error) {
    await writeFile(
      path.join(isolated, `launch-${launchId}.log`),
      `${error.stack}\n\nDesktop output:\n${diagnostic}`,
    );
    if (browser) await browser.close();
    process.kill();
    throw error;
  }
}

async function stop(session) {
  if (session.browser.isConnected()) await session.browser.close();
  if (session.process.exitCode === null) {
    session.process.kill();
    await new Promise((resolve) => {
      session.process.once('exit', resolve);
      setTimeout(resolve, 3000).unref();
    });
  }
}

const seed = '18446744073709551615';
const ipc = (page, command, args) =>
  page.evaluate(
    ({ command, args }) => globalThis.__TAURI_INTERNALS__.invoke(command, args),
    { command, args },
  );
let session = await launch();
let partial;
let bookedWinner;
try {
  const { page } = session;
  await page.getByRole('button', { name: '+ New career' }).click();
  await page.getByLabel(/^Save name/).fill('smoke-career');
  await page.getByLabel(/^World seed/).fill(seed);
  await page
    .getByRole('button', { name: 'Create career', exact: true })
    .click();
  await expect(
    page.getByRole('heading', { name: 'Decision centre' }),
  ).toBeVisible();
  await expect(page.getByText(seed, { exact: true })).toBeVisible();
  await expect(
    page.getByRole('button', { name: /WM.*WRESTLING.*MANAGER/ }),
  ).toBeVisible();
  await page.getByRole('button', { name: /^News and inbox/ }).click();
  await page.getByRole('button', { name: /The book is yours at UWF/ }).click();
  await expect(
    page.getByRole('button', { name: 'Mark unread', exact: true }),
  ).toBeVisible();
  await expect(
    page.getByRole('button', { name: 'News and inbox, 0 unread' }),
  ).toBeVisible();
  await page.keyboard.press('F1');
  await page.screenshot({
    path: path.join(artifacts, 'promotion-overview.png'),
  });
  await page.getByRole('button', { name: 'Open talent search' }).click();
  await expect(
    page.getByRole('heading', { name: 'Talent Search' }),
  ).toBeVisible();
  await expect(page.getByText('40 workers', { exact: true })).toBeVisible();
  await page.getByRole('button', { name: 'Open search filters' }).click();
  await expect(page.getByRole('complementary')).toBeVisible();
  await expect(page.getByLabel('Find a filter')).toBeVisible();
  await page.getByRole('button', { name: /Style OVR/ }).click();
  await expect(page.getByText('40 workers', { exact: true })).toBeVisible();
  await page.screenshot({ path: path.join(artifacts, 'worker-finder.png') });
  const first = page.locator('tbody button').first();
  await first.click();
  await expect(page.getByRole('dialog')).toBeVisible();
  await page
    .getByRole('button', { name: 'Person & traits', exact: true })
    .click();
  await expect(
    page.getByRole('heading', { name: 'Shared qualities', exact: true }),
  ).toBeVisible();
  await expect(
    page.getByText('No exceptional-trait evidence recorded yet.'),
  ).toBeVisible();
  await page.screenshot({ path: path.join(artifacts, 'worker-identity.png') });
  await page
    .getByRole('button', { name: 'Relationships', exact: true })
    .click();
  await expect(
    page.getByRole('heading', { name: 'Speak with wrestler', exact: true }),
  ).toBeVisible();
  await page
    .getByRole('button', { name: 'Introduce yourself', exact: true })
    .click();
  await expect(
    page.getByText(/^(Warm|Open|Guarded|Defensive) response$/),
  ).toBeVisible();
  await expect(page.locator('blockquote')).toBeVisible();
  await page.screenshot({
    path: path.join(artifacts, 'worker-relationships.png'),
  });
  await page.getByRole('button', { name: 'Moveset', exact: true }).click();
  await page.screenshot({ path: path.join(artifacts, 'worker-moves.png') });
  await page.getByRole('button', { name: 'Close panel' }).click();
  await page.keyboard.press('F3');
  await expect(page.getByLabel('Wrestler A', { exact: true })).toBeVisible();
  bookedWinner = await page.getByLabel(/^Booked winner/).inputValue();
  await page.getByRole('button', { name: 'Ask agent to plan' }).click();
  await expect(page.getByText('Agent’s working notes')).toBeVisible();
  await page.getByRole('button', { name: /^Edit beat 1 at/ }).click();
  await expect(page.getByLabel('Beat 1 time', { exact: true })).toBeFocused();
  await page
    .getByRole('button', { name: 'Add to running order', exact: true })
    .click();
  await expect(page.getByRole('button', { name: 'Go on air' })).toBeEnabled();
  await page.getByRole('button', { name: '+ Angle', exact: true }).click();
  await page
    .getByRole('button', { name: 'Add to running order', exact: true })
    .click();
  await expect(page.getByText('2 segments', { exact: true })).toBeVisible();
  await page.screenshot({
    path: path.join(artifacts, 'booking-workspace.png'),
  });
  await page.getByRole('button', { name: 'Go on air' }).click();
  await page.getByRole('button', { name: 'Advance 1s' }).click();
  await expect(
    page.getByRole('button', { name: 'Send via agent' }),
  ).toBeEnabled();
  await page.getByRole('button', { name: 'Send via agent' }).click();
  await page.getByLabel('Viewing mode').selectOption('quick');
  await page.getByRole('button', { name: 'Advance 15s' }).click();
  await expect
    .poll(
      async () =>
        (await ipc(page, 'live_show', { saveId: 'smoke-career', showId: 1 }))
          .tick,
    )
    .toBe(16);
  await page.screenshot({ path: path.join(artifacts, 'live-show.png') });
  partial = await ipc(page, 'live_show', { saveId: 'smoke-career', showId: 1 });
  expect(partial.events.some((event) => event.kind === 'instruction')).toBe(
    true,
  );
} finally {
  await stop(session);
}

session = await launch();
try {
  const { page } = session;
  await page.getByRole('button', { name: 'Open career smoke-career' }).click();
  await page.getByRole('button', { name: 'Resume live show' }).click();
  const resumed = await ipc(page, 'live_show', {
    saveId: 'smoke-career',
    showId: 1,
  });
  expect(resumed).toEqual(partial);
  await page.getByLabel('Viewing mode').selectOption('instant');
  await page.getByRole('button', { name: 'Play', exact: true }).click();
  await page.getByRole('button', { name: 'Open post-show report' }).click();
  await expect(page.getByText('Attendance', { exact: true })).toBeVisible();
  const report = await ipc(page, 'show_report', {
    saveId: 'smoke-career',
    showId: 1,
  });
  expect(report.segments).toHaveLength(2);
  expect(report.segments[0].winnerId).toBe(bookedWinner);
  await page.screenshot({ path: path.join(artifacts, 'post-show-report.png') });
  await page.getByRole('button', { name: /^News and inbox/ }).click();
  await page.getByLabel('News category').selectOption('Results');
  await page
    .getByRole('button', { name: /UWF Thursday Night: the night in full/ })
    .click();
  await expect(
    page.getByRole('button', { name: 'Mark unread', exact: true }),
  ).toBeVisible();
  await page.screenshot({ path: path.join(artifacts, 'news-reader.png') });
  await page
    .getByRole('button', { name: 'Open show report', exact: true })
    .click();
  await expect(page.getByText('Attendance', { exact: true })).toBeVisible();
  await page.getByRole('button', { name: /^Continue/ }).click();
  await expect(page.locator('header time')).toHaveText('2 Jan 2026');
  await page.setViewportSize({ width: 1280, height: 720 });
  await page.keyboard.press('F3');
  await expect(page.getByLabel('Wrestler A', { exact: true })).toBeVisible();
  await page.screenshot({ path: path.join(artifacts, 'booking-compact.png') });
  expect(
    await page.evaluate(
      () =>
        globalThis.document.documentElement.scrollWidth <=
        globalThis.innerWidth,
    ),
  ).toBe(true);
  expect(errors).toEqual([]);
  await page.getByRole('button', { name: 'Game menu' }).click();
  await page.getByRole('button', { name: 'Exit game', exact: true }).click();
  await expect.poll(() => session.process.exitCode).toBe(0);
  console.log(
    'PASS: WM identity, relationship interaction, persisted news/read state, news report links, timeline navigation, native exit, and full booking/live/restart/report career workflow.',
  );
  console.log(`Screenshots and isolated test save: ${isolated}`);
} finally {
  await stop(session);
}
