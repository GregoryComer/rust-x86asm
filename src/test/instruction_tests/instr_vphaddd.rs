use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphaddd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 2, 243], OperandSize::Dword)
}

#[test]
fn vphaddd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDI, 924400703, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 2, 191, 63, 60, 25, 55], OperandSize::Dword)
}

#[test]
fn vphaddd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 2, 220], OperandSize::Qword)
}

#[test]
fn vphaddd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 2069346716, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 2, 172, 191, 156, 185, 87, 123], OperandSize::Qword)
}

#[test]
fn vphaddd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 2, 253], OperandSize::Dword)
}

#[test]
fn vphaddd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EDX, 1246032187, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 2, 162, 59, 241, 68, 74], OperandSize::Dword)
}

#[test]
fn vphaddd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 2, 250], OperandSize::Qword)
}

#[test]
fn vphaddd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 2, 54], OperandSize::Qword)
}

