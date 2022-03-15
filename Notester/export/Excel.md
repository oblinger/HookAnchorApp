# Excel --




    Private Sub Worksheet_SelectionChange(ByVal Target As Range)
       Dim strCell As String
      
       strCell = Target.Address
       Sheets("Sheet2").Range("A1").Formula = "=Sheet1!" & strCell
    End Sub


    Indirect( )
    Lookup()


    ActiveCell.FormulaR1C1 = "23"


    'MsgBox ("hit it" & Target.Address)
