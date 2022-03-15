# Docs.Attach\_Script\_For\_Lotus\_Notes --

    Create Agent "Send to notester"

    Lotus Notes Agent for detaching notes for Notester inclusion


    Sub Initialize
        Dim d As String
        d = "<<>>"
        Dim  ns As New NotesSession()
        Dim doc As NotesDocument
        Set doc = ns.DocumentContext
        Dim rtitem As NotesRichTextItem
        Set rtitem = doc.GetFirstItem( "Body" )
        If doc.hasembedded Then
            If ( rtitem.Type = RICHTEXT ) Then
                Forall o In rtitem.EmbeddedObjects
                    o.ExtractFile(d + "\" + o.name )
                End Forall
            End If
        End If
        
        Open d + "\_embedded.txt" For Output As #1
        
        Write #1, "From: "+doc.from(0)
        'Write #1, "To: "+doc.doc.sendto(0)
        'Write #1, "Cc: "+doc.copyto(0)
        Write #1, "Subject: "+doc.subject(0)
        Write #1, ""
        Write #1, rtitem.text
        'Dim ret As Integer
        ret = Shell("_notester_attach", 1)
    End Sub




    Sub Initialize
        Dim  ns As New NotesSession()
        Dim doc As NotesDocument
        Set doc = ns.DocumentContext
        Dim rtitem As NotesRichTextItem
        Set rtitem = doc.GetFirstItem( "Body" )
        If doc.hasembedded Then
            If ( rtitem.Type = RICHTEXT ) Then
                Forall o In rtitem.EmbeddedObjects
                    o.ExtractFile( "<<>>" + o.name )
                End Forall
            End If
        End If
        
        Open "<<>>" For Output As #1
        
        Write #1, "From: "+doc.from(0)
        'Write #1, "To: "+doc.doc.sendto(0)
        'Write #1, "Cc: "+doc.copyto(0)
        Write #1, "Subject: "+doc.subject(0)
        Write #1, ""
        Write #1, rtitem.text
        'Dim ret As Integer
        ret = Shell("_notester_attach", 1)
    End Sub
