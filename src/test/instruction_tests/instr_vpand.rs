use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpand_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 219, 221], OperandSize::Dword)
}

#[test]
fn vpand_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EBX, 739025149, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 219, 171, 253, 160, 12, 44], OperandSize::Dword)
}

#[test]
fn vpand_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 219, 214], OperandSize::Qword)
}

#[test]
fn vpand_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 219, 22], OperandSize::Qword)
}

#[test]
fn vpand_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 219, 216], OperandSize::Dword)
}

#[test]
fn vpand_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 219, 36, 122], OperandSize::Dword)
}

#[test]
fn vpand_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 219, 208], OperandSize::Qword)
}

#[test]
fn vpand_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 2055681125, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 219, 36, 149, 101, 52, 135, 122], OperandSize::Qword)
}

