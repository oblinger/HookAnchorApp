# Docs.NotesCode --

    Sub Initialize
        Dim  ns As New NotesSession()
        Dim doc As NotesDocument
        Set doc = ns.DocumentContext
        Dim rtitem As NotesRichTextItem
        Set rtitem = doc.GetFirstItem( "Body" )
        If doc.hasembedded Then
            If ( rtitem.Type = RICHTEXT ) Then
                Forall o In rtitem.EmbeddedObjects
                    'Messagebox( o.Name )
                    o.ExtractFile( "<<>>" + o.name )
                End Forall
            End If
        End If
        
        Open "<<>>" For Output As #1
        
        'Messagebox(doc.subject(0))
        Write #1, doc.subject(0)
        'Messagebox(doc.sendto(0))
        'Messagebox(doc.from(0))
        'Messagebox(rtitem.text)
        Write #1, rtitem.text
        Dim ret As Integer
        ret = Shell("notepad <<>>", 1)
    End Sub
