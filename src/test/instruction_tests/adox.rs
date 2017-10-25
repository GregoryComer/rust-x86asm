use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn adox_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 56, 246, 201], OperandSize::Dword)
}

fn adox_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 56, 246, 20, 144], OperandSize::Dword)
}

fn adox_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 56, 246, 242], OperandSize::Qword)
}

fn adox_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Eight, 2114788564, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 56, 246, 188, 217, 212, 28, 13, 126], OperandSize::Qword)
}

fn adox_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(RBX)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 56, 246, 220], OperandSize::Qword)
}

fn adox_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ADOX, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Eight, 216445567, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 56, 246, 148, 243, 127, 178, 230, 12], OperandSize::Qword)
}

