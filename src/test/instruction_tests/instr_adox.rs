use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn adox_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 56, 246, 202], OperandSize::Dword)
}

#[test]
fn adox_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 309887522, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 56, 246, 164, 144, 34, 130, 120, 18], OperandSize::Dword)
}

#[test]
fn adox_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 56, 246, 202], OperandSize::Qword)
}

#[test]
fn adox_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(RAX, 1193435, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 56, 246, 176, 219, 53, 18, 0], OperandSize::Qword)
}

#[test]
fn adox_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(RDI)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 56, 246, 252], OperandSize::Qword)
}

#[test]
fn adox_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 56, 246, 28, 75], OperandSize::Qword)
}

