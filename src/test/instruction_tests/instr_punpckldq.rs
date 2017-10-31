use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpckldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 98, 210], OperandSize::Dword)
}

#[test]
fn punpckldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(MM0)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 98, 2], OperandSize::Dword)
}

#[test]
fn punpckldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 98, 230], OperandSize::Qword)
}

#[test]
fn punpckldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 733537001, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 98, 132, 139, 233, 226, 184, 43], OperandSize::Qword)
}

#[test]
fn punpckldq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 98, 213], OperandSize::Dword)
}

#[test]
fn punpckldq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 473512669, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 98, 164, 185, 221, 58, 57, 28], OperandSize::Dword)
}

#[test]
fn punpckldq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 98, 218], OperandSize::Qword)
}

#[test]
fn punpckldq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 73144731, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 98, 132, 136, 155, 25, 92, 4], OperandSize::Qword)
}

