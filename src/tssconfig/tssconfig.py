"""
Configure a slideshow
"""

import wx

class MainFrame(wx.Frame):
    """
    Main Frame for the tssconfig program
    """

    BORDER = 25
    """
    Minimum border width around buttons in the MainFrame.
    """

    def __init__(self, *args, **kw):
        # ensure the parent's __init__ is called
        super(MainFrame, self).__init__(*args, **kw)

        # create a panel in the frame
        pnl = wx.Panel(self)

        # put buttons into buttonSizer
        buttonSizer = self.makeButtonSizer(pnl)

        pnlSizer = wx.BoxSizer()
        pnlSizer.Add(buttonSizer, 1)
        pnl.SetSizer(pnlSizer, 0)

        # create a menu bar
        self.makeMenuBar()

        # and a status bar
        self.CreateStatusBar()
        self.SetStatusText("Welcome to tssconfig")

    def makeButtonSizer(self, parent) ->wx.BoxSizer:
        """This method creates the sizer coontaining the buttons on the MainFrame window."""
        # put buttons into frame
        sizer = wx.BoxSizer(wx.VERTICAL)
        start = self.makeStartButton(parent)
        quit = self.makeQuitButton(parent)

        sizer.Add(start, 1, wx.TOP | wx.ALIGN_CENTER_HORIZONTAL, self.BORDER)
        sizer.Add(quit, 1, wx.TOP | wx.ALIGN_CENTER_HORIZONTAL, self.BORDER)
        return sizer

    def makeStartButton(self, parent) -> wx.Button:
        """
        This method creates the button to display the Load Defaults dialog.
        """
        button = wx.Button(parent, label = "Start Configuration")
        button.Bind(wx.EVT_BUTTON, self.onStartClicked)
        return button

    def onStartClicked(self, _):
        """This method handles clicks on the Start Configuration button."""
        print("In onStartClicked")

    def makeQuitButton(self, parent) -> wx.Button:
        """This method creates a Quit button for display on the MainFrame"""
        button = wx.Button(parent, label = "Quit")
        button.Bind(wx.EVT_BUTTON, self.OnExit)
        return button

    def makeMenuBar(self):
        """
        A menu bar is composed of menus, which are composed of menu items.
        This method builds a set of menus and binds handlers to be called
        when the menu item is selected.
        """

        # Make a file menu with Hello and Exit items
        fileMenu = wx.Menu()
        # The "\t..." syntax defines an accelerator key that also triggers
        # the same event
        exitItem = fileMenu.Append(wx.ID_EXIT)

        # Now a help menu for the about item
        helpMenu = wx.Menu()
        aboutItem = helpMenu.Append(wx.ID_ABOUT)

        # Make the menu bar and add the two menus to it. The '&' defines
        # that the next letter is the "mnemonic" for the menu item. On the
        # platforms that support it those letters are underlined and can be
        # triggered from the keyboard.
        menuBar = wx.MenuBar()
        menuBar.Append(fileMenu, "&File")
        menuBar.Append(helpMenu, "&Help")

        # Give the menu bar to the frame
        self.SetMenuBar(menuBar)

        # Finally, associate a handler function with the EVT_MENU event for
        # each of the menu items. That means that when that menu item is
        # activated then the associated handler function will be called.
        self.Bind(wx.EVT_MENU, self.OnExit,  exitItem)
        self.Bind(wx.EVT_MENU, self.OnAbout, aboutItem)

    def OnExit(self, event):
        """Close the frame, terminating the application."""
        self.Close(True)

    def OnAbout(self, event):
        """Display an About Dialog"""
        wx.MessageBox("This is the trilliumslideshow configuration program",
                      "About tssconfig",
                      wx.OK|wx.ICON_ERROR)


if __name__ == '__main__':
    # When this module is run (not imported) then create the app, the
    # frame, show it, and start the event loop.
    app = wx.App()
    frm = MainFrame(None, title='Slideshow Configurator')
#    frm.SetSize(-1, -1, 300, 200)
#    frm.Fit()
    frm.Show()
    app.MainLoop()
