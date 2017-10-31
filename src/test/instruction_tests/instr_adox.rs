use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn adox_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 56, 246, 246], OperandSize::Dword)
}

#[test]
fn adox_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 699774947, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 56, 246, 36, 197, 227, 183, 181, 41], OperandSize::Dword)
}

#[test]
fn adox_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 56, 246, 210], OperandSize::Qword)
}

#[test]
fn adox_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(EBX)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 56, 246, 26], OperandSize::Qword)
}

#[test]
fn adox_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(RSP)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 56, 246, 230], OperandSize::Qword)
}

#[test]
fn adox_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1007148706, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 56, 246, 36, 205, 162, 222, 7, 60], OperandSize::Qword)
}

