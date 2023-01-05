//
//  ContentView.swift
//  auto-typer
//
//  Created by ThetaSinner on 29/12/2022.
//

import SwiftUI
import AppKit
import LibAutoTyper


struct ContentView: View {
    @State var accessibleText = "unknown"
    @State var at = LibAutoTyper.create()
    @State var currentFile = "Select a file"
    @State var stateText = "not started"
    @State var positionText = "not started"
    @State var currentSpeed = "default"
    @State var initialised = false
    
    var body: some View {
        VStack {
            HStack {
                Text(currentFile)
                Button(action: pickFile) {
                    Image(systemName: "folder")
                }.background(.blue).cornerRadius(10)
            }
            HStack {
                Text("Accessibility enabled?")
                Text(accessibleText)
                Button("check", action: checkAccessibility)
            }
            HStack {
                Text("Typing speed:")
                Text(currentSpeed)
                Button("slow", action: inputSpeedSlow)
                Button("medium", action: inputSpeedMedium)
                Button("fast", action: inputSpeedFast)
            }
            Text(stateText).padding()
            Button("start", action: start)
            HStack {
                Text(positionText)
                Button("previous", action: previous)
                Button("skip", action: skip)
            }
            Text("Listening for Cmd+Option+F8")
        }
    }
    
    func checkAccessibility() -> Void {
        let key = kAXTrustedCheckOptionPrompt.takeRetainedValue() as String
        if (AXIsProcessTrustedWithOptions([key: true] as CFDictionary)) {
            accessibleText = "true"
        } else {
            accessibleText = "false"
        }
    }
    
    func listener(event: NSEvent) {
        if (event.modifierFlags.contains([.command, .option]) && event.keyCode == 100) {
            withUnsafeMutablePointer(to: &at) { (ptr: UnsafeMutablePointer<LibAutoTyper.AutoTyper>) -> Void in
                if (LibAutoTyper.has_next(ptr)) {
                    self.stateText = "working"
                    LibAutoTyper.next(ptr)
                    if (!LibAutoTyper.has_next(ptr)) {
                        self.stateText = "done"
                    }
                }
            }
            
            setPositionText()
        }
    }
    
    func start() {
        if (!initialised) {
            NSEvent.addGlobalMonitorForEvents(matching: NSEvent.EventTypeMask.keyDown, handler: listener)
            self.initialised = true
        }
        
        withUnsafeMutablePointer(to: &at) { (ptr: UnsafeMutablePointer<LibAutoTyper.AutoTyper>) -> Void in
            LibAutoTyper.configure(ptr)
            
            self.stateText = "configured"
        }
        
        setPositionText()
    }
    
    func previous() {
        withUnsafeMutablePointer(to: &at) { (ptr: UnsafeMutablePointer<LibAutoTyper.AutoTyper>) -> Void in
            LibAutoTyper.previous(ptr)
        }
        
        setPositionText()
    }
    
    func skip() {
        withUnsafeMutablePointer(to: &at) { (ptr: UnsafeMutablePointer<LibAutoTyper.AutoTyper>) -> Void in
            LibAutoTyper.skip(ptr)
        }
        
        setPositionText()
    }
    
    func setPositionText() {
        positionText = String(format: "next step is %d / %d", at.next_stage + 1, at.stage_count)
    }
    
    func pickFile() {
        let dialog = NSOpenPanel();

        dialog.title                   = "Pick input file";
        dialog.showsResizeIndicator    = true;
        dialog.showsHiddenFiles        = false;
        dialog.allowsMultipleSelection = false;
        dialog.canChooseDirectories    = false;

        if (dialog.runModal() ==  NSApplication.ModalResponse.OK) {
            let result = dialog.url // Pathname of the file

            if (result != nil) {
                let path: String = result!.path
                self.currentFile = path
                
                withUnsafeMutablePointer(to: &at) { (ptr: UnsafeMutablePointer<LibAutoTyper.AutoTyper>) -> Void in
                    let str = strdup(self.currentFile)
                    LibAutoTyper.set_file(ptr, UnsafeMutablePointer(str))
                    LibAutoTyper.print(ptr)
                }
            }
        }
    }
    
    func inputSpeedSlow() {
        withUnsafeMutablePointer(to: &at) { (ptr: UnsafeMutablePointer<LibAutoTyper.AutoTyper>) -> Void in
            self.currentSpeed = "slow"
            LibAutoTyper.set_wpm(ptr, 250.0)
            LibAutoTyper.print(ptr)
        }
    }
    
    func inputSpeedMedium() {
        withUnsafeMutablePointer(to: &at) { (ptr: UnsafeMutablePointer<LibAutoTyper.AutoTyper>) -> Void in
            self.currentSpeed = "medium"
            LibAutoTyper.set_wpm(ptr, 400.0)
            LibAutoTyper.print(ptr)
        }
    }
    
    func inputSpeedFast() {
        withUnsafeMutablePointer(to: &at) { (ptr: UnsafeMutablePointer<LibAutoTyper.AutoTyper>) -> Void in
            self.currentSpeed = "fast"
            LibAutoTyper.set_wpm(ptr, 550.0)
            LibAutoTyper.print(ptr)
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
