use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpckhbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 104, 215], OperandSize::Dword)
}

#[test]
fn punpckhbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(MM3)), operand2: Some(IndirectDisplaced(EAX, 361435978, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 104, 152, 74, 19, 139, 21], OperandSize::Dword)
}

#[test]
fn punpckhbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 104, 205], OperandSize::Qword)
}

#[test]
fn punpckhbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 104, 4, 128], OperandSize::Qword)
}

#[test]
fn punpckhbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 104, 233], OperandSize::Dword)
}

#[test]
fn punpckhbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(ECX, 1398902523, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 104, 185, 251, 142, 97, 83], OperandSize::Dword)
}

#[test]
fn punpckhbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 104, 244], OperandSize::Qword)
}

#[test]
fn punpckhbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 104, 25], OperandSize::Qword)
}

