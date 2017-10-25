use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn adox_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 56, 246, 226], OperandSize::Dword)
}

#[test]
fn adox_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(ESI, 645649078, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 56, 246, 174, 182, 210, 123, 38], OperandSize::Dword)
}

#[test]
fn adox_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 56, 246, 218], OperandSize::Qword)
}

#[test]
fn adox_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(ESI)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 56, 246, 55], OperandSize::Qword)
}

#[test]
fn adox_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(RBX)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 56, 246, 222], OperandSize::Qword)
}

#[test]
fn adox_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(RSP)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 56, 246, 35], OperandSize::Qword)
}

