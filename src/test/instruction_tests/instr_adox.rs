use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn adox_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 56, 246, 237], OperandSize::Dword)
}

#[test]
fn adox_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(ESP)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 56, 246, 34], OperandSize::Dword)
}

#[test]
fn adox_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 56, 246, 223], OperandSize::Qword)
}

#[test]
fn adox_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 56, 246, 36, 70], OperandSize::Qword)
}

#[test]
fn adox_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(RCX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 56, 246, 207], OperandSize::Qword)
}

#[test]
fn adox_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 56, 246, 28, 200], OperandSize::Qword)
}

