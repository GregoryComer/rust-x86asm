use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn aesdeclast_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDECLAST, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 223, 223], OperandSize::Dword)
}

#[test]
fn aesdeclast_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDECLAST, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 304470592, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 223, 172, 142, 64, 218, 37, 18], OperandSize::Dword)
}

#[test]
fn aesdeclast_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDECLAST, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 223, 255], OperandSize::Qword)
}

#[test]
fn aesdeclast_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDECLAST, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 452390556, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 223, 172, 199, 156, 238, 246, 26], OperandSize::Qword)
}

