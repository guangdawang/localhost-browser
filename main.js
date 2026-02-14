const { app, BrowserWindow, session, Menu } = require('electron')
const path = require('path')

// 常用本地服务列表（也用于菜单）
const LOCAL_SITES = [
  { label: 'React / Vite (3000)', url: 'http://localhost:3000' },
  { label: 'Vue / Vite (5173)', url: 'http://localhost:5173' },
  { label: 'Angular (4200)', url: 'http://localhost:4200' },
  { label: 'Java Spring Boot (8080)', url: 'http://localhost:8080' },
  { label: 'Python HTTP (8000)', url: 'http://localhost:8000' },
  { label: 'Flask (5000)', url: 'http://localhost:5000' },
  { label: 'Live Server (5500)', url: 'http://127.0.0.1:5500' },
  { label: 'PHP (8081)', url: 'http://localhost:8081' },
  { label: 'React Native (8081)', url: 'http://localhost:8081' }
]

function createWindow() {
  const win = new BrowserWindow({
    width: 1400,
    height: 900,
    title: 'Local Dev Browser',
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
      nodeIntegration: false,
      contextIsolation: true,
      devTools: true
    },
    icon: path.join(__dirname, 'icon.png')
  })

  // 自定义菜单
  const menu = Menu.buildFromTemplate([
    {
      label: '本地服务',
      submenu: LOCAL_SITES.map(site => ({
        label: site.label,
        click: () => win.loadURL(site.url)
      }))
    },
    {
      label: '快捷操作',
      submenu: [
        {
          label: '刷新',
          accelerator: 'CmdOrCtrl+R',
          click: () => win.reload()
        },
        {
          label: '强制刷新',
          accelerator: 'CmdOrCtrl+Shift+R',
          click: () => win.webContents.reloadIgnoringCache()
        },
        {
          label: '开发者工具',
          accelerator: 'CmdOrCtrl+Shift+I',
          click: () => win.webContents.openDevTools()
        },
        { type: 'separator' },
        {
          label: '清除缓存',
          click: async () => {
            await session.defaultSession.clearCache()
            await session.defaultSession.clearStorageData()
            win.reload()
          }
        },
        {
          label: '首页',
          click: () => win.loadFile('dashboard.html')
        }
      ]
    }
  ])
  Menu.setApplicationMenu(menu)

  // 加载仪表盘
  win.loadFile('dashboard.html')
}

app.whenReady().then(() => {
  createWindow()

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) createWindow()
  })
})

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') app.quit()
})
